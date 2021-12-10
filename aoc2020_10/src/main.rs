fn get_ajusted_vec(s: &str) -> Vec<usize> {
    let mut adapters:Vec<usize> = s.lines()
                                   .map(|l|l.parse().unwrap())
                                   .collect();
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap()+3);
    adapters.sort();
    adapters
}

fn count_differences_in_sorted(adapters: Vec<usize>) -> [usize; 3] {
    let mut result = [0,0,0];
    for w in adapters.windows(2) {
            result[w[1]-w[0]-1]+=1;
    }
    result
}

fn task1(s:&str) -> usize {
    let [diff1, _, diff3] = count_differences_in_sorted(get_ajusted_vec(s));
    diff1*diff3
}

fn count_arranjements(adapters: Vec<usize>) -> usize
{
    let mut paths = vec![0; adapters.len()];
    paths[0] = 1;
    for i in 1..adapters.len() {
        for back in 1..=3 {
            if back<=i && adapters[i]-adapters[i-back]<=3 {
                //println!("from {} to {} there are {} ways", adapters[i-back], adapters[i], paths[i-back]);
                paths[i]+=paths[i-back];
            }
        }
    }
    paths[adapters.len()-1]
}

fn task2(s:&str) -> usize {
    count_arranjements(get_ajusted_vec(s))
}

#[cfg(test)]
mod tests {
    use crate::task1;
    use crate::task2;
    #[test]
    fn test_tasks() {
        let input1 = 
"16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(task1(input1), 35);
        assert_eq!(task2(input1), 8);
    }
}



fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("10","2020").unwrap();
    println!("Result1: {}",task1(&input));
    println!("Result2: {}",task2(&input));
}