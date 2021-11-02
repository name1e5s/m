use crate::db;
use crate::db::entity::student;
use argh::FromArgs;
use sea_orm::{EntityTrait, Set};
use std::io::{self, ErrorKind};

#[derive(FromArgs, PartialEq, Debug)]
/// update student by id
#[argh(subcommand, name = "update")]
pub struct UpdateCommand {
    #[argh(option, short = 'i')]
    /// student id
    id: i64,

    #[argh(option, short = 'n')]
    /// student name
    name: Option<String>,

    #[argh(option, short = 'c')]
    /// student class
    class: Option<i64>,
}

pub async fn update(command: UpdateCommand) -> io::Result<()> {
    let db_conn = db::conn::connect().await?;

    let model = student::Entity::find_by_id(command.id)
        .one(&db_conn)
        .await
        .map_err(|db_err| io::Error::new(ErrorKind::Other, db_err))?;
    if let Some(model) = model {
        let mut model: student::ActiveModel = model.into();
        if let Some(class) = command.class {
            model.class = Set(class);
        }
        if let Some(name) = command.name {
            model.name = Set(name);
        }
        Ok(())
    } else {
        Err(io::Error::new(
            ErrorKind::Other,
            "id not found, use add to create it",
        ))
    }
}
