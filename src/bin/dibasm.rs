use std::fs::File;

use bm::BM;

static USAGE: &'static str = "Usage: ./bm <input_file>.basm";

fn main() {
    let mut args = std::env::args();
    args.next().expect("Should work");
    let input_file = File::options()
        .read(true)
        .open(
            args.next()
                .expect(format!("Expected Input File: \n{}", USAGE).as_str()),
        )
        .expect("Could not read input file.");

    let program = BM::deserialize_program_from(&input_file);
    let mut bm: BM = Default::default();
    bm.load_program_from_memory(&program);
    bm.program_to_asm(&mut std::io::stdout())
        .expect("Could not serialize basm");
}
