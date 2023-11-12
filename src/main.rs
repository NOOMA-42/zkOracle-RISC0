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

/*  
(reponse body: String, verifying key: EncodedPoint)
where response body is {"price":, "time":, "sig":}
Host directly send the api response to the guest in preventing potential manipulation
*/
type Payload = (String, EncodedPoint);

fn main() {
    println!("💥 Call Price Oracle Twice");
    price_oracle();
    price_oracle();
    println!("✨✨ Test Pass Successfully ✨✨");
}

fn price_oracle() {
    // Get signing keys
    //
    // TODO Consider proxy expose the signing keys route
    let mut signing_keys = Vec::new();
    signing_keys.push(("btcturk".to_string(), SigningKey::random(&mut OsRng))); // FIXEME 
    signing_keys.push(("uniswap".to_string(), SigningKey::random(&mut OsRng)));
    
    
    // Request proxy data server
    // 
    // Quote binance and uniswap price
    let rt = Runtime::new().expect("Failed to create runtime");
    let url = "http://localhost:3000";
    let mut query_params = Vec::new();
    query_params.push([("source".to_string(), "btcturk".to_string())]);
    query_params.push([("source".to_string(), "uniswap".to_string())]);
    let mut payloads: Vec<Payload> = Vec::new();

    // Send request to corresponding data provider and fill payload
    for params in &query_params {
        let provider_signing_key = &signing_keys
            .iter()
            .find(|(provider, _)| provider == &params[0].1)
            .expect("Provider not found")
            .1;
        let mut payload: Payload = (
            "".to_string(), // Fill in response body later
            provider_signing_key.verifying_key().to_encoded_point(true),
        );

        // Request price feed
        match rt.block_on(get_request(url, params)) {
            Ok(response) => {
                let body = rt.block_on(response.text()).expect("Failed to read response");
                println!("Response: {}", body);
                payload.0 = serde_json::to_string(&body).unwrap();
                payloads.push(payload);
                println!("✨✨ Request Price Feed Success {}", params[0].1);
            },
            Err(e) => eprintln!("Request failed: {}: {}", e, params[0].1),
        }
    }

    // Execute and Proving
    //
    verifiable_compute_engine(payloads);
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
    payloads: Vec<Payload>,
) {
    println!("...Start Verifiable Compute Engine");
    let env = ExecutorEnv::builder()
        .write(&payloads)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    // let receipt = 
    prover.prove_elf(env, VERIFIABLE_COMPUTE_ENGINE_ELF);
    println!("✨ Proving Success");
    // receipt.journal.decode().unwrap()
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
