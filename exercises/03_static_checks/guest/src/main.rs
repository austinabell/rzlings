use risc0_zkvm::guest::env;

fn main() {
    let input: u32 = env::read();

    // TODO modify program to only allow values above 10 to pass.

    env::commit(&input);
}
