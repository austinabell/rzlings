use risc0_zkvm::guest::env;

fn main() {
    env::commit_slice(b"hello world");
}
