extern crate aoc;

fn can_be_triangle(a:i32,b:i32,c:i32) -> bool {
         a + b         > c 
    && ( a - b ).abs() < c
}

fn triangles(input:&str) -> i32 {
    let mut sum = 0;
    for line in input.split('\n') {
        let nums:Vec<i32> = line.split_whitespace()
                                .map(|s| s.parse::<i32>()
                                          .unwrap()
                                    )
                                .collect();
        if can_be_triangle(nums[0], nums[1], nums[2]) {
              sum += 1;
          }
    }
    sum
}

fn triangles_3lines(input:&str) -> i32 {
    let mut sum = 0;
    let mut part = Vec::new();
    for line in input.split('\n') {
        let line_ints:Vec<i32> = line.split_whitespace()
                                     .map(|s| s.parse::<i32>()
                                               .unwrap()
                                         )
                                     .collect();
        part.push( line_ints );
        if part.len()==3 {
            for i in 0..3 {
                if  can_be_triangle( part[0][i], part[1][i], part[2][i] ) {
                    sum += 1;
                }
            }
            part = Vec::new();
        } 
    }
    sum
}

fn main() {
    let input = aoc::get_input_from_ini(env!("CARGO_PKG_NAME")).unwrap();
    println!( "Answer is {}", triangles( &input ) );
    println!( "Second answer is {}", triangles_3lines( &input ) );
}

#[test]
fn tri_test(){
    let inp = "5 10 25";
    assert_eq!(triangles(inp),0);
    let inp = "5 3 4";
    assert_eq!(triangles(inp),1);
    let inp = "\
101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";
    assert_eq!(triangles_3lines(inp),6);
}