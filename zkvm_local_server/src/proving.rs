use nexus_sdk::{
    stwo::seq::{Proof, Stwo},
    KnownExitCodes, Local, Prover, Verifiable, Viewable,
};
use postcard::{from_bytes, to_allocvec};
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

const GUEST_SOURCE_PATH: &str = "../guest/src/main.rs";
const GUEST_ELF_PATH: &str = "../guest/target/riscv32i-unknown-none-elf/release/guest";
const PROOF_PATH: &str = "./proofs/proof";

// Generate nexus zkvm proof for a given rust snippet.
pub fn prove_query(snippet: &str) {
    // Format guest file properly.
    let header = r#"
#![cfg_attr(target_arch = "riscv32", no_std, no_main)]
#[nexus_rt::main]
fn main() {"#;
    let footer = r#"}"#;

    // Overwrite the guest file.
    let mut file = File::create(GUEST_SOURCE_PATH).expect("Failed to create file");
    file.write_all(header.as_bytes())
        .expect("Failed to write header");
    file.write_all(snippet.as_bytes())
        .expect("Failed to write main function logic");
    file.write_all(footer.as_bytes())
        .expect("Failed to write footer");

    // Build the guest program.
    println!("Building guest program...");
    let _output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("riscv32i-unknown-none-elf")
        .current_dir("../guest")
        .output()
        .expect("Failed to build guest program");

    println!("Proving guest program...");
    let prover: Stwo<Local> = Stwo::new_from_file(GUEST_ELF_PATH).expect("failed to load program");
    let (view, proof) = prover.prove().expect("failed to prove program");

    assert_eq!(
        view.exit_code().expect("failed to retrieve exit code"),
        nexus_sdk::KnownExitCodes::ExitSuccess as u32
    );

    println!("Finished proving guest program! ✅");

    // Save proof to file.
    if let Some(parent_dir) = std::path::Path::new(PROOF_PATH).parent() {
        fs::create_dir_all(parent_dir).expect("Failed to create proofs directory");
    }
    let proof_bytes = postcard::to_allocvec(&proof).expect("Failed to serialize proof");
    fs::write(&PROOF_PATH, &proof_bytes).expect("Failed to write proof to file");
}

// Verifies nexus zkvm proof.
pub fn verify_proof() {
    // Load proof from file.
    let proof_bytes = fs::read(&PROOF_PATH).expect("Failed to read proof from file");
    let proof: Proof = postcard::from_bytes(&proof_bytes).expect("Failed to deserialize proof");
    let prover_for_elf: Stwo<Local> = Stwo::new_from_file(GUEST_ELF_PATH)
        .expect("Failed to load program to get ELF for verification");
    let elf_bytes = prover_for_elf.elf;

    println!("Verifying proof...");
    proof
        .verify_expected::<(), ()>(
            &(),
            KnownExitCodes::ExitSuccess as u32,
            &(),
            &elf_bytes,
            &[],
        )
        .expect("Failed to verify proof");
    println!("Proof verified successfully! ✅");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prove() {
        let snippet = r#"
            use nexus_rt::print;
            let x = 5;
            let y = 7;
            // let z = x + y;
            // println!("z is {}", z);
            // let xx = 10;
            // let yy = 11;
            // let zz = xx + yy;
            // println!("zz is {}", zz);
            // assert!(z == 12);
            // assert!(zz == 21);
            // println!("z is {}", z);
        "#;

        prove_query(snippet);
        verify_proof();
    }
}
