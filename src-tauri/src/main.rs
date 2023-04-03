// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod db;

use crate::core::{ErrorMessage, InvokeRequest};
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::sync::Arc;
use tauri::{Manager, Window};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use crate::db::Db;

struct LocalState {
    dispatch: Arc<Mutex<Sender<core::Actions>>>,
}

#[tauri::command]
async fn init(state: tauri::State<'_, LocalState>) -> Result<core::State, ErrorMessage> {
    let (request, receiver) = InvokeRequest::new(());
    let mut lock = state.dispatch.lock().await;
    lock.send(core::Actions::Init(request))
        .await
        .map_err(|e1| e1.to_string())
        .unwrap();

    receiver.await.unwrap()
}

#[tauri::command]
async fn create_race_event(
    new_race_event_dto: core::NewRaceEventDto,
    state: tauri::State<'_, LocalState>,
) -> Result<core::RaceEvent, ErrorMessage> {
    let (request, receiver) = InvokeRequest::new(new_race_event_dto.clone());
    let mut lock = state.dispatch.lock().await;
    lock.send(core::Actions::CreateRaceEvent(request))
        .await
        .map_err(|e1| e1.to_string())
        .unwrap();

    receiver.await.unwrap()
}

#[tauri::command]
async fn set_pilot(
    pilot: core::Pilot,
    state: tauri::State<'_, LocalState>,
) -> Result<core::Pilot, ErrorMessage> {
    println!("{:?}", pilot);
    let (request, receiver) = InvokeRequest::new(pilot.clone());
    let mut lock = state.dispatch.lock().await;
    lock.send(core::Actions::AddPilot(request))
        .await
        .map_err(|e1| e1.to_string())
        .unwrap();

    receiver.await.unwrap()
}

#[tauri::command]
async fn add_race(
    new_race_dto: core::NewRaceDto,
    state: tauri::State<'_, LocalState>,
) -> Result<core::Race, ErrorMessage> {
    let (request, receiver) = InvokeRequest::new(new_race_dto.clone());
    let mut lock = state.dispatch.lock().await;
    lock.send(core::Actions::AddRace(request))
        .await
        .map_err(|e1| e1.to_string())
        .unwrap();

    receiver.await.unwrap()
}

fn main() {
    let mut state = core::State::init();

    let (dispatch, listener) = mpsc::channel(5);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_pilot,
            add_race,
            init,
            create_race_event
        ])
        .manage(LocalState {
            dispatch: Arc::new(Mutex::new(dispatch.clone())),
        })
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window: Window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            // let app_handle = app.handle();

            tauri::async_runtime::spawn(async move {
                core::update_state(&mut state, listener).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
