extern crate aoc;

fn code( lines : &str, map: &Vec<&str>, initial:(usize,usize) ) -> String {
    let mut pos = initial;
    let mut result = String::with_capacity(4); 
    for line in lines.split('\n') {
        for mov in line.chars() {
            let old_pos = pos;
            match mov {
                'U' => pos.1 -= 1, 
                'R' => pos.0 += 1,
                'D' => pos.1 += 1,
                'L' => pos.0 -= 1,
                _   => panic!("Wrong input!"),
            }
            if map[pos.1].chars().nth(pos.0) == Some('X') {
                pos = old_pos;
            }
        }
        result.push(map[pos.1].chars().nth(pos.0).unwrap());
   }
   result
}

fn main() {
    let map = vec![
        "XXXXX",
        "X123X",
        "X456X",
        "X789X",
        "XXXXX",
    ];
    let input = aoc::get_input_from_ini(env!("CARGO_PKG_NAME")).unwrap();
    println!("Answer is {}", code(&input, &map, (2,2)));

    let map = vec![
        "XXXXXXX",
        "XXX1XXX",
        "XX234XX",
        "X56789X",
        "XXABCXX",
        "XXXDXXX",
        "XXXXXXX",
    ];
    println!("Second answer is {}", code(&input, &map, (1,3)));
}

#[test]
fn test_code() {
    let map = vec![
        "XXXXX",
        "X123X",
        "X456X",
        "X789X",
        "XXXXX",
    ];
    assert_eq!(code("ULL
RRDDD
LURDL
UUUUD", &map, (2,2)), "1985");
    let map = vec![
        "XXXXXXX",
        "XXX1XXX",
        "XX234XX",
        "X56789X",
        "XXABCXX",
        "XXXDXXX",
        "XXXXXXX",
    ];
    assert_eq!(code("ULL
RRDDD
LURDL
UUUUD", &map, (1,3)), "5DB3");
}
