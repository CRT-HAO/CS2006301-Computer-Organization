use std::env;

use generator::Generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    // exit if ntustu_id not set
    if args.len() < 2 {
        println!("Usage: {} <ntust_id>", args[0]);
        return;
    }

    let ntust_id = &args[1];

    let generator = Generator::new(&ntust_id);
    let result = generator.generate().expect("generate result failed");

    print!("{}", result.to_plain_text())
}
