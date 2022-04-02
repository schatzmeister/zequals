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

/// Extract the exponent part from a number to create a Zequal number
trait Exponent {
    fn exponent(&self) -> i8;
}

impl Exponent for u128 {
    fn exponent(&self) -> i8 {
        let bits = 127;
        bits - self.leading_zeros() as i8
    }
}

impl From<u128> for Zequal {
    fn from(value: u128) -> Self {
        if value == 0 {
            Self {
                exponent: 0,
                sign: Sign::Zero,
            }
        } else {
            Self {
                // If there is no leading zero, we have 2^127,
                // otherwise, each leading zero halves the number.
                exponent: value.exponent(),
                sign: Sign::Pos,
            }
        }
    }
}

impl From<Zequal> for u128 {
    fn from(value: Zequal) -> Self {
        if value.sign == Sign::Zero {
            0
        } else {
            1 << value.exponent
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

    #[test]
    fn test_zequal_from_0() {
        let z = Zequal::from(0u128);
        assert_eq!(
            z,
            Zequal {
                exponent: 0,
                sign: Sign::Zero
            }
        );
        assert_eq!(0u128, z.into());
    }

    #[test]
    fn test_zequal_from_1() {
        let z = Zequal::from(1u128);
        assert_eq!(
            z,
            Zequal {
                exponent: 0,
                sign: Sign::Pos
            }
        );
        assert_eq!(1u128, z.into());
    }

    #[test]
    fn test_zequal_from_2() {
        let z = Zequal::from(2u128);
        assert_eq!(
            z,
            Zequal {
                exponent: 1,
                sign: Sign::Pos
            }
        );
        assert_eq!(2u128, z.into());
    }

    proptest! {
        #[test]
        // Convert 2**n from u128 to Zequals and back to u128.
        fn test_convert_id_powers_of_two(num in 0..127) {
            let power: u128 = 1u128<<num;
            assert_eq!(u128::from(Zequal::from(power)), power)
        }

        // Convert any number to Zequals and back, and check that the most significant bit
        // is the same
        fn test_convert_id_positive(num: u128) {
            assert_eq!(u128::from(Zequal::from(num)).leading_zeros(), num.leading_zeros())
        }

    }
}
