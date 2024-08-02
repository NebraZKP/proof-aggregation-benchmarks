use groth16::has_primitive_repr::HasPrimitiveRepr;
use groth16::{verify, Inputs, Proof, VerifyingKey};
use risc0_zkvm::guest::env;

fn main() {
    // read the input
    let batch_size: u32 = env::read();
    let inputs_repr: <Inputs as HasPrimitiveRepr>::Repr = env::read();
    let proof_repr: <Proof as HasPrimitiveRepr>::Repr = env::read();
    let vk_repr: <VerifyingKey as HasPrimitiveRepr>::Repr = env::read();

    let inputs = Inputs::from_repr(&inputs_repr);
    let proof = Proof::from_repr(&proof_repr);
    let vk = VerifyingKey::from_repr(&vk_repr);

    for _ in 0..batch_size {
        let result = verify(&vk, &proof, &inputs);
        assert!(result.is_ok());
    }
}
