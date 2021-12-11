const INACTIVE: char = '.';
const ACTIVE  : char = '#';
const STATIONARY: i32 = 2;
const ACTIVATE  : i32 = 3;

#[derive(Clone)]
struct Field3d
{
    cubes:Vec<Vec<Vec<Vec<char>>>>,
    min_x:i32,
    max_x:i32,
    min_y:i32,
    max_y:i32,
    min_z:i32,
    max_z:i32,
}

fn to_usize(x:i32) -> usize
{
    x.abs() as usize - x.is_negative() as usize
}

fn idx(x:i32,y:i32,z:i32) -> usize
{
      ((x.is_negative() as usize)<<2)
    + ((y.is_negative() as usize)<<1)
    +  (z.is_negative() as usize)
}

impl Field3d
{
    fn get(&self, x:i32, y:i32, z:i32) -> char
    {
        let vec_idx =idx(x,y,z);
        let x = to_usize(x);
        if self.cubes[vec_idx].len()<=x {
            INACTIVE
        } else {
            let y = to_usize(y);
            if self.cubes[vec_idx][x].len()<=y {
                INACTIVE
            } else {
                let z = to_usize(z);
                if self.cubes[vec_idx][x][y].len()<=z {
                   INACTIVE
                } else {
                    self.cubes[vec_idx][x][y][z]
                }
            }
        }
    }
    fn set(&mut self, x:i32, y:i32, z:i32, value:char)
    {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.min_z = self.min_z.min(z);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
        self.max_z = self.max_z.max(z);

        let vec_idx =idx(x,y,z);

        let x = to_usize(x);
        if self.cubes[vec_idx].len()<=x {
            self.cubes[vec_idx].resize_with(x+1, ||Vec::new());
        } 
        let y = to_usize(y);
        if self.cubes[vec_idx][x].len()<=y {
            self.cubes[vec_idx][x].resize_with(y+1, ||Vec::new());
        } 
        let z = to_usize(z);
        if self.cubes[vec_idx][x][y].len()<=z {
           self.cubes[vec_idx][x][y].resize_with(z+1, ||INACTIVE);
        }
        self.cubes[vec_idx][x][y][z] = value;
    }

    fn new(s:&str) -> Field3d
    {
        let mut field = Field3d {
            cubes:Vec::with_capacity(8),
            min_x:0,
            min_y:0,
            min_z:0,
            max_x:0,
            max_y:0,
            max_z:0,
        };
        field.cubes.resize_with(8, ||Vec::new());
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                field.set(x as i32,y as i32,0,ch);
            }
        }
        println!("Field created with {} active", field.count_active());
        field
    }
    fn step(&mut self) {
        let mut new = self.clone();
        for x in self.min_x-1..=self.max_x+1 {
            for y in self.min_y-1..=self.max_y+1 {
                for z in self.min_z-1..=self.max_z+1 {
                    match self.neighbors(x,y,z) {
                        ACTIVATE => if self.get(x,y,z)==INACTIVE {
                                new.set(x,y,z,ACTIVE);
                            },
                        STATIONARY => {},
                        _ => if self.get(x,y,z)==ACTIVE {
                                new.set(x,y,z,INACTIVE);
                            },
                    }
                }
            }
        }
        *self = new;
    }
    fn neighbors(&self, x:i32, y:i32, z:i32) -> i32
    {
        let mut result = 0;
        for i in x-1..=x+1 {
            for j in y-1..=y+1 {
                for k in z-1..=z+1 {
                    if (i!=x||j!=y||k!=z) && self.get(i,j,k) == ACTIVE {
                       result += 1;
                    }
                }
            }
        }
        result
    }
    fn count_active(&self) -> u64
    {
        let mut result = 0;
        for i in self.min_x..=self.max_x {
            for j in self.min_y..=self.max_y {
                for k in self.min_z..=self.max_z {
                    if self.get(i,j,k) == ACTIVE {
                       result += 1;
                    }
                }
            }
        }
        result
    }
    /*
    fn print(&self)
    {
        for z in self.min_z..=self.max_z {
            println!("z={}",z);
            for y in self.min_y..=self.max_y {
                for x in self.min_x..=self.max_x {
                    print!("{}",self.get(x,y,z));
                }
                println!("");
            }
        }

    }*/
}

#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    fn test_task1() {
        assert_eq!(task1(".#.
..#
###"),112);
    }
}

#[derive(Clone)]
struct Field4d
{
    cubes:Vec<Vec<Vec<Vec<Vec<char>>>>>,
    min_x:i32,
    max_x:i32,
    min_y:i32,
    max_y:i32,
    min_z:i32,
    max_z:i32,
    min_w:i32,
    max_w:i32,
}

