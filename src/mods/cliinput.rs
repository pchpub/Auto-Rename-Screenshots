use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

/// This is a cli program to rename screenshots
#[derive(StructOpt, Debug)]
pub enum Command {
    /// Add a new file
    Add {
        /// Activate debug mode
        #[structopt(long = "debug")]
        debug: bool,

        /// Set configuration file path
        #[structopt(
            short = "c",
            long = "config",
            default_value = "./config.json",
            parse(from_os_str)
        )]
        config: PathBuf,

        /// Add data to configuration
        #[structopt(short = "d", long = "data")]
        data: Option<Vec<String>>,
    },
    /// Remove file_name from configuration
    Remove {
        /// Activate debug mode
        #[structopt(long = "debug")]
        debug: bool,

        /// Set configuration file path
        #[structopt(
            short = "c",
            long = "config",
            default_value = "./config.json",
            parse(from_os_str)
        )]
        config: PathBuf,

        /// Remove data from configuration
        #[structopt(short = "d", long = "data")]
        data: Option<Vec<String>>,
    },
    /// Rename the files
    Rename {
        /// Activate debug mode
        #[structopt(long = "debug")]
        debug: bool,

        /// Input file folder
        #[structopt(
            short = "i",
            long = "input",
            default_value = "./input",
            parse(from_os_str)
        )]
        input: PathBuf,

        /// Output file
        #[structopt(
            short = "o",
            long = "output",
            default_value = "./output",
            parse(from_os_str)
        )]
        output: PathBuf,

        /// Set configuration file path
        #[structopt(
            short = "c",
            long = "config",
            default_value = "./config.json",
            parse(from_os_str)
        )]
        config: PathBuf,

        /// Compress pictures to zip or 7zip
        #[structopt(long = "compress_type")]
        compress_type: Option<CompressType>,
    },
}

#[derive(StructOpt, Debug)]
pub enum CompressType {
    #[structopt(name = "zip")]
    Zip,
    #[structopt(name = "7zip")]
    _7Zip,
    #[structopt(name = "no-compress")]
    NoCompress,
}

impl FromStr for CompressType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zip" => Ok(Self::Zip),
            "7zip" => Ok(Self::_7Zip),
            _ => Ok(Self::NoCompress),
        }
    }
}
