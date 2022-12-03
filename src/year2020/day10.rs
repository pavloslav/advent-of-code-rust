pub fn parse_input(input: &str) -> Vec<usize> {
    let mut adapters: Vec<usize> =
        input.lines().map(|l| l.parse().unwrap()).collect();
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();
    adapters
}

fn count_differences_in_sorted(adapters: &[usize]) -> [usize; 3] {
    let mut result = [0, 0, 0];
    for w in adapters.windows(2) {
        result[w[1] - w[0] - 1] += 1;
    }
    result
}

pub fn task1(data: &[usize]) -> usize {
    let [diff1, _, diff3] = count_differences_in_sorted(data);
    diff1 * diff3
}

fn count_arranjements(adapters: &[usize]) -> usize {
    let mut paths = vec![0; adapters.len()];
    paths[0] = 1;
    for i in 1..adapters.len() {
        for back in 1..=3 {
            if back <= i && adapters[i] - adapters[i - back] <= 3 {
                //println!("from {} to {} there are {} ways", adapters[i-back], adapters[i], paths[i-back]);
                paths[i] += paths[i - back];
            }
        }
    }
    paths[adapters.len() - 1]
}

pub fn task2(data: &[usize]) -> usize {
    count_arranjements(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tasks() {
        let input1 = "16
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
        let data = parse_input(input1);
        assert_eq!(task1(&data), 35);
        assert_eq!(task2(&data), 8);
    }
}
