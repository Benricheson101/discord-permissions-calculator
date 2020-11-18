#![allow(non_camel_case_types, dead_code)]
mod perms;

#[cfg(test)]
mod tests {
    use super::perms::*;

    #[test]
    fn from_tokens() {
        let perms = vec![
            FLAGS::SEND_MESSAGES,
            FLAGS::ADMINISTRATOR,
        ];

        let p = Perms::from(&perms);

        assert_eq!(p.0, 2056);
    }

    #[test]
    fn from_bits_to_tokens() {
        let p = Perms::new(2056);

        let tokens = vec![
            FLAGS::ADMINISTRATOR,
            FLAGS::SEND_MESSAGES,
        ];

        assert_eq!(p.tokens(), tokens);
    }

    #[test]
    fn add_bit() {
        let mut p = Perms::new(8);
        p.add(&FLAGS::SEND_MESSAGES);

        assert_eq!(p.0, 2056);
    }

    #[test]
    fn remove_bit() {
        let mut p = Perms::new(2056);
        p.remove(&FLAGS::SEND_MESSAGES);

        assert_eq!(p.0, 8);
    }
}
