use regex::Regex;

pub trait Email {
    fn is_email(&self) -> bool;
}

macro_rules! impl_email {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Email for $ty {
                fn is_email(&self) -> bool {
                    // RFC 5322 compliant email regex
                    let regex = Regex::new(r"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*)@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])");
                    match regex {
                        Ok(regex) => regex.is_match(self),
                        Err(e) => {
                            // TODO: Change panic to graceful error handling
                            panic!("Failed to parse email attributes: {}", e.to_string());
                        }
                    }
                }
            }
        )*
    };
}

impl_email!(str, String);
