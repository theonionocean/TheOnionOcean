pub trait CreditCard {
    fn is_credit_card(&self) -> bool;
}

macro_rules! impl_credit_card {
    ($($ty:ty),* $(,)?) => {
        $(
            impl CreditCard for $ty {
                fn is_credit_card(&self) -> bool {
                    let value = self.replace('-', "").replace(' ', "");

                    let mut checksum = 0;
                    let mut even_digit = false;

                    for digit in value.chars().rev() {
                        if !digit.is_ascii_digit() {
                            return false;
                        }

                        let mut digit_value =
                            (digit as u8 - b'0') as u32 * if even_digit { 2 } else { 1 };
                        even_digit = !even_digit;

                        while digit_value > 0 {
                            checksum += digit_value % 10;
                            digit_value /= 10;
                        }
                    }

                    checksum % 10 == 0
                }
            }
        )*
    };
}

impl_credit_card!(str, String);
