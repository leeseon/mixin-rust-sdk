use reqwest::{blocking::Response, header, Method};
use serde::{Deserialize, Serialize};
use std::{error, fmt, time::Duration};
use uuid::Uuid;

use crate::{authorization, keystore};

pub fn request<T: Serialize + ?Sized>(
    ks: keystore::KeyStore,
    method: Method,
    path: &str,
    json: &T,
) -> Result<Response, Box<dyn error::Error>> {
    let mut body = String::from("");
    if method == "POST" {
        let j = serde_json::to_string(&json)?;
        body = j.clone();
    }
    let token = authorization::sign_token(method.clone(), path, &body, ks)?;

    Ok(request_with_token(method, path, json, &token))
}

pub fn request_with_token<T: Serialize + ?Sized>(
    method: Method,
    path: &str,
    json: &T,
    token: &str,
) -> Response {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Request-Id", Uuid::new_v4().to_string().parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Client::build()");

    let uri = format!("https://mixin-api.zeromesh.net{}", path).to_string();
    if method == Method::GET {
        return client.request(method, uri).send().unwrap();
    }

    client.request(method, uri).json(json).send().unwrap()
}