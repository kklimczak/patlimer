use chrono::{DateTime, Utc};
use rusqlite::{Connection, params, Result};
use crate::core::{RaceEvent, RaceEventType};

pub struct Db {
    connection: Connection,
}

impl Db {
    pub fn new() -> Db {
        let connection = Connection::open("db").expect("Can not open the database!");

        Db {
            connection
        }
    }


    pub fn init() -> Vec<RaceEvent> {
        let connection = Connection::open("db").expect("Can not open the database!");

        connection.execute("CREATE TABLE IF NOT EXISTS raceEvents (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL,
            race_event_type TEXT NOT NULL
        )", ()).expect("Can not create the race events table!");

        let mut statement = connection.prepare("SELECT id, race_event_type, created_at, name FROM raceEvents").expect("Can not prepare the statement!");

        let race_events_iter = statement.query_map([], |row| {
            Ok(RaceEvent::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        }).unwrap();

        race_events_iter.map(|r| r.unwrap()).collect()
    }

    pub fn insert_race(&self, name: String, created_at: DateTime<Utc>, race_event_type: RaceEventType) -> RaceEvent {
        self.connection.execute(
            "INSERT INTO raceEvents (name, created_at, race_event_type) VALUES (?1, ?2, ?3)",
            params![name, created_at, race_event_type.to_string()]
        ).unwrap();
        RaceEvent::new(self.connection.last_insert_rowid(), race_event_type, created_at, name)
    }
}
