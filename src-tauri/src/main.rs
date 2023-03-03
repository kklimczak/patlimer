// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;

use std::sync::Arc;
use tauri::{Manager, Window};
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Sender;

struct LocalState {
    dispatch: Arc<Mutex<Sender<core::Actions>>>,
}

#[tauri::command]
async fn set_pilot(pilot: core::Pilot, state: tauri::State<'_, LocalState>) -> Result<core::Pilot, String> {
    println!("{:?}", pilot);
    let mut lock = state.dispatch.lock().await;
    lock.send(core::Actions::AddPilot(pilot.clone())).await
        .map(|_| pilot)
        .map_err(|e1|e1.to_string())
}

fn main() {
    let mut state = core::State::init();

    let (dispatch, listener) = mpsc::channel(5);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_pilot])
        .manage(LocalState {dispatch: Arc::new(Mutex::new(dispatch.clone()))})
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window: Window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            let app_handle = app.handle();

            tauri::async_runtime::spawn(async move {
                core::update_state(&mut state, listener, &app_handle).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
