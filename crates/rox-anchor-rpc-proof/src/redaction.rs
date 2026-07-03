//! RO:WHAT — Redaction helpers for local RPC proof reports.
//! RO:WHY — Keeps long signatures readable without dumping full values into status output.
//! RO:INTERACTS — future CLI/coordinator report rendering.
//! RO:INVARIANTS — redaction changes display only, never review decisions.
//! RO:SECURITY — no secrets are required or emitted by local review.
//! RO:TEST — covered by redaction unit tests.

pub fn redact_signature(signature: &str) -> String {
    let chars = signature.chars().collect::<Vec<_>>();

    if chars.len() <= 12 {
        return format!("short:{}", chars.len());
    }

    let prefix = chars.iter().take(8).collect::<String>();
    let suffix = chars.iter().skip(chars.len() - 4).collect::<String>();

    format!("{prefix}...{suffix}")
}
