use crate::db;
use crate::db::entity::student;
use argh::FromArgs;
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::io::{self, ErrorKind};

#[derive(FromArgs, PartialEq, Debug)]
/// delete student by id
#[argh(subcommand, name = "delete")]
pub struct DeleteCommand {
    #[argh(option, short = 'i')]
    /// student id
    id: i64,
}

pub async fn delete(command: DeleteCommand) -> io::Result<()> {
    let db_conn = db::conn::connect().await?;

    let model = student::Entity::find_by_id(command.id)
        .one(&db_conn)
        .await
        .ok()
        .flatten();
    if let Some(model) = model {
        let active_model: student::ActiveModel = model.into();
        active_model
            .delete(&db_conn)
            .await
            .map_err(|db_err| io::Error::new(ErrorKind::Other, db_err))?;
    }
    Ok(())
}
