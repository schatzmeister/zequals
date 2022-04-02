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

impl Exponent for i128 {
    fn exponent(&self) -> i8 {
        // The maximal exponent is 126, but we have to account for
        // the sign bit.
        let bits = 127;
        bits - self.abs().leading_zeros() as i8
    }
}

/// Trait for converting a number into its zequalized form.
trait Zequals {
    /// Perform the operation.
    fn zequals(self) -> Self;
}

impl<T> Zequals for T
where
    Zequal: From<T> + Into<T>,
{
    fn zequals(self) -> Self {
        Zequal::from(self).into()
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

impl From<i128> for Zequal {
    fn from(value: i128) -> Self {
        if value == 0 {
            Self {
                exponent: 0,
                sign: Sign::Zero,
            }
        } else if value > 0 {
            Self {
                exponent: value.exponent(),
                sign: Sign::Pos,
            }
        } else {
            Self {
                exponent: value.exponent(),
                sign: Sign::Neg,
            }
        }
    }
}

impl From<Zequal> for i128 {
    fn from(value: Zequal) -> Self {
        match value.sign {
            Sign::Neg => -(1 << value.exponent),
            Sign::Pos => 1 << value.exponent,
            Sign::Zero => 0,
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

    #[test]
    fn test_zequals_of_0u128() {
        assert_eq!(0.zequals(), 0u128);
    }

    #[test]
    fn test_zequals_of_0i128() {
        assert_eq!(0.zequals(), 0i128);
    }

    proptest! {
        #[test]
        // Convert 2**n from u128 to Zequals and back to u128.
        fn test_convert_id_powers_of_two(num in 0..127) {
            let power: u128 = 1u128<<num;
            assert_eq!(u128::from(Zequal::from(power)), power)
        }

        #[test]
        // Convert 2**n from i128 to Zequals and back to u128.
        fn test_convert_id_powers_of_two_i128(num in 0..127) {
            let power: i128 = dbg!(1i128<<dbg!(num));
            assert_eq!(i128::from(Zequal::from(power)), power)
        }

        #[test]
        // Convert any number to Zequals and back, and check that the most significant bit
        // is the same
        fn test_convert_id_positive(num: u128) {
            assert_eq!(u128::from(Zequal::from(num)).leading_zeros(), num.leading_zeros())
        }

        #[test]
        fn test_convert_id_negative_sign(num: i128) {
            let zeq = i128::from(Zequal::from(num));
            assert_eq!(zeq.signum(), num.signum());
        }

        #[test]
        fn test_convert_id_negative_leading_zeros(num: i128) {
            let zeq = i128::from(Zequal::from(num));
            assert_eq!(zeq.abs().leading_zeros(), num.abs().leading_zeros());
        }

        #[test]
        // The zero special case is excluded here.
        fn test_zequals_u128(num in 1..u128::MAX) {
            let zeq = num.zequals();
            assert_eq!(zeq.count_ones(), 1);

            // zeq <= num < 2*zeq
            assert!(zeq <= num);
            if num.leading_ones() == 0 {
                assert!(zeq<<1 > num)
            }
        }
    }
}
