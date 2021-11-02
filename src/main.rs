#![feature(stdin_forwarders)]
mod db;
mod init;
mod student;

use argh::FromArgs;
use init::{init, InitCommand};
use std::io;
use student::student;
use student::StudentCommand;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
struct TopLevel {
    #[argh(subcommand)]
    subcommand: SubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommandEnum {
    Init(InitCommand),
    Student(StudentCommand),
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let command: TopLevel = argh::from_env();
    let result = match command.subcommand {
        SubCommandEnum::Init(command) => init(command).await,
        SubCommandEnum::Student(command) => student(command).await,
    };
    result.map(|_| {
        eprintln!("SUCCESS");
        ()
    })
}
