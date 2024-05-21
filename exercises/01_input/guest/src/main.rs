use risc0_zkvm::guest::env;
use std::io::Read;

// DO NOT CHANGE THE GUEST TO MAKE IT PASS.
fn main() {
    let mut buf = Vec::new();
    env::stdin().read_to_end(&mut buf).unwrap();
    assert!(
        !buf.is_empty(),
        "Guest received no input. Try setting the input on `ExecutorEnv`"
    );
}
