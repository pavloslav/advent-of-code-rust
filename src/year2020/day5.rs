fn seat_id(code:&str) -> usize
{
    code.chars()
        .map(|c|if c=='F' || c=='L' {0} else {1})
        .fold(0, |acc, n|acc*2+n)
}

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(s:&str) -> usize
{
    s.lines()
     .map(|line|seat_id(line))
     .max()
     .unwrap()
}

pub fn task2(s:&str) -> usize
{
    let (min, max, sum) = s
        .lines()
        .fold((1024,0,0),|(min, max, sum), line|{
            let id = seat_id(line);
            (std::cmp::min(id, min), std::cmp::max(id, max), sum+id)
        }); 
    (max-min+1)*(max+min)/2 - sum
}