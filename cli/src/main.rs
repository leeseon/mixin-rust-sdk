use std::{path::{PathBuf, Path}, ffi::OsStr, mem};
use mixin_sdk::{keystore::KeyStore, MixinHttpError};
use mixin_sdk::Client;
use reqwest::{Method, Url};
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "mixin-cli")]
#[command(author = "Sean Lee <leeseon@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Mixin Network API command line tool", long_about = None)]
struct Cli {
    /// Sets a custom keystore file
    #[arg(short, long, value_name = "FILE")]
    #[arg(default_value = "~/.mixin-cli/keystore.json")]
    file: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    User(UserCommand),
    Http(HttpCommand),
}

#[derive(Debug, Args)]
/// manager users
struct UserCommand {
    #[command(subcommand)]
    command: UserCommands,
}

#[derive(Debug, Subcommand)]
enum UserCommands {
    /// create new user
    Create{},
    Me{},
    /// search user by uuid or mixin id
    Search{ uuid: String},
}

#[derive(Parser, Debug)]
/// mixin api http client
struct HttpCommand {
    #[arg(short='v', long)]
    /// dump http request
    dump: bool,

    #[arg(long)]
    /// raw json body
    raw: Option<String>,

    #[clap(value_name = "[METHOD] URL")]
    raw_method_or_url: String,

    #[clap(value_name = "REQUEST_ITEM", verbatim_doc_comment)]
    raw_rest_args: Vec<String>,

    /// The HTTP method, if supplied.
    #[clap(skip)]
    pub method: Option<Method>,    

    /// The request URL.
    #[clap(skip = ("http://placeholder".parse::<Url>().unwrap()))]
    pub url: Url,
    
    // /// Optional key-value pairs to be included in the request.
    // #[clap(skip)]
    // pub request_items: RequestItems,    
}

fn abspath_buf(p: &str) -> Option<PathBuf> {
    shellexpand::full(p)
        .ok()
        .and_then(|x| Path::new(OsStr::new(x.as_ref())).canonicalize().ok())
}

fn main() {
    let cli = Cli::parse();

    let mut expanded_path: PathBuf = PathBuf::new();
    if let Some(ref config_path) = cli.file {
        let p = config_path.as_path().display().to_string();
        expanded_path = abspath_buf(&p).unwrap();
    }

    let ks = KeyStore::from_file(expanded_path.as_path());

    let client = Client::new(ks);

    match cli.command {
        Commands::User(user) => {

            match user.command {
                UserCommands::Create{}  => { 
                    println!("create");
                }
                UserCommands::Me{} => {
                    let me = client.me();
                    match me {
                        Ok(j) => println!("{}", serde_json::to_string_pretty(&j).unwrap()),
                        Err(e) =>  {
                            if let Some(mixin_error) = e.downcast_ref::<MixinHttpError>() {
                                println!("{:?}", serde_json::to_string_pretty(&mixin_error));
                            }
                        }
                    }
                }
                UserCommands::Search { uuid } => {
                    println!("Search {:?}", uuid);
                }
            }
        }
        Commands::Http(mut http) => {

            if let Some(ref raw) = http.raw {
                println!("raw {:?}", raw);
            }

            match http.dump {
                true => println!("dump"),
                false => println!("no dump"),
            }

            println!("raw_rest_args {:?}", http.raw_rest_args);

            let mut rest_args = mem::take(&mut http.raw_rest_args).into_iter();
            let raw_url = match parse_method(&http.raw_method_or_url) {
                Some(method) => {
                    http.method = Some(method);
                    rest_args
                        .next()
                        // .ok_or_else(|| Error(ErrorKind::MissingRequiredArgument, "Missing <URL>"))? // .ok_or_else())?
                }
                None => Some({
                    http.method = None;
                    mem::take(&mut http.raw_method_or_url)
                })
            };
            println!("{:?}", http.method.unwrap());
            println!("{:?}", raw_url.unwrap());
        }
    }


}

fn parse_method(method: &str) -> Option<Method> {
    // This unfortunately matches "localhost"
    if !method.is_empty() && method.chars().all(|c| c.is_ascii_alphabetic()) {
        // Method parsing seems to fail if the length is 0 or if there's a null byte
        // Our checks rule those both out, so .unwrap() is safe
        Some(method.to_ascii_uppercase().parse().unwrap())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use clap::{error::Error};

    use super::*;

    fn parse<I>(args: I) -> Result<Cli, Error>
    where
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        Cli::try_parse_from(
            Some("xh".into())
                .into_iter()
                .chain(args.into_iter().map(Into::into)),
        )
    }    

    #[test]
    fn implicit_methods() {
        let cli = parse(["http", "/logs"]).unwrap();
        if let Commands::Http(ref http) = cli.command {
            assert_eq!(http.method, None);
        }
    }
}