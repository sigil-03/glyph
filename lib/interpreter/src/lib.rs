use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
/// Glyphics interpreter
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Glyphic Error")]
    GlyphicError,
    #[error("Parsing Error")]
    ParseError(#[from] clap::error::Error),
}

#[derive(Parser, ValueEnum, Serialize, Debug, Clone)]
pub enum Commands {
    #[value()]
    Test,
}

pub fn interpret_glyphic(input: &str) -> Result<Commands, Error> {
    // here's some bullshit bc i'm too tired to write my own parser atm
    // and clap for whatever reason NEEDS a first value
    let s = format!("bs {}", input);
    Ok(Commands::try_parse_from(s.split(" "))?)
}
