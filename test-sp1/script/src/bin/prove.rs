use clap::Parser;
use groth16::has_json_repr::load_json;
use groth16::has_primitive_repr::HasPrimitiveRepr;
use groth16::{Inputs, Proof, VerifyingKey};
use sp1_sdk::{ProverClient, SP1Stdin};
use std::time::Instant;

pub const G16_AGGREGATION_ELF: &[u8] =
    include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

const SAMPLE_INPUTS_FILE: &str = "../../groth16/src/data/inputs.json";
const SAMPLE_PROOF_FILE: &str = "../../groth16/src/data/proof.json";
const SAMPLE_VK_FILE: &str = "../../groth16/src/data/vk.json";

/// The arguments for the prove command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ProveArgs {
    /// Batch size
    #[clap(long, default_value = "1")]
    n: u32,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Read input from JSON
    let inputs: Inputs = load_json(SAMPLE_INPUTS_FILE);
    let proof: Proof = load_json(SAMPLE_PROOF_FILE);
    let groth16_vk: VerifyingKey = load_json(SAMPLE_VK_FILE);

    // Parse batch size from command line
    let args = ProveArgs::parse();
    let batch_size: u32 = args.n;
    println!("Batch size: {batch_size}");

    // Write the batch size, inputs, proof, and vk to stdin.
    let mut stdin = SP1Stdin::new();
    stdin.write(&batch_size);
    stdin.write(&inputs.to_repr());
    stdin.write(&proof.to_repr());
    stdin.write(&groth16_vk.to_repr());
    println!("Public input length: {}", inputs.len());

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the program.
    let (pk, vk) = client.setup(G16_AGGREGATION_ELF);

    // Generate the proof.
    let now = Instant::now();
    let proof = client
        .prove(&pk, stdin)
        .plonk()
        .run()
        .expect("failed to generate proof");
    println!("Proof generation time: {}s", now.elapsed().as_secs());

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
}
