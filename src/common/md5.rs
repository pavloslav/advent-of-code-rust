/*
 * wrapper for rust-crypto
 */

use crypto::digest::Digest;
use crypto::md5::Md5;

pub struct Md5Hasher {
    hasher: Md5,
}

impl Md5Hasher {
    pub fn from_str(s: &str) -> Md5Hasher {
        let mut hasher = Md5Hasher { hasher: Md5::new() };
        hasher.hasher.input_str(s);
        //        assert_eq!(self.hasher.output_bits(), u128::BITS as usize);
        hasher
    }

    pub fn add_str(&mut self, s: &str) -> &mut Md5Hasher {
        self.hasher.input_str(s);
        self
    }

    pub fn as_str(&mut self) -> String {
        self.hasher.result_str()
    }

    pub fn as_u8(&mut self) -> [u8; 16] {
        let mut result = [0u8; 16];
        self.hasher.result(&mut result);
        result
    }

    pub fn has_zeroes(&mut self, zeroes: usize) -> bool {
        let arr = self.as_u8();
        arr[..zeroes / 2].iter().all(|&x| x == 0)
            && (zeroes % 2 == 0 || arr[zeroes / 2] & 0xF0 == 0)
    }
}
