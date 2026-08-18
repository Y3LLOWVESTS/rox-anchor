//! RO:WHAT — Binary entry point for local ROX Anchor inspection.
//! RO:WHY — Dispatches terminal commands into real proof/core behavior.
//! RO:INTERACTS — rox_anchor_cli::run_from_args.
//! RO:INVARIANTS — no transaction submission or value movement; Phase 4 live simulation is explicit and non-broadcasting.
//! RO:SECURITY — never deploys, mints, burns, settles, stakes, or submits transactions; explicit Phase 4 simulation may perform RPC reads and simulation.
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
