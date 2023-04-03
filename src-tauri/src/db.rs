use rusqlite::{Connection, Result};

pub struct Db {
    connection: Connection,
}

impl Db {
    pub fn new() -> Db {
        let connection = Connection::open_in_memory().expect("Can not open the database!");

        connection.execute("CREATE TABLE IF NOT EXISTS raceEvents (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )", ()).expect("Can not create the race events table!");

        Db {
            connection
        }
    }

    pub fn insert_race(&self, name: String) -> i64 {
        self.connection.execute(
            "INSERT INTO raceEvents (name) VALUES (?1)", [name]
        ).unwrap();
        self.connection.last_insert_rowid()
    }
}
