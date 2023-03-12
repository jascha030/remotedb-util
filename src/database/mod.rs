use crate::command::Command;
use mysql::prelude::*;
use mysql::*;
use rpassword::read_password;
use std::fs;
use std::process::Command;
use std::process::{exit, Stdio};
use structopt::StructOpt;

struct DatabaseCredentials {
    name: String,
    user: String,
    password: String,
}

impl DatabaseCredentials {
    pub fn new(user: String, name: String) -> Self {
        println!("Database password: ");

        DatabaseCredentials {
            name: String::from(name),
            user: String::from(user),
            password: read_password().unwrap(),
        }
    }

    pub fn password_display(&self) -> String {
        let mut display: String = String::from("");
        let mut i = 0;

        while i < self.password.chars().count() {
            display.push_str("*");
            i = i + 1;
        }

        return display;
    }
}
