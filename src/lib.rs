//! Utility for generating random blobs of data
//! ```
//! let lorem = get_lorem(1024, true);
//! assert!(lorem.as_bytes() >= 1024);
//!
//! let junk = get_string(1024, "junk", false);
//! assert!(junk.as_bytes() >= 1024);
//!
//! let blob = get_blob(1024, &[1, 2, 3]);
//! assert!(test_val.len() >= 1024);
//! ```

extern crate rand;

/// This function returns a string at least as long as the
/// `bytes` parameter. The `numbered` will prefix each iteration
/// of lorem ipsum.
pub fn get_lorem(bytes: usize, numbered: bool) -> String {
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    get_string(bytes, lorem, numbered)
}

/// This function returns a string at least as long as the
/// `bytes` parameter. The `template` `&str` will be repeated
/// the number of times required to reach the `bytes`
/// exceeding the value if it doesn't evenly divide into the `bytes`
/// if `numbered` is true, each iteration will be extened by the number
/// plus '.' and a space. Any overflow will be truncated
pub fn get_string(bytes: usize, template: &str, numbered: bool) -> String {
    let mut ret = String::new();
    if numbered {
        for i in 0..((bytes / template.len()) + 1) {
            ret.push_str(format!("{}. {}", i, template).as_str());
        }
    } else {
        while ret.as_bytes().len() < bytes {
            ret.push_str(format!("{}", template).as_str());
        }
    }
    ret.truncate(bytes);
    ret
}

pub fn get_rng_blob(bytes: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    while ret.len() < bytes {
        ret.push(rand::random());
    }
    ret
}

/// Get a vector of u8 values the length of the bytes param
/// the template param will be repeated to fill the return
/// value
pub fn get_blob(bytes: usize, template: &[u8]) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    for _ in 0..((bytes / template.len() as usize) + 1) {
        ret.extend(template);
    }
    ret.truncate(bytes);
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lorem_test() {
        let test_val = get_lorem(1024, true);
        let test_val_len = test_val.as_bytes().len();
        assert_eq!(test_val_len, 1024);
    }

    #[test]
    fn non_lorem_test() {
        let template = "test";
        let test_val = get_string(1024, template, false);
        assert_eq!(test_val.as_bytes().len(), 1024);
        assert!(test_val.contains(template));
        let empty_test = test_val.replace(template, "");
        assert_eq!(empty_test.len(), 0);
    }

    #[test]
    fn blob_test() {
        let template = &[1, 2, 3];
        let test_val = get_blob(1024, template);
        assert!(test_val.len() >= 1024);
        let bytes = get_blob(6, template);
        assert_eq!(bytes, &[1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn rng_blob_test() {
        let test_val = get_rng_blob(1024);
        assert_eq!(test_val.len(), 1024);
    }
}
