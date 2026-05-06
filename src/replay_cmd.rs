//! CLI command handlers for replay guard inspection and management.

use crate::replay::ReplayGuard;

/// Displays the current state of the replay guard.
pub fn cmd_replay_status(guard: &ReplayGuard) {
    println!("Replay Guard Status");
    println!("  Active nonces tracked : {}", guard.active_count());
}

/// Registers a nonce via CLI and reports success or failure.
pub fn cmd_replay_register(guard: &mut ReplayGuard, nonce: &str) {
    match guard.check_and_register(nonce) {
        Ok(()) => println!("Nonce '{}' registered successfully.", nonce),
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// Forces a purge of expired nonces and reports how many remain.
pub fn cmd_replay_purge(guard: &mut ReplayGuard) {
    guard.purge_expired();
    println!(
        "Purge complete. {} nonce(s) still active.",
        guard.active_count()
    );
}
