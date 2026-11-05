#![cfg_attr(not(test), no_std)]

pub mod net {
    pub mod byte_manager;
}

pub mod errors {
    pub mod hardware_issue;
    pub mod connectivity_issue;
    pub mod conversion_issue;
    pub mod issue;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
