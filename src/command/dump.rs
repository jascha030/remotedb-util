use ssh::*;
use super::{Run, Dump};

impl Run for Dump {
    fn run(&self) -> Result<(), std::io::Error> {
        (_)
    }
}
