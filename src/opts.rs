use clap::Parser;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(name = "rctl", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats.")]
    Csv(CsvOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum FileFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: FileFormat,
    #[arg(long, default_value_t = true)]
    header: bool,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
}

fn verify_file(filepath: &str) -> Result<String, &'static str> {
    if Path::new(filepath).exists() {
        Ok(filepath.to_string())
    } else {
        Err("Input file does not exist")
    }
}

fn parse_format(format: &str) -> Result<FileFormat, anyhow::Error> {
    format.parse()
}

impl From<FileFormat> for &'static str {
    fn from(format: FileFormat) -> Self {
        match format {
            FileFormat::Json => "json",
            FileFormat::Yaml => "yaml",
            FileFormat::Toml => "toml",
        }
    }
}

impl FromStr for FileFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(FileFormat::Json),
            "yaml" => Ok(FileFormat::Yaml),
            "toml" => Ok(FileFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
