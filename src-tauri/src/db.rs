use chrono::{DateTime, Utc};
use rusqlite::{Connection, params, Result};
use crate::core::{Heat, NewRaceDto, Pilot, Race, RaceEvent, RaceEventType, RaceStatus};

pub struct Db {
    connection: Connection,
    name: String,
}

impl Db {
    pub fn new(name: String) -> Db {
        let connection = Connection::open(&name).expect("Can not open the database!");

        Db {
            connection,
            name
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

        let mut connection = Connection::open(format!("{}", self.connection.last_insert_rowid())).expect("Can not open the race event database during first connection");

        let tx = connection.transaction().unwrap();

        tx.execute("CREATE TABLE IF NOT EXISTS pilots (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )", ()).expect("Can not create the pilots table!");

        tx.execute("CREATE TABLE IF NOT EXISTS races (
            id INTEGER PRIMARY KEY,
            name INTEGER NOT NULL,
            status TEXT NOT NULL
        )", ()).expect("Can not create the races table!");

        tx.execute("CREATE TABLE IF NOT EXISTS heats (
            id INTEGER PRIMARY KEY,
            no INTEGER NOT NULL,
            channel TEXT NOT NULL,
            pilot_id INTEGER NOT NULL,
            race_id INTEGER NOT NULL,
            rssi_raw TEXT NOT NULL,
            FOREIGN KEY(pilot_id) REFERENCES pilots(id),
            FOREIGN KEY(race_id) REFERENCES races(id)
        )", ()).expect("Can not create the heats table!");

        tx.commit().expect("Can not perform transaction!");

        RaceEvent::new(self.connection.last_insert_rowid(), race_event_type, created_at, name)
    }

    pub fn remove_race_event(&self, race_event_id: i64) {
        let connection = Connection::open(format!("{}", self.connection.last_insert_rowid())).expect("Can not open the race event database");
        std::fs::remove_file(format!("{}", race_event_id)).unwrap_or(());
        self.connection
            .execute("DELETE FROM raceEvents WHERE id = ?1", params![race_event_id]).expect("Can not remove raceEvent!");
    }

    pub fn insert_pilot(&self, name: String) -> Pilot {
        self.connection.execute(
            "INSERT INTO pilots (name) VALUES (?1)",
            params![name]
        ).unwrap();

        Pilot {name, id: self.connection.last_insert_rowid()}
    }

    pub fn find_pilots(&self) -> Vec<Pilot> {
        let mut statement = self.connection.prepare("SELECT id, name FROM pilots").expect("Can not prepare the statement!");

        let pilots_iter = statement.query_map([], |row| {
            Ok(Pilot::new(row.get(0)?, row.get(1)?))
        }).unwrap();

        pilots_iter.map(|r| r.unwrap()).collect()
    }

    pub fn insert_race_with_heats(&mut self, new_race_dto: NewRaceDto) -> Race {
        let tx = self.connection.transaction().unwrap();

        tx.execute(
            "INSERT INTO races (name, status) VALUES (?1, ?2)",
            params![new_race_dto.name, RaceStatus::New.to_string()]
        ).unwrap();

        let new_race_id = tx.last_insert_rowid();

        let heats = new_race_dto.heats.iter().map(|heat| {
            tx.execute(
                "INSERT INTO heats (no, channel, pilot_id, race_id, rssi_raw) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![heat.no, heat.channel, heat.pilot_id, new_race_id, ""]
            ).unwrap();

            Heat::new(tx.last_insert_rowid(), heat.no, heat.channel.clone(), heat.pilot_id)
        }).collect();

        tx.commit().unwrap();

        Race::new(new_race_id, new_race_dto.name, RaceStatus::New, heats)
    }

    pub fn find_races_with_heats(&self) -> Vec<Race> {
        let mut races_statement = self.connection.prepare(
            "SELECT id, name, status FROM races"
        ).unwrap();

        races_statement.query_map([], |row| {
            let race_id: i64 = row.get(0)?;

            let mut heats_statement = self.connection.prepare(
                "SELECT id, no, channel, pilot_id FROM heats WHERE race_id = ?1"
            ).unwrap();

            let heats_iter = heats_statement.query_map([race_id], |heat_row| {
                Ok(Heat::new(heat_row.get(0)?, heat_row.get(1)?, heat_row.get(2)?, heat_row.get(3)?))
            }).unwrap().map(|heat| heat.unwrap()).collect();

            Ok(Race::new(race_id, row.get(1)?, row.get(2)?, heats_iter))
        }).unwrap().map(|race| race.unwrap()).collect()
    }
}
