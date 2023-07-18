// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use hex_literal::hex;
use hmac::{Hmac, Mac};
use json::parse;
use json_core::Outputs;
use risc0_zkvm::{
  guest::env,
  sha::{Impl, Sha256},
};
use sha2::Sha256;

risc0_zkvm::guest::entry!(main);

pub async fn generate_xhmac_sig(content: &str) -> Result<String, Box<dyn Error>> {
  dotenv().ok();
  let shared_secret_key = env::var("TEST_VERIFF_SECRET").expect("TEST_VERIFF_SECRET must be set"); // This should be used to create the X-MAC-SIGNATURE

  let mut mac = HmacSha256::new_from_slice(shared_secret_key.as_bytes())
    .expect("HMAC can take key of any size");

  mac.update(content.as_bytes());
  // `result` has type `CtOutput` which is a thin wrapper around array of
  // bytes for providing constant time equality check
  let result = mac.finalize();
  let hash = hex::encode(result.into_bytes());

  let x_hmac_signature = format!("sha256={}", hash);

  println!("x_hmac_signature: {}", x_hmac_signature);

  Ok(format!("{}", x_hmac_signature))
}

pub fn main() {
  let data: String = env::read();
  let sha = *Impl::hash_bytes(&data.as_bytes());
  let data = parse(&data).unwrap();

  // xhmac provided by veriff.com on the header's response
  let xhmac = data["xhmac"].as_u32().unwrap();

  // payload containing the user's data
  let input = data["payload"].as_string().unwrap();
  let sig = generate_xhmac_sig(&payload);

  // We check integrity and authenticity,
  assert_eq!(xhmac, sig);

  // if the check passes we create the proof making firstName public
  let proven_val = data["payload"]["firstName"].as_u32().unwrap();
  let out = Outputs {
    data: proven_val,
    hash: sha,
  };
  env::commit(&out);
}
