use crate::db;
use crate::db::entity::student;
use argh::FromArgs;
use sea_orm::EntityTrait;
use std::io::{self, ErrorKind};

#[derive(FromArgs, PartialEq, Debug)]
/// Add student to class
#[argh(subcommand, name = "add")]
pub struct AddCommand {
    #[argh(option, short = 'i')]
    /// student id
    id: i64,

    #[argh(option, short = 'n')]
    /// student name
    name: String,

    #[argh(option, short = 'c')]
    /// student class
    class: i64,
}

pub async fn add(command: AddCommand) -> io::Result<()> {
    let db_conn = db::conn::connect().await?;

    if student::Entity::find_by_id(command.id)
        .one(&db_conn)
        .await
        .ok()
        .flatten()
        .is_some()
    {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Student id {} already in db.", command.id),
        ));
    }

    let model: student::ActiveModel = student::Model {
        id: command.id,
        name: command.name,
        class: command.class,
    }
    .into();
    student::Entity::insert(model)
        .exec(&db_conn)
        .await
        .map_err(|db_err| io::Error::new(ErrorKind::Other, db_err))?;
    Ok(())
}
