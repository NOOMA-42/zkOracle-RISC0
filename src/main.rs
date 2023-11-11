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
use methods::SEARCH_JSON_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
    EncodedPoint,
};
use rand_core::OsRng;

fn main() {
    /* let data = include_str!("../res/example.json");
    let outputs = search_json(data);
    /* println!();
    println!("  {:?}", outputs.hash);
    println!(
        "provably contains a field 'critical_data' with value {}",
        outputs.data
    ); */ */

    ecdsa();
    println!(" tesst");
}

fn ecdsa() {
    // Generate a random secp256k1 keypair and sign the message.
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = b"This is a message that will be signed, and verified within the zkVM";
    let signature: Signature = signing_key.sign(message);
    
    
    let input = (signing_key.verifying_key().to_encoded_point(true), message, &signature);
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    prover.prove_elf(env, SEARCH_JSON_ELF).unwrap();
}

fn search_json(data: &str) { //  -> Outputs
    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .build()
        .unwrap();

        /* 
        .write(&data)
        .unwrap()
         */
    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, SEARCH_JSON_ELF).unwrap();

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
