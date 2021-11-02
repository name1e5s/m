mod add;
mod delete;
mod export;
mod find;
mod update;

use argh::FromArgs;

use add::add;
use add::AddCommand;
use delete::{delete, DeleteCommand};
use export::{export, ExportCommand};
use find::{find, FindCommand};
use std::io;
use update::update;
use update::UpdateCommand;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "student")]
/// Student management command
pub struct StudentCommand {
    #[argh(subcommand)]
    subcommand: SubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommandEnum {
    Add(AddCommand),
    Delete(DeleteCommand),
    Find(FindCommand),
    Update(UpdateCommand),
    Export(ExportCommand),
}

pub async fn student(command: StudentCommand) -> io::Result<()> {
    match command.subcommand {
        SubCommandEnum::Add(command) => add(command).await,
        SubCommandEnum::Find(command) => find(command).await,
        SubCommandEnum::Update(command) => update(command).await,
        SubCommandEnum::Delete(command) => delete(command).await,
        SubCommandEnum::Export(command) => export(command).await,
    }
}
