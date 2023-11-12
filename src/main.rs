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

use json_core::Outputs;
use methods::VERIFIABLE_COMPUTE_ENGINE_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
    EncodedPoint,
};
use rand_core::OsRng;
use tokio::runtime::Runtime;
use reqwest::{Client, Error, Response};

fn main() {
    println!("💥 Call Price Oracle Twice");
    price_oracle();
    price_oracle();
    println!("✨✨ Test Pass Successfully ✨✨");
}

fn price_oracle() {
    // Request proxy data server
    // 
    // Quote binance and uniswap price
    let rt = Runtime::new().expect("Failed to create runtime");
    let url = "http://localhost:3000";
    let mut query_params = Vec::new();
    query_params.push([("source".to_string(), "binance".to_string())]);
    query_params.push([("source".to_string(), "uniswap".to_string())]);
    let mut test_payloads = Vec::new();

    for params in &query_params {
        match rt.block_on(get_request(url, params)) {
            Ok(response) => {
                let body = rt.block_on(response.text()).expect("Failed to read response");
                println!("Response: {}", body);
                test_payloads.push(serde_json::to_string(&body).unwrap());
            },
            Err(e) => eprintln!("Request failed: {}: {}", e, params[0].1),
        }
        println!("✨✨ Request Price Feed Success {}", params[0].1);
    }

    // FIXME: remove
    // Generate a random secp256k1 keypair and sign the message.
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let val: u32 = 47;
    let signature: Signature = signing_key.sign(&val.to_be_bytes()); 

    // Execute and Proving
    //
    let payload = include_str!("../res/example.json");
    verifiable_compute_engine(payload, signing_key.verifying_key(), &signature);
    println!("✨ Price Oracle Execution Success");
}

async fn get_request(url: &str, query: &[(String, String)]) -> Result<Response, Error> {
    let client = Client::new();
    let response = client.get(url)
        .query(query)
        .send()
        .await?;

    Ok(response)
}

// Execute guest functions and proving
fn verifiable_compute_engine(
    payload: &str,
    verifying_key: &VerifyingKey,
    signature: &Signature,
) {
    println!("...Start Verifiable Compute Engine");
    let input = (payload, verifying_key.to_encoded_point(true), signature);
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, VERIFIABLE_COMPUTE_ENGINE_ELF).unwrap();
    println!("✨ Proving Success");
    receipt.journal.decode().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let data = include_str!("../res/example.json");
        let outputs = super::search_json(data);
        assert_eq!(
            outputs.data, 47,
            "Did not find the expected value in the critical_data field"
        );
    }
}
