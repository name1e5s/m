use crate::db;
use crate::db::entity::student;
use argh::FromArgs;
use sea_orm::{ConnectionTrait, DbConn, EntityTrait, Schema};
use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::path::Path;

#[derive(FromArgs, PartialEq, Debug)]
/// Init database for manager
#[argh(subcommand, name = "init")]
pub struct InitCommand {
    #[argh(switch)]
    /// use stdin to receive student info
    stdin: bool,

    #[argh(option)]
    /// receive student info by file
    file: Option<String>,
}

pub async fn init(command: InitCommand) -> io::Result<()> {
    // Parse command
    let mut student_info: Vec<student::ActiveModel> = vec![];

    if command.stdin {
        student_info.extend(parse_stdin()?);
    } else if let Some(file) = command.file {
        student_info.extend(parse_file(file)?);
    } else {
        return Err(io::Error::new(ErrorKind::Other, "No input found"));
    }

    // Check database file
    let file_path = db::conn::get_db_file();
    if !Path::new(&file_path).exists() {
        File::create(&file_path).expect("Cannot create db file");
    }

    let db_conn = db::conn::connect().await?;
    setup_schema(&db_conn).await?;
    student::Entity::insert_many(student_info)
        .exec(&db_conn)
        .await
        .map_err(|db_err| io::Error::new(ErrorKind::Other, db_err))?;
    eprintln!("Init manager database succeed.");
    Ok(())
}

async fn setup_schema(db_conn: &DbConn) -> io::Result<()> {
    let create_student_stmt = Schema::create_table_from_entity(student::Entity);
    db_conn
        .execute(db_conn.get_database_backend().build(&create_student_stmt))
        .await
        .map_err(|db_err| io::Error::new(ErrorKind::Other, db_err))?;
    Ok(())
}

fn parse_student_info(line: String) -> io::Result<student::ActiveModel> {
    let info: Vec<&str> = line.split(":").collect();
    let id = info.get(0).map(|&f| f.parse::<i64>().ok()).flatten();
    let name = info.get(1).map(|&f| f.to_owned());
    let class = info.get(2).map(|&f| f.parse::<i64>().ok()).flatten();
    if let (Some(id), Some(class)) = (id, class) {
        let name = name.unwrap_or(id.to_string());
        return Ok(student::Model { id, name, class }.into());
    } else {
        return Err(io::Error::new(ErrorKind::Other, "Unexpected stdin format"));
    }
}

/// Info from stdin should has the following format:
/// <id>:<name>:<class>
fn parse_stdin() -> io::Result<Vec<student::ActiveModel>> {
    let mut vec = vec![];
    let lines = io::stdin().lines();
    for line in lines {
        let line = line?;
        vec.push(parse_student_info(line)?);
    }
    Ok(vec)
}

/// Info from file has the same format as stdin.
fn parse_file(file: String) -> io::Result<Vec<student::ActiveModel>> {
    let mut vec = vec![];
    let file = File::open(file)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line?;
        vec.push(parse_student_info(line)?);
    }
    Ok(vec)
}
