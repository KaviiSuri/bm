use bm::BM;
use std::{fs::File, process};

const BM_EXECUTION_LIMIT: usize = 69;

static USAGE: &'static str = "Usage: ./bme <input_file>.bm";

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

    let program = BM::deserialize_program_from(input_file);

    let mut bm: BM = Default::default();
    bm.load_program_from_memory(program.as_slice());

    for _ in 0..BM_EXECUTION_LIMIT {
        if bm.is_halted() {
            break;
        }
        if let Err(t) = bm.execute() {
            eprintln!("{}", t);
            bm.dump_stack(&mut std::io::stderr()).expect("should work");
            process::exit(1);
        }
    }
    bm.dump_stack(&mut std::io::stderr()).expect("should work");
}