fn idx4(x:i32,y:i32,z:i32, w:i32) -> usize
{
      ((x.is_negative() as usize)<<3)
    + ((y.is_negative() as usize)<<2)
    + ((z.is_negative() as usize)<<1)
    + ((w.is_negative() as usize)<<0)
}

impl Field4d
{
    fn get(&self, x:i32, y:i32, z:i32, w:i32) -> char
    {
        let vec_idx =idx4(x,y,z,w);
        let x = to_usize(x);
        if self.cubes[vec_idx].len()<=x {
            INACTIVE
        } else {
            let y = to_usize(y);
            if self.cubes[vec_idx][x].len()<=y {
                INACTIVE
            } else {
                let z = to_usize(z);
                if self.cubes[vec_idx][x][y].len()<=z {
                   INACTIVE
                } else {
                    let w = to_usize(w);
                    if self.cubes[vec_idx][x][y][z].len()<=w {
                        INACTIVE
                    } else{
                        self.cubes[vec_idx][x][y][z][w]
                    }
                }
            }
        }
    }
    fn set(&mut self, x:i32, y:i32, z:i32, w:i32, value:char)
    {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.min_z = self.min_z.min(z);
        self.min_w = self.min_w.min(w);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
        self.max_z = self.max_z.max(z);
        self.max_w = self.max_w.max(w);

        let vec_idx =idx4(x,y,z,w);

        let x = to_usize(x);
        if self.cubes[vec_idx].len()<=x {
            self.cubes[vec_idx].resize_with(x+1, ||Vec::new());
        } 
        let y = to_usize(y);
        if self.cubes[vec_idx][x].len()<=y {
            self.cubes[vec_idx][x].resize_with(y+1, ||Vec::new());
        } 
        let z = to_usize(z);
        if self.cubes[vec_idx][x][y].len()<=z {
           self.cubes[vec_idx][x][y].resize_with(z+1, ||Vec::new());
        }
        let w = to_usize(w);
        if self.cubes[vec_idx][x][y][z].len()<=w {
           self.cubes[vec_idx][x][y][z].resize_with(w+1, ||INACTIVE);
        }
        self.cubes[vec_idx][x][y][z][w] = value;
    }

    fn new(s:&str) -> Field4d
    {
        let mut field = Field4d {
            cubes:Vec::with_capacity(8),
            min_x:0,
            min_y:0,
            min_z:0,
            min_w:0,
            max_x:0,
            max_y:0,
            max_z:0,
            max_w:0,
        };
        field.cubes.resize_with(16, ||Vec::new());
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                field.set(x as i32,y as i32,0,0,ch);
            }
        }
        println!("Field created with {} active", field.count_active());
        field
    }
    fn step(&mut self) {
        let mut new = self.clone();
        for x in self.min_x-1..=self.max_x+1 {
            for y in self.min_y-1..=self.max_y+1 {
                for z in self.min_z-1..=self.max_z+1 {
                    for w in self.min_w-1..=self.max_w+1 {
                        match self.neighbors(x,y,z,w) {
                            ACTIVATE => if self.get(x,y,z,w)==INACTIVE {
                                    new.set(x,y,z,w,ACTIVE);
                                },
                            STATIONARY => {},
                            _ => if self.get(x,y,z,w)==ACTIVE {
                                    new.set(x,y,z,w,INACTIVE);
                                },
                        }
                    }
                }
            }
        }
        *self = new;
    }
    fn neighbors(&self, x:i32, y:i32, z:i32, w:i32) -> i32
    {
        let mut result = 0;
        for i in x-1..=x+1 {
            for j in y-1..=y+1 {
                for k in z-1..=z+1 {
                    for l in w-1..=w+1 {
                        if (i!=x||j!=y||k!=z||l!=w) && self.get(i,j,k,l) == ACTIVE {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }
    fn count_active(&self) -> u64
    {
        let mut result = 0;
        for i in self.min_x..=self.max_x {
            for j in self.min_y..=self.max_y {
                for k in self.min_z..=self.max_z {
                    for l in self.min_w..=self.max_w {
                        if self.get(i,j,k,l) == ACTIVE {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }
    /*
    fn print(&self)
    {
        for z in self.min_z..=self.max_z {
            println!("z={}",z);
            for y in self.min_y..=self.max_y {
                for x in self.min_x..=self.max_x {
                    print!("{}",self.get(x,y,z));
                }
                println!("");
            }
        }

    }*/
}

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(s:&str) -> u64
{
    let mut field = Field3d::new(s);
    for _ in 0..6 {
        field.step();
    }
    field.count_active()
}

pub fn task2(s:&str) -> u64
{
    let mut field = Field4d::new(s);
    for _ in 0..6 {
        field.step();
    }
    field.count_active()
}