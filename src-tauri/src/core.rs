use tauri::Manager;
use tokio::sync::mpsc::{channel, Receiver, Sender};

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

#[derive(Debug)]
pub enum Actions {
    AddPilot(Pilot)
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
            Actions::AddPilot(pilot) => {
                state.pilots.push(pilot);
            }
        }
        dbg!(&state);
    }
}
