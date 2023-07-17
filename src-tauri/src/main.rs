// this hides the console for Windows release builds
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use dotenv::dotenv;
use std::env;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde_json::json;
use std::error::Error;

use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager; // used by .get_window
use tauri::{self, SystemTrayEvent, SystemTrayMenuItem};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};
use tauri_plugin_store::PluginBuilder;
use zksync_web3_rs as zksync;

use zksync::prelude::k256::ecdsa::SigningKey;
use zksync::signers::{Signer, Wallet};

use sha2::Sha256;
use hmac::{Hmac, Mac};
use hex_literal::hex;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

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

// 
// zkSYnc Wallet
// 
#[tauri::command]
async fn create_zksync_wallet(
  window: tauri::Window,
  ethereumpk: String,
) -> Result<CustomResponse, String> {
  println!("Called from ==>> {} and {:?}", window.label(), ethereumpk);

  let private_key: Wallet<SigningKey> = ethereumpk.parse().unwrap();
  // Testnet network info
  let zksync_era_chain_id: u64 = 280;

  let wallet = Wallet::with_chain_id(private_key, zksync_era_chain_id);

  // initialize your wallet
  println!("{:?}", wallet);
  // Wallet { address: 0x36615cf349d7f6344891b1e7ca7c72883f5dc049, chain_Id: 270 }

  Ok(CustomResponse {
    message: format!("{}", { wallet.address() }),
  })
}

#[tauri::command]
async fn create_zksync_transfer(
  window: tauri::Window,
  ethereumpk: String,
) -> Result<CustomResponse, String> {
  println!("Called from ==>> {} and {:?}", window.label(), ethereumpk);

  let private_key: Wallet<SigningKey> = ethereumpk.parse().unwrap();
  // Testnet network info
  let zksync_era_chain_id: u64 = 280;

  let wallet = Wallet::with_chain_id(private_key, zksync_era_chain_id);

  // initialize your wallet
  println!("{:?}", wallet);
  // Wallet { address: 0x36615cf349d7f6344891b1e7ca7c72883f5dc049, chain_Id: 270 }

  Ok(CustomResponse {
    message: format!("{}", { wallet.address() }),
  })
}

// 
// KYC DATA: Veriff
// 
#[derive(serde::Serialize)]
struct Data {
  applicant_id: String,
  report_names: Vec<&'static str>,
}

pub async fn create_session() -> Result<serde_json::Value, Box<dyn Error>> {
    dotenv().ok();
    
    let api_token = env::var("TEST_API_TOKEN").expect("TEST_API_TOKEN must be set");
    // TODO: This should be used to create the X-MAC-SIGNATURE
    let verif_token = env::var("TEST_VERIFF_SECRET").expect("TEST_VERIFF_SECRET must be set");
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    println!("create_veriff_session");

    // Set headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("X-AUTH-CLIENT", HeaderValue::from_str(&api_token).unwrap());

    // Set body
    let body = json!({
        "verification": {
            "callback": "https://veriff.com",
            "person": {
                "firstName": "John",
                "lastName": "Smith",
                "idNumber": "123456789"
            },
            "document": {
                "number": "B01234567",
                "type": "PASSPORT",
                "country": "EE"
            },
            "vendorData": "11111111"
        }
    });

    let url = base_url + "/v1/sessions/";

    let resp = Client::new()
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await;

    let resp = resp.unwrap(); // this could panic if resp is an Err
    let json_body: serde_json::Value = resp.json().await.unwrap_or_default();

    Ok(json_body)
}


#[tauri::command]
async fn create_veriff_session() -> Result<CustomResponse, String> {

  let session = create_session();
  let sesion_info = match session.await {
      Ok(custom_response) => format!("{}", custom_response),
      Err(e) => format!("Error: {}", e),
  };

  let stringified = serde_json::to_string(&sesion_info).unwrap();
  
  println!("{}", stringified);
  Ok(CustomResponse {
    message: format!("{}", stringified),
  })
}


pub async fn generate_xhmac_sig(sessiontoken: String, sessionid: String) -> Result<String, Box<dyn Error>> {

  let shared_secret_key = "abcdef12abcd-abcd-abcd-abcdef012345"; // TEST_VERIFF_SECRET
  let payload = "{\"verification\":{\"callback\":\"https://veriff.com\",\"person\":{\"firstName\":\"John\",\"lastName\":\"Smith\"},\"document\":{\"type\":\"PASSPORT\",\"country\":\"EE\"},\"vendorData\":\"unique id of the end-user\",\"timestamp\":\"2016-05-19T08:30:25.597Z\"}}";

  let mut mac = HmacSha256::new_from_slice(shared_secret_key.as_bytes())
    .expect("HMAC can take key of any size");

  mac.update(payload.as_bytes());

  // `result` has type `CtOutput` which is a thin wrapper around array of
  // bytes for providing constant time equality check
  let result = mac.finalize();
  let hash = hex::encode(result.into_bytes());

  let x_hmac_signature = format!("sha256={}", hash);  

  println!("x_hmac_signature: {}", x_hmac_signature);

  Ok(format!("{}", x_hmac_signature))
}

// pub async fn get_decision(sessiontoken: String, sessionid: String) -> Result<String, String> {
//   dotenv().ok();
//   let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
//   let url = format!("{}/v1/sessions/{}/decision", base_url, sessionid);

//   let mut headers = HeaderMap::new();
//   headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
//   headers.insert("X-HMAC-SIGNATURE", HeaderValue::from_str("334141f052e317fde6668de54dc6640b4a5c47582ad86a8bed63afe566f17b14").unwrap());

//   // TODO: This should the the API token generated by the ID
//   // headers.insert("X-AUTH-CLIENT", HeaderValue::from_str(&api_token).unwrap());
//   let resp = Client::new()
//       .get(url)
//       .headers(headers)
//       .send()
//       .await;
  
//   match resp {
//     Ok(resp) => {
//         println!("{:?}", resp);
//         // println!("{:?}", resp.headers());
//     }
//     Err(err) => {
//         println!("Error sending request: {:?}", err);
//         // other code handling the error...
//     }
//   }

//   Ok(format!("generatedProof"))
// }

#[tauri::command]
async fn generate_proof(
  window: tauri::Window,
  erapk: String,
  sessiontoken: String,
  sessionid: String,
) -> Result<CustomResponse, String> {
  println!("generate_proof triggered");

  let sig = generate_xhmac_sig(sessiontoken, sessionid);
  let signature = match sig.await {
    Ok(custom_response) => format!("{}", custom_response),
    Err(e) => format!("Error: {}", e),
  };
  
  // let sig = get_decision();

  Ok(CustomResponse {
    // message: format!("{}", message),
    message: format!("aaa"),
  })
}


// 
// Proof generation: Risc0 Bonsai
// 







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
    .invoke_handler(tauri::generate_handler![create_zksync_wallet, create_zksync_transfer, create_veriff_session, generate_proof])
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
