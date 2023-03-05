use tauri::Manager;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Pilot {
    id: String,
    name: String,
}

#[derive(Debug)]
enum RaceStatus {
    New, InProgress, Interrupted, Finished,
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

#[derive(Debug, serde::Serialize)]
enum ResponseStatus {
    Success, Failure
}

#[derive(Debug, serde::Serialize)]
pub struct InvokeResponse<T> {
    status: ResponseStatus,
    data: T,
}

impl<T> InvokeResponse<T> {
    fn success(data: T) -> InvokeResponse<T> {
        InvokeResponse {
            status: ResponseStatus::Success,
            data
        }
    }

    fn failure(data: T) -> InvokeResponse<T> {
        InvokeResponse {
            status: ResponseStatus::Failure,
            data
        }
    }
}

#[derive(Debug)]
pub struct InvokeRequest<T> {
    body: T,
    response_tx: oneshot::Sender<InvokeResponse<T>>
}

impl<T> InvokeRequest<T> {
    pub fn new(body: T) -> (InvokeRequest<T>, oneshot::Receiver<InvokeResponse<T>>) {
        let (sender, receiver) = oneshot::channel();
        (InvokeRequest {
            body,
            response_tx: sender
        }, receiver)
    }
}

#[derive(Debug)]
pub enum Actions {
    AddPilot(InvokeRequest<Pilot>)
}

#[derive(Clone, serde::Serialize)]
struct ActionPayload<T> {
    actionType: String,
    data: T
}

pub async fn update_state<R: tauri::Runtime>(state: &mut State, mut rx: Receiver<Actions>, app_handle: &impl Manager<R>) {
    while let Some(action) = rx.recv().await {
        dbg!(&action, &state);
        match action {
            Actions::AddPilot(invoke_request) => {
                state.pilots.push(invoke_request.body.clone());
                invoke_request.response_tx.send(InvokeResponse::success(invoke_request.body)).unwrap();
            }
        }
        dbg!(&state);
    }
}
