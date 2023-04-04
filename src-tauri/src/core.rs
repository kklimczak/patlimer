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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Pilot {
    name: String,
    raceEventId: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum RaceStatus {
    New,
    InProgress,
    Interrupted,
    Finished,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Heat {
    no: u8,
    channel: String,
    pilot: Pilot,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Race {
    id: String,
    raceEventId: i64,
    name: String,
    status: RaceStatus,
    heats: Vec<Heat>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NewRaceDto {
    name: String,
    heats: Vec<Heat>,
    raceEventId: i64,
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
    CreateRaceEvent(InvokeRequest<NewRaceEventDto, RaceEvent>),
    AddPilot(InvokeRequest<Pilot, Pilot>),
    AddRace(InvokeRequest<NewRaceDto, Race>),
}

pub async fn update_state(state: &mut State, mut rx: Receiver<Actions>) {
    let db = Db::new();

    while let Some(action) = rx.recv().await {
        dbg!(&action, &state);
        match action {
            Actions::Init(invoke_request) => {
                invoke_request.response_tx.send(Ok(state.clone())).unwrap();
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
                if state.pilots.contains(&invoke_request.body) {
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
                    state.pilots.push(invoke_request.body.clone());
                    invoke_request
                        .response_tx
                        .send(Ok(invoke_request.body))
                        .unwrap();
                }
            }
            Actions::AddRace(invoke_request) => {
                let new_race = Race {
                    name: invoke_request.body.name,
                    heats: invoke_request.body.heats,
                    id: state.upcoming_races.len().to_string(),
                    status: RaceStatus::New,
                    raceEventId: invoke_request.body.raceEventId,
                };

                state.upcoming_races.push(new_race.clone());
                invoke_request.response_tx.send(Ok(new_race)).unwrap();
            }
        }
        dbg!(&state);
    }
}
