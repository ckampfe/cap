use dirs;
use rusqlite::{Connection, NO_PARAMS};
use std::boxed::Box;
use std::error::Error;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use structopt::*;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "cap")]
struct Options {
    #[structopt(short = "l", long = "database-location", parse(from_str))]
    db_location: Option<PathBuf>,

    #[structopt(short = "r", long = "recent-entries")]
    recent_entries: Option<i64>,

    #[structopt()]
    entry: Option<String>,
}

#[derive(Debug)]
struct Entry {
    id: i32,
    body: String,
    created_at: time::Timespec,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();

    let db_location = if let Some(db_location) = options.db_location {
        db_location
    } else {
        let homedir = dirs::home_dir().ok_or("Could not get home dir")?;
        let mut db_location = PathBuf::new();
        db_location.push(homedir);
        db_location.push("cap.db");
        db_location
    };

    let conn = Connection::open(db_location)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        body TEXT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );",
        NO_PARAMS,
    )?;

    let entry = if let Some(entry_arg) = options.entry {
        entry_arg
    } else {
        let mut entry_buf = String::new();
        io::stdin().read_to_string(&mut entry_buf)?;
        entry_buf
    };

    conn.execute("INSERT INTO entries (body) VALUES (?1)", &[entry])?;

    if let Some(n_entries_to_echo) = options.recent_entries {
        let mut select_star = conn.prepare(
            "SELECT * FROM entries
                 ORDER BY created_at DESC
                 LIMIT ?;",
        )?;

        let entries = select_star.query_map(&[n_entries_to_echo], |row| {
            Ok(Entry {
                id: row.get(0)?,
                body: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;

        for entry in entries {
            let e = &(entry?);
            println!(
                "{:?}, {:?}, {:?}",
                e.id,
                e.body,
                time::strftime("%FT%TZ", &time::at_utc(e.created_at))?
            );
        }
    }

    Ok(())
}
