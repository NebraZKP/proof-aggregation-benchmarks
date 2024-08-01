use sp1_sdk::{HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};

pub const G16_AGGREGATION_ELF: &[u8] =
    include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the program.
    let (pk, vk) = client.setup(G16_AGGREGATION_ELF);

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();

    // Generate the proof.
    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
}
