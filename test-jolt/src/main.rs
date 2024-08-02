use clap::Parser;
use groth16::has_json_repr::load_json;
use groth16::has_primitive_repr::HasPrimitiveRepr;
use groth16::{Inputs, Proof, VerifyingKey};
use std::time::Instant;

const SAMPLE_INPUTS_FILE: &str = "../groth16/src/data/inputs.json";
const SAMPLE_PROOF_FILE: &str = "../groth16/src/data/proof.json";
const SAMPLE_VK_FILE: &str = "../groth16/src/data/vk.json";

/// The arguments for the prove command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ProveArgs {
    /// Batch size
    #[clap(long, default_value = "1")]
    n: u32,
}

pub fn main() {
    println!("ZKVM: Jolt with standard precompiles");

    // Read input from JSON
    let inputs: Inputs = load_json(SAMPLE_INPUTS_FILE);
    let proof: Proof = load_json(SAMPLE_PROOF_FILE);
    let groth16_vk: VerifyingKey = load_json(SAMPLE_VK_FILE);

    // Parse batch size from command line
    let args = ProveArgs::parse();
    let batch_size: u32 = args.n;
    println!("Batch size: {batch_size}");

    let (prove_agg, verify_agg) = guest::build_aggregate_g16();

    let now = Instant::now();
    let (_output, proof) = prove_agg(
        batch_size,
        inputs.to_repr(),
        proof.to_repr(),
        groth16_vk.to_repr(),
    );
    println!("Proof generation time: {}s", now.elapsed().as_secs());

    let is_valid = verify_agg(proof);

    println!("valid: {}", is_valid);
}
