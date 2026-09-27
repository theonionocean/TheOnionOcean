use regex::Regex;

pub trait Matches {
    fn is_matches(&self, pattern: &str) -> bool;
}

macro_rules! impl_matches {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Matches for $ty {
                fn is_matches(&self, pattern: &str) -> bool {
                    let regex = Regex::new(pattern);

                    match regex {
                        Ok(regex) => regex.is_match(self),
                        Err(e) => {
                            // TODO: Change panic to graceful error handling
                            panic!("Failed to parse matches attributes: {}", e.to_string());
                        }
                    }
                }
            }
        )*
    };
}

impl_matches!(str, String);
