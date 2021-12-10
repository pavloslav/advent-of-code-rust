#[derive(Debug)]
struct Ship {
    lon: i32,
    lat: i32,
    dir: i32,
}

const DIRECTIONS:[(i32, i32); 4] = [(1,0),(0,1),(-1,0),(0,-1)];

impl Ship {
    fn step(&mut self, action: &str) {
        let command       = action.chars().nth(0).unwrap();
        let parameter:i32 = action[1..].parse().unwrap();
        match command {
            'E' => self.lon+=parameter,
            'W' => self.lon-=parameter,
            'S' => self.lat-=parameter,
            'N' => self.lat+=parameter,
            'L' => self.dir = (self.dir as i32 + parameter/90).rem_euclid(4),
            'R' => self.dir = (self.dir as i32 - parameter/90).rem_euclid(4),
            'F' => { 
                self.lon += DIRECTIONS[self.dir as usize].0 * parameter;
                self.lat += DIRECTIONS[self.dir as usize].1 * parameter;
            }
            _ => panic!()
        }
        //dbg!(self);
    }
    fn new() -> Ship { Ship {lon:0, lat:0, dir:0} }
    fn distance(&self) -> i32 {
        self.lon.abs() + self.lat.abs()
    }
    fn travel(&mut self, s:&str) {
        for line in s.lines() {
            self.step(line);
        }
    }

}

struct ShipWaypoint
{
    lon: i32,
    lat: i32,
    waypt_lon: i32,
    waypt_lat: i32,
}


impl ShipWaypoint {
    fn step(&mut self, action: &str) {
        let command       = action.chars().nth(0).unwrap();
        let parameter:i32 = action[1..].parse().unwrap();
        match command {
            'E' => self.waypt_lon+=parameter,
            'W' => self.waypt_lon-=parameter,
            'S' => self.waypt_lat-=parameter,
            'N' => self.waypt_lat+=parameter,
            'L' => self.turn(parameter),
            'R' => self.turn(-parameter),
            'F' => { 
                self.lon += self.waypt_lon * parameter;
                self.lat += self.waypt_lat * parameter;
            }
            _ => panic!()
        }
    }
    fn new() -> ShipWaypoint { ShipWaypoint {lon:0, lat:0, waypt_lon:10, waypt_lat:1} }
    fn distance(&self) -> i32 {
        self.lon.abs() + self.lat.abs()
    }
    fn travel(&mut self, s:&str) {
        for line in s.lines() {
            self.step(line);
        }
    }
    fn turn(&mut self, angle:i32) {
        match angle.rem_euclid(360) {
            0 => {},
            90 => {
                let t = self.waypt_lon;
                self.waypt_lon = -self.waypt_lat;
                self.waypt_lat = t;
            },
            180 => {
                self.waypt_lon = -self.waypt_lon;
                self.waypt_lat = -self.waypt_lat;
            },
            270 => {
                let t = self.waypt_lon;
                self.waypt_lon = self.waypt_lat;
                self.waypt_lat = -t;
            },
            _ => panic!()
        }
    }
}

fn task1(s:&str) -> i32
{
    let mut ship = Ship::new();
    ship.travel(s);
    ship.distance()
}

fn task2(s:&str) -> i32
{
    let mut ship = ShipWaypoint::new();
    ship.travel(s);
    ship.distance()
}

#[cfg(test)]
mod test {
    use crate::task1;
    use crate::task2;
    #[test]
    fn test_tasks() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(task1(input), 25);
        assert_eq!(task2(input), 286);
    }
}

fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("12","2020").unwrap();
    println!("Result1: {}",task1(&input));
    println!("Result2: {}",task2(&input));
}