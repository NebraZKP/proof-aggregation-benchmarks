use nexus_sdk::{
    compile::CompileOpts,
    nova::seq::{Generate, Nova, PP},
    Local, Prover, Verifiable,
};
use std::time::Instant;

const PACKAGE: &str = "guest";

fn main() {
    println!("Setting up Nova public parameters...");
    let pp: PP = PP::generate().expect("failed to generate parameters");

    let mut opts = CompileOpts::new(PACKAGE);
    opts.set_memlimit(8); // use an 8mb memory

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    let input: String = "42".to_string();
    println!("Proving execution of vm...");
    let now = Instant::now();
    let proof = prover.prove_with_input(&pp, &input).expect("failed to prove program");
    println!("Proof generation took: {:?}s", now.elapsed().as_secs());

    println!(">>>>> Logging\n{}<<<<<", proof.logs().join(""));

    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");

    println!("  Succeeded!");
}
