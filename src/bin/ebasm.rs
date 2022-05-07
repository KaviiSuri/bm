use bm::BM;
use std::fs::File;

static USAGE: &'static str = "Usage: ./ebasm <input_file>.ebasm <output_file>.bm";

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

    let program = BM::from_asm(&input_file);

    let output_file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(
            args.next()
                .expect(format!("Expected Output File: \n{}", USAGE).as_str()),
        )
        .expect("Could not open or create output file");

    BM::serialize_program_into(&output_file, &program);
}