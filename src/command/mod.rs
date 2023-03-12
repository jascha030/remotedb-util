use std::io::Error;

use clap::Parser;

pub mod dump;

#[derive(Parser)]
#[clap(
    name = "sshsql",
    author = "Jascha030 <contact@jaschavanaalst.nl>",
    version = "0.1.0",
    about = "Mysqldump over ssh",
    long_about = None
)]
pub enum Command {
    Dump(Dump),
}

#[derive(Parser)]
pub struct Dump {
    #[clap(short, long)]
    ssh: String,
    #[clap(short, long)]
    db_user: String,
    #[clap(short, long)]
    db_pass: String,
    #[clap(short, long)]
    db_name: String,
}

pub trait Run {
    fn run(&self) -> Result<(), Error>;
}

impl Run for Command {
    fn run(&self) -> Result<(), Error> {
        match self {
            Command::Dump(cmd) => cmd.run(),
        }
    }
}
