use std::path::PathBuf;
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
    file: Option<PathBuf>,

    #[command(subcommand)]
    commond: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    User(UserCommand),
    Http(HttpCommand),
}

#[derive(Debug, Args)]
struct UserCommand {
    #[command(subcommand)]
    command: Option<UserCommands>,
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

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.file.as_deref() {
        println!("Value for file: {}", config_path.display());
        let ks = KeyStore::from_file(config_path);
        println!("{:?}", ks);
        let client = Client::new(ks);
        println!("{:?}", client);
    }



    // // You can check for the existence of subcommands, and if found use their
    // // matches just as you would the top level cmd
    // match &cli.command {
    //     Some(Commands::Test { list }) => {
    //         if *list {
    //             println!("Printing testing lists...");
    //         } else {
    //             println!("Not printing testing lists...");
    //         }
    //     }
    //     None => {}
    // }

    // // Continued program logic goes here...
}