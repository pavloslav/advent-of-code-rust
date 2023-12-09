pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>>{
    input.lines().map(|lines|Ok(prse::parse(line, "{: :}")?)).collect()
}

pub fn task1(input: &[Vec<i32>]) -> anyhow::Result<i32>{
    Ok(input.iter().map(|line|{
         let mut s=0;
         let mut line = line.clone();
         while !line.iter().all(|x|x==0){
            s+=line.back().context("unreachable")?;
            line = line.iter().zip(line[1..].iter()).map(|(a,b)|b-a).collect();
         }
         s
    }).sum())
}

pub fn task2(_input: &[Vec<i32>]) -> anyhow::Result<i32> {
    anyhow::bail!("Todo")
}