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

fn main() {
    // FIXME: remove
    // Generate a random secp256k1 keypair and sign the message.
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let val: u32 = 47;
    let signature: Signature = signing_key.sign(&val.to_be_bytes()); 

    let payload = include_str!("../res/example.json");
    verifiable_compute_engine(payload, signing_key.verifying_key(), &signature);
    println!("test pass");
}

// Execute guest functions and proving
fn verifiable_compute_engine(
    payload: &str,
    verifying_key: &VerifyingKey,
    signature: &Signature,
) {
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
