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


// use std::io::Read;
// use json_core::Outputs;
/* use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
}; */
// use ethabi::{ethereum_types::U256, ParamType, Token};
use json::parse;
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);
use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};

fn main() {
    /* // Read data sent from the application contract.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    // Type array passed to `ethabi::decode_whole` should match the types encoded in
    // the application contract.
    let input: Vec<Token> = ethabi::decode_whole(&[ParamType::Uint(256)], &input_bytes).unwrap();
    let n: U256 = input[0].clone().into_uint().unwrap();

    // Run the computation.
    // Encoded types should match the args expected by the application callback.
    env::commit_slice(&ethabi::encode(&[Token::Uint(n), Token::Uint(result)])); */

     
    // Parsing Data 
    //
    // Parse the price feed JSON data, and decode the verifying key, message, and signature from the inputs.
    let payloads: Vec<(String, EncodedPoint)> = env::read();
    let mut prices = Vec::<f64>::new();
    payloads.iter().for_each(|(
        response,  
        verifyingkey
    )| {
        let verifying_key = VerifyingKey::from_encoded_point(&verifyingkey).unwrap();

        let data = parse(&response).unwrap();
        let price_literal = data["price"].as_str().unwrap().as_bytes();
        let price = data["price"].as_f64().unwrap(); // FIXME: check signing method
        let signature_byte = data["sig"].as_u32().unwrap().to_le_bytes();
        let signature = Signature::from_slice(&signature_byte).unwrap();
        
        // verification 
        //
        // verify signature with price feed from binance, panicking if verification fails.
        verifying_key
            .verify(&price_literal, &signature)
            .expect("ECDSA signature verification failed");
        prices.push(price);
    });

    // Commit to the journal the verifying key and message that was signed.
    // env::commit(&(encoded_verifying_key, &price_val_le_byte));

    // verify the signature along with the price feed
    // verify price feed from Uniswap
    /* let out = Outputs {
        data: price_val,
        timestamp: 0,
    }; */
}
