use crate::{GenerateResult, Operation};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl GenerateResult {
    pub fn to_plain_text(&self) -> String {
        let mut text = String::new();

        text.push_str(&format!("A = {}\n", self.a()));
        text.push_str(&format!("B = {:032b}\n", self.b()));
        text.push_str(&format!("C = {:016b}\n", self.c()));
        text.push_str(&format!("D = {:016b}\n", self.d()));
        text.push_str(&format!("CxD\n"));
        text.push_str(&format!("\n"));

        text.push_str(&format!("Step   {: <35}   Next Op.\n", "Product"));
        for (i, s) in self.steps().iter().enumerate() {
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

            text.push_str(&format!(
                " {: >3}   {:016b} {:016b} {}   {}\n",
                i,
                s.product().high(),
                s.product().low(),
                s.product().right_bit() as u8,
                next_op
            ));

            let mask = 0xFFFF;
            match s.next_operation() {
                Operation::Add => {
                    let num = self.c() & mask;
                    text.push_str(&format!("      +{:<016b}\n", num));
                    let added = (s.product().high() + num) & mask;
                    text.push_str(&format!(
                        "       {:016b} {:016b} {}\n",
                        added,
                        s.product().low(),
                        s.product().right_bit() as u8
                    ));
                }
                Operation::Sub => {
                    let num = (self.c() * -1) & mask;
                    text.push_str(&format!("      +{:<016b}\n", num));
                    let added = (s.product().high() + num) & mask;
                    text.push_str(&format!(
                        "       {:016b} {:016b} {}\n",
                        added,
                        s.product().low(),
                        s.product().right_bit() as u8
                    ));
                }

                _ => {}
            }
        }

        let final_product = self.product();

        text.push_str(&format!(
            " CxD = {:016b} {:016b}\n",
            final_product.high(),
            final_product.low()
        ));

        text
    }
}
