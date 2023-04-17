// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod db;
mod device;

use std::fmt::format;
use crate::core::{ErrorMessage, InvokeRequest, RaceEventDetailsDto};
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;
use serialport::SerialPortType::UsbPort;
use tauri::{Manager, Window};
use tokio::select;
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
    new_pilot_dto: core::NewPilotDto,
    state: tauri::State<'_, LocalState>,
) -> Result<core::Pilot, ErrorMessage> {
    let (request, receiver) = InvokeRequest::new(new_pilot_dto.clone());
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

#[tauri::command]
async fn remove_race_event(
    race_event_id: i64,
    state: tauri::State<'_, LocalState>
) -> Result<(), ErrorMessage> {
    let (request, receiver) = InvokeRequest::new(race_event_id);
    let mut lock = state.dispatch.lock().await;
    lock.send(core::Actions::RemoveRaceEvent(request))
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    receiver.await.unwrap()
}

#[tauri::command]
async fn find_race_event_details(
    race_event_id: i64,
    state: tauri::State<'_, LocalState>
) -> Result<RaceEventDetailsDto, ErrorMessage> {
    let (request, receiver) = InvokeRequest::new(race_event_id);
    let mut lock = state.dispatch.lock().await;
    lock.send(core::Actions::LoadRaceEvent(request))
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    receiver.await.unwrap()
}

#[tauri::command]
async fn start_race(
    state: tauri::State<'_, LocalState>
) -> Result<(), ErrorMessage> {
    let (request, receiver) = InvokeRequest::new(());
    let mut lock = state.dispatch.lock().await;
    lock.send(core::Actions::StartRace(request))
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    receiver.await.unwrap()
}

fn main() {
    let mut state = core::State::init(Db::init());

    let (dispatch, listener) = mpsc::channel(5);
    let (device_tx, device_rx) = mpsc::channel(5);

    let token = tokio_util::sync::CancellationToken::new();
    let cloned_token = token.clone();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_pilot,
            add_race,
            init,
            create_race_event,
            remove_race_event,
            find_race_event_details,
            start_race,
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
                core::update_state(&mut state, listener, device_tx).await;
            });

            tauri::async_runtime::spawn(async move {
                select! {
                    _ = cloned_token.cancelled() => {}
                    _ = device::process_data(device_rx) => {}
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |_app_handle, event| match event {
            tauri::RunEvent::Exit => {
                token.cancel();
            },
            _ => {}
        });
}
