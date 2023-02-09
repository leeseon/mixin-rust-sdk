use std::{path::{PathBuf, Path}, ffi::OsStr, fmt::DebugMap};
use mixin_sdk::{keystore::KeyStore, MixinHttpError};
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
        println!("{:?}", p);
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
        Commands::Http(http) => {

            if let Some(ref raw) = http.raw {
                println!("raw {:?}", raw);
            }

            match http.dump {
                true => println!("dump"),
                false => println!("no dump"),
            }

        }
    }


}