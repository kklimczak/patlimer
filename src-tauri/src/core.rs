use tauri::Manager;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Pilot {
    name: String,
}

#[derive(Debug)]
enum RaceStatus {
    New,
    InProgress,
    Interrupted,
    Finished,
}

#[derive(Debug)]
struct Heat {
    number: u8,
    channel: String,
    pilot: Pilot,
    rssis: Vec<String>,
}

#[derive(Debug)]
struct Race {
    id: String,
    name: String,
    status: RaceStatus,
    heats: Vec<Heat>,
}

#[derive(Debug)]
pub struct State {
    current_race: Option<Race>,
    pilots: Vec<Pilot>,
}

impl State {
    pub fn init() -> State {
        State {
            current_race: None,
            pilots: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct InvokeRequest<T> {
    body: T,
    response_tx: oneshot::Sender<Result<T, ErrorMessage>>,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorMessage {
    message: String,
}

impl<T> InvokeRequest<T> {
    pub fn new(body: T) -> (InvokeRequest<T>, oneshot::Receiver<Result<T, ErrorMessage>>) {
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
    AddPilot(InvokeRequest<Pilot>),
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
            }
        }
        dbg!(&state);
    }
}
