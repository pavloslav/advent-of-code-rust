fn seat_id(code:&str) -> usize
{
    code.chars()
        .map(|c|if c=='F' || c=='L' {0} else {1})
        .fold(0, |acc, n|acc*2+n)
}

fn max_pass_number(s:&str) -> usize
{
    s.lines()
     .map(|line|seat_id(line))
     .max()
     .unwrap()
}

fn task1(s:&str) -> usize
{
    max_pass_number(s)
}

fn empty_seat(s:&str) -> usize
{
    let (min, max, sum) = s
        .lines()
        .fold((1024,0,0),|(min, max, sum), line|{
            let id = seat_id(line);
            (std::cmp::min(id, min), std::cmp::max(id, max), sum+id)
        }); 
    (max-min+1)*(max+min)/2 - sum
}

fn task2(s:&str) -> usize
{
    empty_seat(s)
}


fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("5","2020").unwrap();
    println!("Result1: {}",task1(&input));
    println!("Result2: {}",task2(&input));
}