use tauri::Manager;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Pilot {
    name: String,
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
    name: String,
    status: RaceStatus,
    heats: Vec<Heat>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NewRaceDto {
    name: String,
    heats: Vec<Heat>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct State {
    upcoming_races: Vec<Race>,
    current_race: Option<Race>,
    pilots: Vec<Pilot>,
}

impl State {
    pub fn init() -> State {
        State {
            upcoming_races: Vec::new(),
            current_race: None,
            pilots: Vec::new(),
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
    pub fn new(body: T) -> (InvokeRequest<T, K>, oneshot::Receiver<Result<K, ErrorMessage>>) {
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
    AddPilot(InvokeRequest<Pilot, Pilot>),
    AddRace(InvokeRequest<NewRaceDto, Race>),
}

pub async fn update_state(state: &mut State, mut rx: Receiver<Actions>) {
    while let Some(action) = rx.recv().await {
        dbg!(&action, &state);
        match action {
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
            },
            Actions::AddRace(invoke_request) => {
                let new_race = Race {
                    name: invoke_request.body.name,
                    heats: invoke_request.body.heats,
                    id: state.upcoming_races.len().to_string(),
                    status: RaceStatus::New,
                };

                state.upcoming_races.push(new_race.clone());
                invoke_request
                    .response_tx
                    .send(Ok(new_race))
                    .unwrap();
            }
        }
        dbg!(&state);
    }
}
