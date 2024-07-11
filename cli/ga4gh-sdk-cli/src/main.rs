use clap::{arg, value_parser, Command};
use clap_complete::{generate, Generator, Shell};
use std::{error::Error, fs, io, path::Path};

// https://github.com/clap-rs/clap/blob/master/examples/git.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("nanopub")
        .bin_name("np")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Sign, publish, and check Nanopublications.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("sign")
                .about("Sign a Nanopub")
                .arg(arg!(<NANOPUB_FILE> "The file to sign"))
                .arg(
                    arg!(-k --key <PRIVATE_KEY> "The path to a private key used to sign. Found in ~/.nanopub by default")
                        .default_value("")
                )
                .arg(
                    arg!(-p --profile <PROFILE> "The path to a profile.yml file. Default: ~/.nanopub/profile.yml")
                        .default_value("")
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("publish")
                .about("Sign, publish, or check a Nanopublication (https://nanopub.net)")
                .arg(arg!(<NANOPUB_FILE> "The file to publish"))
                .arg(
                    arg!(-k --key <PRIVATE_KEY> "The path to a private key used to sign.")
                        .default_value("")
                )
                .arg(
                    arg!(-p --profile <PROFILE> "The path to a profile.yml file. Default: ~/.nanopub/profile.yml")
                        .default_value("")
                )
                .arg(
                    arg!(-t --test "To publish to the test server instead of the Nanopublication network.")
                )
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("check")
                .about("Check if a Nanopub is valid")
                .arg(arg!(<NANOPUB_FILE> "The file to check"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("completions")
                .about("Generates completion scripts for your shell")
                .arg(arg!([SHELL] "The shell to generate scripts for")
                    .value_parser(value_parser!(Shell)))
        );

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("sign", sub)) => {
            
        }
        Some(("publish", sub)) => {
            
        }
        Some(("check", sub)) => {
            
        }
        Some(("completions", sub)) => {
            
        }
        _ => {}
    }
    Ok(())
}
