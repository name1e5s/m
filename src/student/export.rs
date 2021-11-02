use crate::db;
use crate::db::entity::student;
use argh::FromArgs;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use std::fs::{self, File};
use std::io::{self, ErrorKind, Write};
use std::path::Path;

#[derive(FromArgs, PartialEq, Debug)]
/// find student by id
#[argh(subcommand, name = "export")]
pub struct ExportCommand {
    #[argh(option, short = 'c')]
    /// student id
    class: Vec<i64>,

    #[argh(switch)]
    /// use stdin to export student info
    stdout: bool,

    #[argh(option, short = 'f')]
    /// file to output
    file: Option<String>,

    #[argh(switch, short = 'm')]
    /// output format as m init file(default)
    _format_m_init: bool,

    #[argh(switch, short = 's')]
    /// output format as student list
    format_student_list: bool,

    #[argh(switch, short = 'a')]
    /// output format as account list
    format_account_list: bool,
}

pub async fn export(command: ExportCommand) -> io::Result<()> {
    let db_conn = db::conn::connect().await?;
    let mut result: Vec<student::Model> = vec![];
    if command.class.is_empty() {
        result.extend(
            student::Entity::find()
                .all(&db_conn)
                .await
                .map_err(|db_err| io::Error::new(ErrorKind::Other, db_err))?,
        );
    } else {
        let mut conditions = Condition::any();
        for i in command.class {
            conditions = conditions.add(student::Column::Class.eq(i));
        }
        result.extend(
            student::Entity::find()
                .filter(conditions)
                .all(&db_conn)
                .await
                .map_err(|db_err| io::Error::new(ErrorKind::Other, db_err))?,
        );
    }
    let mut output = "".to_owned();
    if command.format_account_list {
        for i in result {
            output.push_str(&format!("{}:bupt{}\n", i.id, i.id));
        }
    } else if command.format_student_list {
        for i in result {
            output.push_str(&format!("{}\n", i.id));
        }
    } else {
        for i in result {
            output.push_str(&format!("{}:{}:{}\n", i.id, i.name, i.class));
        }
    }
    if let Some(file) = command.file {
        write_to_file(file, output)
    } else {
        print!("{}", output);
        Ok(())
    }
}

fn write_to_file(path: String, output: String) -> io::Result<()> {
    if Path::new(&path).exists() {
        fs::remove_file(&path).expect("Cannot remove old file");
    }
    let mut file = File::create(&path).expect("Cannot create file");
    file.write_all(output.as_bytes())
}
