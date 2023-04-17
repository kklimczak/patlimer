use std::fmt;
use std::fmt::{Formatter, write};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, ErrorKind, Write};

use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use chrono::serde::ts_microseconds;
use chrono::{DateTime, Utc};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use tauri::Manager;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;
use crate::db::Db;
use crate::device::Commands;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Pilot {
    pub id: i64,
    pub name: String,
}

impl Pilot {
    pub fn new(id: i64, name: String) -> Pilot {
        Pilot { id, name }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NewPilotDto {
    pub race_event_id: i64,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RaceStatus {
    New,
    InProgress,
    Interrupted,
    Finished,
}

impl fmt::Display for RaceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromSql for RaceStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| {
            match s {
                "New" => Ok(RaceStatus::New),
                "InProgress" => Ok(RaceStatus::InProgress),
                "Interrupted" => Ok(RaceStatus::Interrupted),
                "Finished" => Ok(RaceStatus::Finished),
                _ => Err(FromSqlError::InvalidType)
            }
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Heat {
    id: i64,
    no: u8,
    channel: String,
    pilot_id: i64,
}

impl Heat {
    pub fn new(id: i64, no: u8, channel: String, pilot_id: i64) -> Heat {
        Heat {
            id, no, channel, pilot_id
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Race {
    id: i64,
    name: String,
    status: RaceStatus,
    heats: Vec<Heat>,
}

impl Race {
    pub fn new(id: i64, name: String, status: RaceStatus, heats: Vec<Heat>) -> Race {
        Race {id, name, status, heats}
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NewHeatDto {
    pub no: u8,
    pub pilot_id: i64,
    pub channel: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NewRaceDto {
    pub name: String,
    pub heats: Vec<NewHeatDto>,
    pub race_event_id: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RaceEventType {
    Local,
    Cloud,
}

impl fmt::Display for RaceEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromSql for RaceEventType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| {
            match s {
                "Local" => Ok(RaceEventType::Local),
                "Cloud" => Ok(RaceEventType::Cloud),
                _ => Err(FromSqlError::InvalidType)
            }
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RaceEvent {
    id: i64,
    name: String,
    race_event_type: RaceEventType,
    #[serde(with = "ts_microseconds")]
    created_at: DateTime<Utc>,
}

impl RaceEvent {
    pub fn new(id: i64, race_event_type: RaceEventType, created_at: DateTime<Utc>, name: String) -> RaceEvent {
        RaceEvent {
            id,
            name,
            race_event_type,
            created_at,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NewRaceEventDto {
    name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RaceEventDetailsDto {
    pilots: Vec<Pilot>,
    races: Vec<Race>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct State {
    upcoming_races: Vec<Race>,
    current_race: Option<Race>,
    pilots: Vec<Pilot>,
    race_events: Vec<RaceEvent>,
}

impl State {
    pub fn init(race_events: Vec<RaceEvent>) -> State {
        State {
            upcoming_races: Vec::new(),
            current_race: None,
            pilots: Vec::new(),
            race_events,
        }
    }
}

#[derive(Debug)]
pub struct InvokeRequest<T, K> {
    body: T,
    response_tx: oneshot::Sender<Result<K, ErrorMessage>>,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorMessage {
    message: String,
}

impl<T, K> InvokeRequest<T, K> {
    pub fn new(
        body: T,
    ) -> (
        InvokeRequest<T, K>,
        oneshot::Receiver<Result<K, ErrorMessage>>,
    ) {
        let (sender, receiver) = oneshot::channel();
        (
            InvokeRequest {
                body,
                response_tx: sender,
            },
            receiver,
        )
    }
}

#[derive(Debug)]
pub enum Actions {
    Init(InvokeRequest<(), State>),
    LoadRaceEvent(InvokeRequest<i64, RaceEventDetailsDto>),
    CreateRaceEvent(InvokeRequest<NewRaceEventDto, RaceEvent>),
    RemoveRaceEvent(InvokeRequest<i64, ()>),
    AddPilot(InvokeRequest<NewPilotDto, Pilot>),
    AddRace(InvokeRequest<NewRaceDto, Race>),
    StartRace(InvokeRequest<(), ()>),
}

pub async fn update_state(state: &mut State, mut rx: Receiver<Actions>, device_tx: Sender<Commands>) {
    let db = Db::new("db".to_string());

    while let Some(action) = rx.recv().await {
        dbg!(&action, &state);
        match action {
            Actions::Init(invoke_request) => {
                invoke_request.response_tx.send(Ok(state.clone())).unwrap();
            }
            Actions::LoadRaceEvent(invoke_request) => {
                let db = Db::new(invoke_request.body.to_string());

                let pilots = db.find_pilots();
                let races = db.find_races_with_heats();

                invoke_request.response_tx
                    .send(Ok(RaceEventDetailsDto {pilots, races}))
                    .unwrap();
            }
            Actions::CreateRaceEvent(invoke_request) => {
                if invoke_request.body.name.len() > 0 {
                    let new_race_event = db.insert_race(invoke_request.body.name.clone(), Utc::now(), RaceEventType::Local);
                    state.race_events.push(new_race_event.clone());

                    invoke_request
                        .response_tx
                        .send(Ok(new_race_event.clone()))
                        .unwrap();
                } else {
                    invoke_request
                        .response_tx
                        .send(Err(ErrorMessage {
                            message: format!("Missing 'name' property in RaceEvent").into(),
                        }))
                        .unwrap();
                }
            }
            Actions::AddPilot(invoke_request) => {
                if state.pilots.iter().any(|x| x.name == invoke_request.body.name) {
                    invoke_request
                        .response_tx
                        .send(Err(ErrorMessage {
                            message: format!(
                                "Pilot with name '{}' already exists",
                                invoke_request.body.name
                            )
                            .into(),
                        }))
                        .unwrap();
                } else {
                    let db = Db::new(invoke_request.body.race_event_id.to_string());
                    let new_pilot = db.insert_pilot(invoke_request.body.name);
                    state.pilots.push(new_pilot.clone());
                    invoke_request
                        .response_tx
                        .send(Ok(new_pilot))
                        .unwrap();
                }
            }
            Actions::AddRace(invoke_request) => {
                let mut db = Db::new(invoke_request.body.race_event_id.to_string());
                let new_race = db.insert_race_with_heats(invoke_request.body);
                state.upcoming_races.push(new_race.clone());
                invoke_request.response_tx.send(Ok(new_race)).unwrap();
            }
            Actions::RemoveRaceEvent(invoke_request) => {
                state.race_events.remove(state.race_events.iter().position(|x| x.id == invoke_request.body).unwrap());
                db.remove_race_event(invoke_request.body);
                invoke_request.response_tx.send(Ok(())).unwrap();
            }
            Actions::StartRace(invoke_request) => {
                device_tx.send(Commands::StartRace).await.unwrap();
                invoke_request.response_tx.send(Ok(())).unwrap();
            }
        }
        dbg!(&state);
    }
}
