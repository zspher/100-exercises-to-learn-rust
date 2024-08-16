// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

trait Power<T: Copy> {
    fn power(self, exp: T) -> Self;
}
macro_rules! power_trait_alias {
    ($to: ty, $exp_type: ty) => {
        impl Power<$exp_type> for $to {
            fn power(self, mut exp: $exp_type) -> Self {
                let mut base: Self = self;
                let mut acc: Self = 1;

                if exp == 0 {
                    return 1;
                }

                while exp > 1 {
                    if (exp & 1) == 1 {
                        acc *= base;
                    }
                    exp /= 2;
                    base *= base
                }

                base * acc
            }
        }
        impl Power<&$exp_type> for $to {
            fn power(self, exp: &$exp_type) -> Self {
                let mut base: Self = self;
                let mut acc: Self = 1;
                let mut exp = *exp;

                if exp == 0 {
                    return 1;
                }

                while exp > 1 {
                    if (exp & 1) == 1 {
                        acc *= base;
                    }
                    exp /= 2;
                    base *= base
                }

                base * acc
            }
        }
    };
}

power_trait_alias!(u32, u16);
power_trait_alias!(u32, u32);

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
