//! `organon-protocol`: check that a stream speaks the protocol, and print its JSON schemas.

use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context as _;
use clap::{Parser, Subcommand};
use organon_protocol::{check_stream, Message, PROTOCOL_VERSION};

#[derive(Parser)]
#[command(
    name = "organon-protocol",
    version,
    about = "Validate organon protocol streams; print JSON schemas"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Check a JSONL stream (file, or `-` for stdin). Exit 0 if valid, 1 if not.
    Validate { input: PathBuf },
    /// Print the JSON Schema of a stream line (`Message`).
    Schema,
    /// Print the protocol major version.
    Version,
}

fn main() -> anyhow::Result<ExitCode> {
    match Cli::parse().cmd {
        Cmd::Validate { input } => {
            let report = if input.as_os_str() == "-" {
                check_stream(io::stdin().lock())?
            } else {
                let f =
                    File::open(&input).with_context(|| format!("opening {}", input.display()))?;
                check_stream(BufReader::new(f))?
            };
            for e in &report.errors {
                eprintln!("{e}");
            }
            println!(
                "{} messages, {} errors",
                report.messages.len(),
                report.errors.len()
            );
            Ok(if report.is_ok() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
        Cmd::Schema => {
            let schema = schemars::schema_for!(Message);
            println!("{}", serde_json::to_string_pretty(&schema)?);
            Ok(ExitCode::SUCCESS)
        }
        Cmd::Version => {
            println!("{PROTOCOL_VERSION}");
            Ok(ExitCode::SUCCESS)
        }
    }
}
