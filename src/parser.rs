use clap_derive::Parser;

#[derive(Parser)]
pub struct CommandArgument {
    #[clap(short, long)]
    pub day: Option<u32>,

    #[clap(short, long)]
    pub year: Option<i32>,

    pub part: Option<u32>,

    #[clap(short, long)]
    pub publish: bool,

    #[clap(long)]
    pub all: bool,
}
