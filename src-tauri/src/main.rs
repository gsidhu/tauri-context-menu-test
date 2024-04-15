// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{menu::{Menu, MenuItem}, Manager};


#[tauri::command]
fn open_context_menu(window: tauri::Window) {
  let manager = window.app_handle();
  let context_menu = Menu::with_items(manager, &[
    &MenuItem::with_id(manager, "open_file", "Open File", true, None::<&str>).unwrap(),
    &MenuItem::with_id(manager, "open_folder", "Open File Folder", true, None::<&str>).unwrap(),
  ]).unwrap();

  window.popup_menu(&context_menu).unwrap();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![ open_context_menu, greet ])
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
        #[cfg(desktop)]
        {
          app.on_menu_event(|_app_handle: &tauri::AppHandle, event| {
            println!("context menu clicked!");
            println!("menu event: {:?}", event);
          });
        }
        Ok(())
    })
    .menu(|app_handle| Menu::default(app_handle))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
