use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub url: Option<String>,

    #[clap(short, long, value_parser)]
    pub path: Option<PathBuf>,
}
