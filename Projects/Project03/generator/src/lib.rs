use std::fmt;

pub struct Generator<'a> {
    ntust_id: &'a str,
}

impl<'a> Generator<'a> {
    pub fn new(ntust_id: &'a str) -> Self {
        Self { ntust_id }
    }

    pub fn ntust_id(&self) -> &str {
        self.ntust_id
    }

    pub fn generate(&self) -> anyhow::Result<GenerateResult> {
        let a = String::from(&self.ntust_id[1..]);
        let a: i64 = a.parse()?;
        let b = a.pow(2);
        let c = (b >> 16) & 0b111111111111111;
        let d = ((b >> 16) & 0b111111111111111) | 0b1000000000000000;

        let mut steps = Vec::new();

        let mut product = Product::new((d << 1).into());

        for _ in 0..16 {
            if (!product.last_bit() && !product.right_bit())
                || (product.last_bit() && product.right_bit())
            {
                // 00/11 -> shift
                steps.push(Step {
                    product: product.clone(),
                    next_operation: Operation::Shift,
                });
            } else if !product.last_bit() && product.right_bit() {
                // 01 -> add
                steps.push(Step {
                    product: product.clone(),
                    next_operation: Operation::Add,
                });

                product.add(c);
            } else if product.last_bit() && !product.right_bit() {
                // 10 -> sub
                steps.push(Step {
                    product: product.clone(),
                    next_operation: Operation::Sub,
                });

                product.sub(c);
            }

            // Final shift
            product.shift();
        }

        // done
        steps.push(Step {
            product: product.clone(),
            next_operation: Operation::Done,
        });

        Ok(GenerateResult {
            a,
            b,
            c,
            d,
            steps,
            product,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Shift,
    Add,
    Sub,
    Done,
}

#[derive(Clone, Debug)]
pub struct Step {
    product: Product,
    next_operation: Operation,
}

impl Step {
    pub fn product(&self) -> &Product {
        &self.product
    }

    pub fn next_operation(&self) -> Operation {
        self.next_operation
    }
}

pub struct GenerateResult {
    a: i64,
    b: i64,
    c: i64,
    d: i64,

    steps: Vec<Step>,
    product: Product,
}

impl GenerateResult {
    pub fn a(&self) -> i64 {
        self.a
    }

    pub fn b(&self) -> i64 {
        self.b
    }

    pub fn c(&self) -> i64 {
        self.c
    }

    pub fn d(&self) -> i64 {
        self.d
    }

    pub fn steps(&self) -> Vec<Step> {
        self.steps.clone()
    }

    pub fn product(&self) -> Product {
        self.product.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Product {
    number: i64,
}

impl Product {
    const MAX: i64 = 0b111111111111111111111111111111111;

    pub fn new(number: i64) -> Self {
        Self { number }
    }

    pub fn number(&self) -> i64 {
        self.number
    }

    pub fn high(&self) -> i64 {
        (((self.number >> 1) >> 16) & 0xFFFF).try_into().unwrap()
    }

    pub fn low(&self) -> i64 {
        ((self.number >> 1) & 0xFFFF).try_into().unwrap()
    }

    /// 0000000000000000 1000111110001100 0
    ///                                 ^
    pub fn last_bit(&self) -> bool {
        if (self.number >> 1) & 1 == 1 {
            true
        } else {
            false
        }
    }

    /// 0000000000000000 1000111110001100 0
    ///                                   ^
    pub fn right_bit(&self) -> bool {
        if self.number & 1 == 1 {
            true
        } else {
            false
        }
    }

    pub fn shift(&mut self) {
        let shifted = (self.number >> 1) & Self::MAX;

        let padding = 33 - 1;

        let mask = 1 << (63 - padding);
        let should_pad = (shifted & mask) != 0;

        self.number = if should_pad {
            shifted | (1 << (padding))
        } else {
            shifted
        };
    }

    pub fn add(&mut self, n: i64) {
        let tmp: i64 = ((n << 16) << 1) & Self::MAX;
        self.number = (self.number + tmp) & Self::MAX;
    }

    pub fn sub(&mut self, n: i64) {
        self.add(n * -1);
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.number)
    }
}

impl Into<i64> for Product {
    fn into(self) -> i64 {
        self.number
    }
}
