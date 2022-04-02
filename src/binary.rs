#[derive(PartialEq, Eq, Debug)]
enum Sign {
    Neg,
    Pos,
    Zero,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Zequal {
    exponent: i8,
    sign: Sign,
}

impl Zequal {
    /// `Zequal` representation of `0`.
    pub fn new() -> Self {
        Self {
            exponent: 0,
            sign: Sign::Zero,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::proptest;

    #[test]
    fn test_new() {
        assert_eq!(
            Zequal::new(),
            Zequal {
                exponent: 0,
                sign: Sign::Zero
            }
        )
    }

