use crate::*;

pub fn parse_input(input: &str) -> AocResult<&str> {
    Ok(input.trim())
}

fn decode2_len(input: &str) -> u64 {
    let array: Vec<_> = input.chars().collect();
    let mut i = 0;
    let mut res = 0u64;
    while i < array.len() {
        res += match array[i] {
            '(' => {
                let start = i + 1;
                while array[i] != 'x' {
                    i += 1;
                }
                let x = i;
                let len: usize = input[start..x].parse().unwrap();
                while array[i] != ')' {
                    i += 1;
                }
                let end = i + 1;
                let times: u64 = input[x + 1..i].parse().unwrap();
                i += len + 1;
                times * decode2_len(&input[end..end + len])
            }
            _ => {
                i += 1;
                1u64
            }
        }
    }
    res
}

fn decode(input: &str) -> String {
    let array: Vec<_> = input.chars().collect();
    let mut i = 0;
    let mut res = String::new();
    while i < array.len() {
        res += &match array[i] {
            '(' => {
                let start = i + 1;
                while array[i] != 'x' {
                    i += 1;
                }
                let x = i;
                let len: usize = input[start..x].parse().unwrap();
                while array[i] != ')' {
                    i += 1;
                }
                let end = i + 1;
                let times: usize = input[x + 1..i].parse().unwrap();
                i += len + 1;
                input[end..end + len].repeat(times)
            }
            other => {
                i += 1;
                other.to_string()
            }
        }
    }
    res
}

pub fn task1(input: &str) -> AocResult<usize> {
    let res = decode(input);
    Ok(res.len())
}

pub fn task2(input: &str) -> AocResult<u64> {
    Ok(decode2_len(input))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode("ADVENT"), "ADVENT");
        assert_eq!(decode("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decode("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decode("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decode("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decode("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_decode2_len() {
        assert_eq!(decode2_len("(3x3)XYZ"), 9u64);
        assert_eq!(decode2_len("X(8x2)(3x3)ABCY"), 20u64);
        assert_eq!(decode2_len("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920u64);
        assert_eq!(
            decode2_len("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445u64
        );
    }
}
