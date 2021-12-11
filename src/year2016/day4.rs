use std::collections::HashMap;
use std::cmp::Ordering;
use std::iter::FromIterator;

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1( input:&str ) -> i32 {
    let mut sum = 0;
    for line in input.split('\n') {
        let last_dash = line.rfind('-').unwrap();
        let bracket = line.rfind('[').unwrap();
        let ref symbols = line[..last_dash];
        let sector_id = line[last_dash+1..bracket].parse::<i32>().unwrap();
        let ref check_sum = line[bracket+1..bracket+6];
        let mut dict:HashMap<char,u32> = HashMap::new();
        for c in symbols.chars().filter(|c|*c!='-') {
            *dict.entry(c).or_insert(0) += 1;
        }
        let mut calc:Vec<_> = dict.iter().collect();
        calc.sort_by( |x,y| {
                               if x.1>y.1 || x.1==y.1 && x.0<y.0 { Ordering::Less }
                               else { Ordering::Greater }
                            }
                    );
        if String::from_iter(calc.iter().take(5).map(|x|*x.0)) == check_sum {
            sum += sector_id;
        }
        
    }
    sum
}

fn decypher( line:&str, shift:u32 ) -> String {
    let mut result = String::with_capacity(line.len());
    let a_code = u32::from('a');
    
    for c in line.chars() {
        if c=='-' {
            result.push(' ')
        } else {
            result.push( char::from(
                                      ( (u32::from(c)-a_code+shift)%26+a_code) as u8
                                   ) 
                       )
        }
    }
    result
}

pub fn task2( input:&str ) -> u32{
    for line in input.split('\n') {
        let last_dash = line.rfind('-').unwrap();
        let bracket = line.rfind('[').unwrap();
        let ref symbols = line[..last_dash];
        let sector_id = line[last_dash+1..bracket].parse::<u32>().unwrap();
        let ref check_sum = line[bracket+1..bracket+6];
        let mut dict:HashMap<char,u32> = HashMap::new();
        for c in symbols.chars().filter(|c|*c!='-') {
            *dict.entry(c).or_insert(0) += 1;
        }
        let mut calc:Vec<_> = dict.iter().collect();
        calc.sort_by( |x,y| {
                               if x.1>y.1 || x.1==y.1 && x.0<y.0 { Ordering::Less }
                               else { Ordering::Greater }
                            }
                    );
        if String::from_iter(calc.iter().take(5).map(|x|*x.0)) == check_sum {
            println!("{} - {}",sector_id,decypher(symbols,sector_id));
            return sector_id; //WRONG!
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_sum_rooms() {
        let inp="\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";
        assert_eq!(task1( &inp ), 1514 );
        let inp="qzmt-zixmtkozy-ivhz";
        assert_eq!(decypher( &inp, 343 ), "very encrypted name");
    }
}