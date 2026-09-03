use std::env;
use avro_core::parser::EnglishToBangla;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: avro-cli <word1> [word2 ...]");
        std::process::exit(1);
    }

    let mut parser = EnglishToBangla::new();
    parser.auto_correct = false; // match pascal CLI

    for i in 1..args.len() {
        let input_word = &args[i];
        match parser.convert(input_word) {
            Ok(output_word) => println!("{}", output_word),
            Err(e) => eprintln!("Error converting {}: {}", input_word, e),
        }
    }
}
