type Presents = Vec<Vec<usize>>;

pub fn parse_input(input: &str) -> Presents {
    input.lines()
         .map(|line|line.split('x')
                        .map(|x|x.parse().unwrap())
                        .collect())
         .collect()
}

pub fn task1(presents: &Presents) -> usize {
    presents.iter()
            .map(|dims|{
                let side1 = dims[0]*dims[1];
                let side2 = dims[0]*dims[2];
                let side3 = dims[1]*dims[2];
                2*(side1+side2+side3)+[side1, side2, side3].iter().min().unwrap()
            })
            .sum()
}

pub fn task2(presents: &Presents) -> usize {
    presents.iter()
            .map(|dims| {
                2*(dims.iter().sum::<usize>() - dims.iter().max().unwrap())+dims[0]*dims[1]*dims[2]
            })
            .sum()
}
