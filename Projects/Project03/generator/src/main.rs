use std::env;

use generator::{Generator, Operation};

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

    println!("A = {}", result.a());
    println!("B = {:032b}", result.b());
    println!("C = {:016b}", result.c());
    println!("D = {:016b}", result.d());
    println!("CxD");
    println!();

    println!("Step   {: <35}   Next Op.", "Product");
    for (i, s) in result.steps().iter().enumerate() {
        let next_op = match s.next_operation() {
            Operation::Shift => format!(
                "{}{} -> shift",
                s.product().last_bit() as u8,
                s.product().right_bit() as u8
            ),
            Operation::Add => "01 -> add".to_string(),
            Operation::Sub => "10 -> sub".to_string(),
            Operation::Done => "done".to_string(),
        };

        println!(
            " {: >3}   {:016b} {:016b} {}   {}",
            i,
            s.product().high(),
            s.product().low(),
            s.product().right_bit() as u8,
            next_op
        );

        let mask = 0xFFFF;
        match s.next_operation() {
            Operation::Add => {
                let num = result.c() & mask;
                println!("      +{:<016b}", num);
                let added = (s.product().high() + num) & mask;
                println!(
                    "       {:016b} {:016b} {}",
                    added,
                    s.product().low(),
                    s.product().right_bit() as u8
                );
            }
            Operation::Sub => {
                let num = (result.c() * -1) & mask;
                println!("      +{:<016b}", num);
                let added = (s.product().high() + num) & mask;
                println!(
                    "       {:016b} {:016b} {}",
                    added,
                    s.product().low(),
                    s.product().right_bit() as u8
                );
            }

            _ => {}
        }
    }

    let final_product = result.product();

    println!(
        " CxD = {:016b} {:016b}",
        final_product.high(),
        final_product.low()
    );
}
