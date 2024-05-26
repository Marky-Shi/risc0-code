// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use core::Outputs;
use methods::{BEVY_ELF, BEVY_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let turn = 3u32;

    let env = ExecutorEnv::builder()
        .write(&turn)
        .unwrap()
        .build()
        .unwrap();

    // prover
    let prover = default_prover();

    // receipt
    let receipt = prover.prove(env, BEVY_ELF).unwrap();

    // verify
    receipt.verify(BEVY_ID).unwrap();

    // outputs
    let outputs: Outputs = receipt
        .journal
        .decode()
        .expect("Journal should contain an outputs object");

    assert_eq!(outputs.position, turn as f32);

    println!(
        "Game state provably moved primary entity by {} units on the x-axis",
        outputs.position
    );
}
