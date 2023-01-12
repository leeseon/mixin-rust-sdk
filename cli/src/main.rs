use std::{path::{PathBuf, Path}, ops::Deref, ffi::OsStr};
use mixin_sdk::keystore::KeyStore;
use mixin_sdk::Client;

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
struct UserCommand {
    #[command(subcommand)]
    command: UserCommands,
}

#[derive(Debug, Subcommand)]
enum UserCommands {
    Create{},
    Me{},
    Search{ uuid: String},
}

#[derive(Parser, Debug)]
struct HttpCommand {

}

// fn abspath(p: &str) -> Option<String> {
//     shellexpand::full(p)
//         .ok()
//         .and_then(|x| Path::new(OsStr::new(x.as_ref())).canonicalize().ok())
//         .and_then(|p| p.into_os_string().into_string().ok())
// }

fn abspath_buf(p: &str) -> Option<PathBuf> {
    shellexpand::full(p)
        .ok()
        .and_then(|x| Path::new(OsStr::new(x.as_ref())).canonicalize().ok())
}

fn main() {
    let cli = Cli::parse();

    let mut expanded_path: PathBuf = PathBuf::new();
    if cli.file.is_none() {
        println!("None");
    }

    if let Some(ref config_path) = cli.file {
        let p = config_path.as_path().display().to_string();
        println!("{:?}", p);
        expanded_path = abspath_buf(&p).unwrap();
    }

    println!("{:?}", expanded_path.as_path().display());


    let ks = KeyStore::from_file(expanded_path.as_path());
    println!("{:?}", ks);

    let client = Client::new(ks);
    println!("{:?}", client);

    match cli.command {
        Commands::User(user) => {

            match user.command {
                UserCommands::Create{}  => { 
                    println!("create");
                }
                UserCommands::Me{} => {
                    let me = client.me();
                    println!("me {:?}", me);
                }
                UserCommands::Search { uuid } => {
                    println!("Search {:?}", uuid);
                }
            }
        }
        Commands::Http(http) => {

        }
    }


}