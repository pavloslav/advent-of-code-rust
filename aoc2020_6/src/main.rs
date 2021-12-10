fn task1(s:&str) -> u32
{
    let mut counter = 0_u32;
    let mut group   = 0_u32;
    for line in s.lines() {
        if line.is_empty() {
            counter += group.count_ones();
            group = 0;
        }
        for c in line.chars() {
            group |= 1<<(c as u32 - 'a' as u32);
        }
    }
    counter + group.count_ones()
}

fn task2(s:&str) -> u32
{
    let mut counter = 0_u32;
    let mut group   = 0_u32;
    let mut first = true;
    for line in s.lines() {
        if line.is_empty() {
            counter += group.count_ones();
            first = true;
        }
        else {
            let mut person = 0_u32;
            for c in line.chars() {
                person |= 1<<(c as u32 - 'a' as u32);
            }
            if first {
                group = person;
                first = false;
            } else {
                group &= person;
            }
        }
    }
    counter + group.count_ones()
}

#[cfg(test)]
mod tests {
     use crate::task2;
    #[test]
    fn test_task2() {
        let input = 
"abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(task2(input), 6);
    }
}

fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("6","2020").unwrap();
    println!("Result1: {}",task1(&input));
    println!("Result2: {}",task2(&input));
}