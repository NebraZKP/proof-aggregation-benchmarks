#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use groth16::has_primitive_repr::HasPrimitiveRepr;
use groth16::{verify, Inputs, Proof, VerifyingKey};

#[jolt::provable]
fn aggregate_g16(
    batch_size: u32,
    inputs: <Inputs as HasPrimitiveRepr>::Repr,
    proof: <Proof as HasPrimitiveRepr>::Repr,
    vk: <VerifyingKey as HasPrimitiveRepr>::Repr,
) -> () {
    let inputs = Inputs::from_repr(&inputs);
    let proof = Proof::from_repr(&proof);
    let vk = VerifyingKey::from_repr(&vk);

    for _ in 0..batch_size {
        let result = verify(&vk, &proof, &inputs);
        assert!(result.is_ok());
    }
}
