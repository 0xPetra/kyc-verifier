// this hides the console for Windows release builds
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager; // used by .get_window
use tauri::{self, SystemTrayEvent, SystemTrayMenuItem};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};
use tauri_plugin_store::PluginBuilder;
use zksync_web3_rs as zksync;

use zksync::prelude::k256::ecdsa::SigningKey;
use zksync::signers::{Signer, Wallet};
use zksync_web3_rs::types::TransactionReceipt;

use hex_literal::hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use json_methods::SEARCH_JSON_ELF;
use risc0_zkvm::sha::Digest;
use risc0_zkvm::{
  default_executor_from_elf,
  serde::{from_slice, to_vec},
  ExecutorEnv,
};

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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
  pub data: u32,
  pub hash: Digest,
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
  let zksync_era_chain_id: u64 = 270;

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
  wallet: String,
  amount: f32,
) -> Result<CustomResponse, String> {
  println!("Called create_zksync_transfer");

  // Create provider
  // let provider = zksync::zks_provider::try_from("http://localhost:3050").unwrap();

  // // Create transfer
  // let sender_address: zksync::Address = wallet.address().parse().unwrap();
  // let receiver_address: zksync::Address = "0xa61464658AfeAf65CccaaFD3a512b69A83B77618".parse().unwrap();
  // let amount_to_transfer = zksync::U256::from(amount);

  // let mut payment_request = zksync::Eip1559TransactionRequest::new()
  //     .from(sender_address)
  //     .to(receiver_address)
  //     .value(amount_to_transfer);

  // let fee = provider
  //     .clone()
  //     .estimate_fee(payment_request.clone())
  //     .await
  //     .unwrap();

  // payment_request = payment_request.max_priority_fee_per_gas(fee.max_priority_fee_per_gas);
  // payment_request = payment_request.max_fee_per_gas(fee.max_fee_per_gas);

  // let transaction: zksync::TypedTransaction = payment_request.into();

  // // Send transaction
  // let signer_middleware = provider.clone().with_signer(wallet);
  // let payment_response: TransactionReceipt =
  //     zksync::SignerMiddleware::send_transaction(&signer_middleware, transaction, None)
  //         .await
  //         .unwrap()
  //         .await
  //         .unwrap()
  //         .unwrap();

  // // initialize your wallet
  // println!("{:?}", wallet);
  // // Wallet { address: 0x36615cf349d7f6344891b1e7ca7c72883f5dc049, chain_Id: 270 }

  Ok(CustomResponse {
    message: format!("Transferred"),
    // message: format!("{}", { wallet.address() }),
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
  let verif_token = env::var("TEST_VERIFF_SECRET").expect("TEST_VERIFF_SECRET must be set"); // TODO: This should be used to create the X-MAC-SIGNATURE
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

pub async fn generate_xhmac_sig(sessionid: &str) -> Result<String, Box<dyn Error>> {
  dotenv().ok();
  let shared_secret_key = env::var("TEST_VERIFF_SECRET").expect("TEST_VERIFF_SECRET must be set"); // This should be used to create the X-MAC-SIGNATURE

  let mut mac = HmacSha256::new_from_slice(shared_secret_key.as_bytes())
    .expect("HMAC can take key of any size");

  mac.update(sessionid.as_bytes());
  // `result` has type `CtOutput` which is a thin wrapper around array of
  // bytes for providing constant time equality check
  let result = mac.finalize();
  let hash = hex::encode(result.into_bytes());

  let x_hmac_signature = format!("sha256={}", hash);

  println!("x_hmac_signature: {}", x_hmac_signature);

  Ok(format!("{}", x_hmac_signature))
}

pub async fn get_decision(
  sessiontoken: String,
  sessionid: &str,
  signature: &str,
) -> Result<String, Box<dyn Error>> {
  println!("get_decision triggered");
  dotenv().ok();
  let api_token = env::var("TEST_API_TOKEN").expect("TEST_API_TOKEN must be set");
  let shared_secret_key = env::var("TEST_VERIFF_SECRET").expect("TEST_VERIFF_SECRET must be set"); // This should be used to create the X-MAC-SIGNATURE
  let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
  let url = format!("{}/v1/sessions/{}/decision", base_url, sessionid);

  let mut headers = HeaderMap::new();
  headers.insert(
    CONTENT_TYPE,
    HeaderValue::from_str("application/json").unwrap(),
  );
  headers.insert(
    "X-HMAC-SIGNATURE",
    HeaderValue::from_str(signature).unwrap(),
  );
  headers.insert("X-AUTH-CLIENT", HeaderValue::from_str(&api_token).unwrap()); // TODO: should the the API token generated by the ID???

  let resp = Client::new()
    .get(&url) // Note that reqwest::get expects a reference to a string
    .headers(headers)
    .send()
    .await;

  match resp {
    Ok(resp) => {
      if resp.status().is_success() {
        // Successful response, handle here
        println!("Response: {:?}", resp);
      } else if resp.status().as_u16() == 401 {
        // The server returned a 401 error, handle here
        eprintln!("Received 401 Unauthorized error");
        // You can print the response body here if it contains more details about the error.
      } else {
        // The server returned another kind of error, handle here
        eprintln!("Received an error from the server: {}", resp.status());
        // You can print the response body here if it contains more details about the error.
      }
    }
    Err(e) => {
      // An error occurred while making the request, handle here
      eprintln!("Error sending request: {}", e);
    }
  }

  // let file = match File::open("mock_data.json") {
  //   Ok(file) => file,
  //   Err(e) => return Err(e.to_string()), // Converts the error to a String
  // };
  // let reader = BufReader::new(file);
  // let output: serde_json::Value = match serde_json::from_reader(reader) {
  //   Ok(data) => data,
  //   Err(e) => return Err(e.to_string()), // Handle the error
  // };
  let output = include_str!("mock_data.json");

  Ok(output.to_string())
}

#[tauri::command]
async fn generate_proof(
  window: tauri::Window,
  wallet: String,
  sessiontoken: String,
  sessionid: String,
) -> Result<CustomResponse, String> {
  println!("generate_proof triggered");

  // Here instead of the paylad we use the sessionid to create the xhmac
  let sig = generate_xhmac_sig(&sessionid);
  let signature = match sig.await {
    Ok(custom_response) => format!("{}", custom_response),
    Err(e) => format!("Error: {}", e),
  };

  let decision = get_decision(sessiontoken, &sessionid, &signature).await;

  // Calculate Response signature:
  let outputs = match decision {
    Ok(value) => {
      let outputs = prove_signature(&value);
      println!();
      println!("  {:?}", outputs.hash);
      println!(
        "provably contains a field 'firstName' with value {}",
        outputs.data
      );
      outputs
    }
    Err(e) => {
      eprintln!("Error occurred: {}", e);
    }
  };

  Ok(CustomResponse {
    // message: format!("{}", message),
    message: format!("aaa"),
  })
}

//
// Proof generation: Risc0 Bonsai
//
fn prove_signature(data: &str) -> Outputs {
  let env = ExecutorEnv::builder()
    .add_input(&to_vec(&data).unwrap())
    .build()
    .unwrap();

  let mut exec = default_executor_from_elf(env, SEARCH_JSON_ELF).unwrap();
  let session = exec.run().unwrap();
  let receipt = session.prove().unwrap();

  from_slice(&receipt.journal).unwrap()
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
    .invoke_handler(tauri::generate_handler![
      create_zksync_wallet,
      create_zksync_transfer,
      create_veriff_session,
      generate_proof
    ])
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
