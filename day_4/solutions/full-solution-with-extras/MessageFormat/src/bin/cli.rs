use anyhow::{Error, Result, anyhow};
use std::fmt::Debug;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
pub enum Input {
    Stdin,
    File { path: PathBuf },
}

#[derive(Debug)]
pub enum Output {
    Stdout,
    File { path: PathBuf },
}

impl Into<BufReader<Box<dyn std::io::Read>>> for Input {
    fn into(self) -> BufReader<Box<dyn std::io::Read>> {
        match self {
            Input::Stdin => BufReader::new(Box::new(std::io::stdin())),
            Input::File { path } => BufReader::new(Box::new(std::fs::File::open(path).unwrap())),
        }
    }
}

impl Into<BufWriter<Box<dyn std::io::Write>>> for Output {
    fn into(self) -> BufWriter<Box<dyn std::io::Write>> {
        match self {
            Output::Stdout => BufWriter::new(Box::new(std::io::stdout())),
            Output::File { path } => BufWriter::new(Box::new(std::fs::File::open(path).unwrap())),
        }
    }
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "stdin" => Ok(Input::Stdin),
            _ => {
                let path = Path::new(s).to_path_buf();
                if path.exists() {
                    Ok(Input::File { path })
                } else {
                    Err(anyhow!("Could not find file {:?}", path))
                }
            }
        }
    }
}

impl FromStr for Output {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "stdout" => Ok(Output::Stdout),
            _ => {
                let path = Path::new(s).to_path_buf();
                if !path.exists() {
                    Ok(Output::File { path })
                } else {
                    Err(anyhow!("File already exists {:?}", path))
                }
            }
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "FgwMessageEncoder",
about = "Encodes fge messages into a byte stream."
)]
#[structopt(global_settings(& [structopt::clap::AppSettings::ColoredHelp]))]
pub struct Encoder {
    #[structopt(name = "input-file", default_value = "stdin")]
    #[structopt(help = "containing the binary fge data")]
    pub input: Input,

    #[structopt(name = "output-file", default_value = "stdout")]
    #[structopt(help = "containing the binary fge data")]
    pub output: Output,

    #[structopt(name = "count", short = "c")]
    #[structopt(help = "exit if this number of messages have been decoded")]
    pub count: Option<usize>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "FgwMessageDecoder", about = "Decodes a fge byte stream.")]
#[structopt(global_settings(& [structopt::clap::AppSettings::ColoredHelp]))]
pub struct Decoder {
    #[structopt(name = "input-file", default_value = "stdin")]
    #[structopt(help = "containing the binary fge data")]
    pub input: Input,

    #[structopt(name = "count", short = "c")]
    #[structopt(help = "exit if this number of messages have been decoded")]
    pub count: Option<usize>,
}
