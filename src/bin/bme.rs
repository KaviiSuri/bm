use bm::BM;
use std::{fs::File, process};

static USAGE: &'static str = "Usage: ./bme <input_file>.bm [-l <limit>]";

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

    // parse limtt
    let limit = match args.next() {
        Some(l) if l == "-l" => Some(
            args.next()
                .expect(format!("Expected a limit after -l\n {}", USAGE).as_str())
                .parse::<usize>()
                .expect(format!("limit must be an usigned integer\n {}", USAGE).as_str()),
        ),
        Some(l) => panic!("Unknown flag {}\n{}", l, USAGE),
        None => None,
    };

    let program = BM::deserialize_program_from(input_file);
    let mut bm: BM = Default::default();
    bm.load_program_from_memory(program.as_slice());
    match bm.execute_program(limit) {
        Ok(()) => bm.dump_stack(&mut std::io::stdout()).expect("should work"),
        Err(e) => {
            eprintln!("{}", e);
            bm.dump_stack(&mut std::io::stderr()).expect("should work");
            process::exit(1);
        }
    };
}
