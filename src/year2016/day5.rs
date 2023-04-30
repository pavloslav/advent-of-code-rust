use crate::*;

use super::super::common::Md5Hasher;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input.trim())
}

pub fn task1(input: &str) -> Result<String> {
    let mut result = String::with_capacity(8);
    for i in 0.. {
        let mut hasher = Md5Hasher::new_from_str(input);
        if hasher.add_str(&i.to_string()).has_zeroes(5) {
            result.push_str(&hasher.as_str()[5..6]);
        }
        if result.len() == 8 {
            break;
        }
    }
    Ok(result)
}

pub fn task2(input: &str) -> Result<String> {
    let mut result = ['X'; 8];
    let mut count = 0;
    for i in 0.. {
        let mut hasher = Md5Hasher::new_from_str(input);
        if hasher.add_str(&i.to_string()).has_zeroes(5) {
            let md5 = hasher.as_u8();
            let pos = (md5[2] & 0b1111) as usize;
            let symb = char::from_digit(md5[3] as u32 >> 4, 16).unwrap();
            if pos < result.len() && result[pos] == 'X' {
                result[pos] = symb;
                count += 1;
            }
        }
        if count == 8 {
            break;
        }
    }
    Ok(result.iter().cloned().collect::<String>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_password() {
        assert_eq!(task1("abc").unwrap(), "18f47a30");
        assert_eq!(task2("abc").unwrap(), "05ace8e3");
    }
}
