use std::path::PathBuf;
use mixin_sdk::keystore::KeyStore;
use mixin_sdk::Client;

use clap::Parser;

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
    // command: Option<Commands>,
    subcommand: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    User(UserCommand),
    Http(HttpCommand),
}

#[derive(Parser, Debug)]
// #[command(subcommand)]
struct UserCommand {
    // Create(UserCreateCommand),
    // Me(UserMeCommand),
    // Search(UserSearchCommand),
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