use chrono::{DateTime, Local, NaiveDateTime, NaiveDate};
use rusqlite::{params, Connection, Result, Error, RowIndex};
use rusqlite::NO_PARAMS;

#[derive(Debug)]
pub struct LogRecord {
    pub id: i32,
    pub message: String,
    pub time: DateTime<Local>
}

pub struct DbManager {
    connection: Connection
}

impl DbManager {
    pub fn new() -> Result<DbManager> {
        let conn = Connection::open("database.sqlite")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS logs (
                  id              INTEGER PRIMARY KEY,
                  message         TEXT NOT NULL,
                  time            DATETIME
                  )",
            params![],
        )?;

        return Ok(DbManager{
            connection: conn,
        });
    }

    pub fn insert(&self, message: &str, time: NaiveDateTime) -> Result<()> {
        self.connection.execute(
            "INSERT INTO logs (message, time) VALUES (?1, ?2)",
            params![message, time],
        )?;

        Ok(())
    }

    pub fn get_last_date(&self) -> Result<NaiveDateTime> {
        let mut stmt = self.connection.prepare("SELECT MAX(time) as time FROM logs WHERE DATE(time) < DATE('now')")?;
        let date_time = stmt.query_row(NO_PARAMS, |row| {
            Ok(row.get::<&str, NaiveDateTime>("time")?)
        });

        date_time
    }

    pub fn list_date(&self, date: NaiveDate) -> Result<Vec<LogRecord>> {
        let mut stmt = self.connection.prepare("SELECT id, message, time FROM logs WHERE DATE(time) = DATE(?1)")?;
        let row_iterator = stmt.query_map(params![date], |row| {
            Ok(LogRecord {
                id: row.get(0)?,
                message: row.get(1)?,
                time: row.get(2)?,
            })
        })?.collect();

        row_iterator
    }

    pub fn update(&self, id: i32, message: Option<String>, time: Option<NaiveDateTime>) -> Result<()> {
        if let Some(new_message) = message {
            self.connection.execute(
                "UPDATE logs SET message = ?1 WHERE id = ?2",
                params![new_message, id]
            )?;
        }

        if let Some(new_time) = time {
            self.connection.execute(
                "UPDATE logs SET time = ?1 WHERE id = ?2",
                params![new_time, id]
            )?;
        }

        Ok(())
    }

    pub fn delete(&self, id: i32) -> Result<()> {
        self.connection.execute("DELETE FROM logs WHERE id = ?1", params![id])?;

        Ok(())
    }
}
