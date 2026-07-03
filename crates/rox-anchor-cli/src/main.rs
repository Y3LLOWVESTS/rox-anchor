//! RO:WHAT — Binary entry point for local ROX Anchor inspection.
//! RO:WHY — Dispatches terminal commands into real proof/core behavior.
//! RO:INTERACTS — rox_anchor_cli::run_from_args.
//! RO:INVARIANTS — local inspection only; no network, wallet, submission, or value movement.
//! RO:SECURITY — does not deploy, mint, burn, settle, stake, or submit live RPC transactions.
//! RO:TEST — cargo run -p rox-anchor-cli -- check.

#![forbid(unsafe_code)]

fn main() {
    match rox_anchor_cli::run_from_args(std::env::args()) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(2);
        }
    }
}
