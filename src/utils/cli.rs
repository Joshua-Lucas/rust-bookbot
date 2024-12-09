use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    #[clap(short = 't', long = "title")]
    pub title: String,
}
