//! Naive Groth16 Aggregation using SP1
#![no_main]
sp1_zkvm::entrypoint!(main);

use groth16::has_primitive_repr::HasPrimitiveRepr;
use groth16::{verify, Inputs, Proof, VerifyingKey};

pub fn main() {
    let batch_size: u32 = sp1_zkvm::io::read();

    let inputs_repr: <Inputs as HasPrimitiveRepr>::Repr = sp1_zkvm::io::read();
    let proof_repr: <Proof as HasPrimitiveRepr>::Repr = sp1_zkvm::io::read();
    let vk_repr: <VerifyingKey as HasPrimitiveRepr>::Repr = sp1_zkvm::io::read();

    let inputs = Inputs::from_repr(&inputs_repr);
    let proof = Proof::from_repr(&proof_repr);
    let vk = VerifyingKey::from_repr(&vk_repr);

    // For simplicity, we simulate verifying a batch of proofs by repeatedly
    // verifying one proof.
    for _ in 0..batch_size {
        let result = verify(&vk, &proof, &inputs);
        assert!(result.is_ok());
    }
}
