use std::collections::BTreeMap;

trait Model
{
    fn run(&mut self, s:&str) -> &Self;
    fn set_mem(&mut self, address:u64, value:u64);
    fn set_mask(&mut self, mask:&str);
    fn get_sum(&self) -> u64;
}

struct ModelOne
{
    mem: BTreeMap<u64,u64>,
    and_mask: u64,
    or_mask: u64,
}

impl ModelOne
{
    fn new() -> ModelOne { ModelOne {mem:BTreeMap::new(), and_mask:u64::MAX, or_mask:0} }
}

impl Model for ModelOne
{
    fn run(&mut self, s:&str) -> &ModelOne
    {
        for line in s.lines() {
            let mut parts = line.split(" = ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            if left == "mask" {
                self.set_mask(right);
            } else {
                let address = left.strip_prefix("mem[").unwrap()
                                  .strip_suffix("]").unwrap()
                                  .parse().unwrap();
                let value = right.parse().unwrap();
                self.set_mem(address, value);
            }
        }
        self
    }
    fn set_mem(&mut self, address:u64, value:u64)
    {
        self.mem.insert(address, value&self.and_mask|self.or_mask);
    }
    fn set_mask(&mut self, mask:&str)
    {
        self.and_mask = mask.chars().fold(u64::MAX, |acc, c|
            match c {
                'X'|'1' => acc<<1 | 1,
                '0' => acc<<1,
                _ => panic!(),
            });
        self.or_mask = mask.chars().fold(0, |acc, c|
            match c {
                'X'|'0' => acc<<1,
                '1' => acc<<1 | 1,
                _ => panic!(),
            });
    }
    fn get_sum(&self) -> u64
    {
        self.mem.values().map(|&x|x).sum()
    }
}

fn task1(s:&str) -> u64
{
    ModelOne::new().run(s).get_sum()
}

struct ModelTwo
{
    mem: BTreeMap<u64,u64>,
    or_mask: u64,
    floating_bits: Vec<u64>
}

impl ModelTwo
{
    fn new() -> ModelTwo { ModelTwo {mem:BTreeMap::new(), floating_bits:Vec::new(), or_mask:0} }
    fn make_masks(&self, x: u64) -> (u64, u64)
    {
        let mut and_mask = u64::MAX;
        let mut or_mask = self.or_mask;
        for (i, &bit) in self.floating_bits.iter().enumerate() {
            let bitvalue = (x>>i)&1;
            if bitvalue == 0 {
                and_mask &= !(1<<bit);
            } else {
                or_mask |= (1)<<bit;
            }
        }
        (and_mask, or_mask)
    }
}


impl Model for ModelTwo
{
    fn run(&mut self, s:&str) -> &ModelTwo
    {
        for line in s.lines() {
            let mut parts = line.split(" = ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            if left == "mask" {
                self.set_mask(right);
            } else {
                let address = left.strip_prefix("mem[").unwrap()
                                  .strip_suffix("]").unwrap()
                                  .parse().unwrap();
                let value = right.parse().unwrap();
                self.set_mem(address, value);
            }
        }
        self
    }
    fn set_mem(&mut self, address:u64, value:u64)
    {
        for i in 0..1<<self.floating_bits.len() {
            let (and_mask, or_mask) = self.make_masks(i);
            //println!("mem[{:b}&!{:b}|{:b}={}]={}", address, !and_mask, or_mask, address&and_mask|or_mask, value);
            self.mem.insert(address&and_mask|or_mask, value);
        }
    }
    fn set_mask(&mut self, mask:&str)
    {
        self.floating_bits = mask.chars()
                                 .rev()
                                 .enumerate()
                                 .filter_map(|(i,c)|if c=='X' {
                                    Some(i as u64)
                                 } else {
                                    None
                                }).collect();
        self.or_mask = mask.chars().fold(0, |acc, c|
            match c {
                'X'|'0' => acc<<1,
                '1' => acc<<1 | 1,
                _ => panic!(),
            });
        //println!("Mask: {} or: {:#x} floating {:?}", mask, self.or_mask, self.floating_bits);
    }
    fn get_sum(&self) -> u64
    {
        self.mem.values().map(|&x|x).sum()
    }
}

fn task2(s:&str) -> u64
{
    ModelTwo::new().run(s).get_sum()
}

#[cfg(test)]
mod test
{
    use crate::task2;
    #[test]
    fn test_task2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(task2(input), 208)
;    }
}

fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("14","2020").unwrap();
    println!("Result1: {}",task1(&input));
    println!("Result2: {}",task2(&input));
}