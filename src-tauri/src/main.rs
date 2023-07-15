// this hides the console for Windows release builds
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager; // used by .get_window
use tauri::{self, SystemTrayEvent, SystemTrayMenuItem};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};
use tauri_plugin_store::PluginBuilder;
use zksync_web3_rs as zksync;

use zksync::prelude::k256::ecdsa::SigningKey;
use zksync::signers::{Signer, Wallet};

#[derive(Clone, serde::Serialize)]
struct SingleInstancePayload {
  args: Vec<String>,
  cwd: String,
}

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
}

fn get_epoch_ms() -> u128 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
}

#[tauri::command]
async fn message_from_rust(window: tauri::Window) -> Result<CustomResponse, String> {
  println!("Called from {}", window.label());
  Ok(CustomResponse {
    message: format!("Hello from rust!\nTime: {}", get_epoch_ms()),
  })
}

#[tauri::command]
async fn create_zksync_wallet(
  window: tauri::Window,
  ethereumpk: String,
) -> Result<CustomResponse, String> {
  println!("Called from ==>> {} and {:?}", window.label(), ethereumpk);

  let private_key: Wallet<SigningKey> = ethereumpk.parse().unwrap();

  let wallet = Wallet::with_chain_id(private_key, zksync_era_chain_id);

  // initialize your wallet
  println!("{:?}", wallet);
  // Wallet { address: 0x36615cf349d7f6344891b1e7ca7c72883f5dc049, chain_Id: 270 }

  Ok(CustomResponse {
    message: format!("Hello from rust!\nTime: {}", wallet),
  })
}

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);

  // main window should be invisible to allow either the setup delay or the plugin to show the window
  tauri::Builder::default()
    .system_tray(SystemTray::new().with_menu(tray_menu))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { .. } => {
        let window = match app.get_window("main") {
          Some(window) => match window.is_visible().expect("winvis") {
            true => {
              // hide the window instead of closing due to processes not closing memory leak: https://github.com/tauri-apps/wry/issues/590
              window.hide().expect("winhide");
              // window.close().expect("winclose");
              return;
            }
            false => window,
          },
          None => return,
        };
        #[cfg(not(target_os = "macos"))]
        {
          window.show().unwrap();
        }
        window.set_focus().unwrap();
      }
      SystemTrayEvent::RightClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a right click");
      }
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a double click");
      }
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "quit" => {
          std::process::exit(0);
        }
        "hide" => {
          let window = app.get_window("main").unwrap();
          window.hide().unwrap();
        }
        _ => {}
      },
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![message_from_rust])
    .invoke_handler(tauri::generate_handler![create_zksync_wallet])
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      app
        .emit_all(
          "fromOtherInstance",
          SingleInstancePayload { args: argv, cwd },
        )
        .unwrap();
    }))
    /* .plugin(tauri_plugin_window_state::Builder::default().build()) */ // Enable if you want to control the window state
    .plugin(PluginBuilder::default().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
