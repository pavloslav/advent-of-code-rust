use super::super::common::Error;
use super::super::common::Error::TaskError;
use super::super::common::Result;

#[derive(Debug)]
struct Ship {
    lon: i32,
    lat: i32,
    dir: i32,
}

pub enum OrderType {
    East,
    West,
    South,
    North,
    Left,
    Right,
    Forward,
}

pub struct Order {
    com: OrderType,
    par: i32,
}

impl std::str::FromStr for Order {
    type Err = Error;
    fn from_str(s: &str) -> Result<Order> {
        let (typ, par) = scan_fmt::scan_fmt!(s, "{1[EWSNLRF]}{}", char, i32)?;
        Ok(Order {
            com: match typ {
                'E' => OrderType::East,
                'W' => OrderType::West,
                'S' => OrderType::South,
                'N' => OrderType::North,
                'L' => OrderType::Left,
                'R' => OrderType::Right,
                'F' => OrderType::Forward,
                other => Err(TaskError(format!("Unknown order: '{other}'")))?,
            },
            par,
        })
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Ship {
    fn step(&mut self, action: &Order) {
        match action.com {
            OrderType::East => self.lon += action.par,
            OrderType::West => self.lon -= action.par,
            OrderType::South => self.lat -= action.par,
            OrderType::North => self.lat += action.par,
            OrderType::Left => {
                self.dir = (self.dir + action.par / 90).rem_euclid(4)
            }
            OrderType::Right => {
                self.dir = (self.dir - action.par / 90).rem_euclid(4)
            }
            OrderType::Forward => {
                self.lon += DIRECTIONS[self.dir as usize].0 * action.par;
                self.lat += DIRECTIONS[self.dir as usize].1 * action.par;
            }
        }
    }
    fn new() -> Ship {
        Ship {
            lon: 0,
            lat: 0,
            dir: 0,
        }
    }
    fn distance(&self) -> i32 {
        self.lon.abs() + self.lat.abs()
    }
    fn travel(&mut self, orders: &[Order]) {
        for order in orders {
            self.step(order);
        }
    }
}

struct ShipWaypoint {
    lon: i32,
    lat: i32,
    waypt_lon: i32,
    waypt_lat: i32,
}

impl ShipWaypoint {
    fn step(&mut self, action: &Order) -> Result<()> {
        match action.com {
            OrderType::East => self.waypt_lon += action.par,
            OrderType::West => self.waypt_lon -= action.par,
            OrderType::South => self.waypt_lat -= action.par,
            OrderType::North => self.waypt_lat += action.par,
            OrderType::Left => self.turn(action.par)?,
            OrderType::Right => self.turn(-action.par)?,
            OrderType::Forward => {
                self.lon += self.waypt_lon * action.par;
                self.lat += self.waypt_lat * action.par;
            }
        }
        Ok(())
    }
    fn new() -> ShipWaypoint {
        ShipWaypoint {
            lon: 0,
            lat: 0,
            waypt_lon: 10,
            waypt_lat: 1,
        }
    }
    fn distance(&self) -> i32 {
        self.lon.abs() + self.lat.abs()
    }
    fn travel(&mut self, orders: &[Order]) -> Result<()> {
        for order in orders {
            self.step(order)?;
        }
        Ok(())
    }
    fn turn(&mut self, angle: i32) -> Result<()> {
        match angle.rem_euclid(360) {
            0 => {}
            90 => {
                let t = self.waypt_lon;
                self.waypt_lon = -self.waypt_lat;
                self.waypt_lat = t;
            }
            180 => {
                self.waypt_lon = -self.waypt_lon;
                self.waypt_lat = -self.waypt_lat;
            }
            270 => {
                let t = self.waypt_lon;
                self.waypt_lon = self.waypt_lat;
                self.waypt_lat = -t;
            }
            other => {
                return Err(TaskError(format!(
                    "Angle {other} is not supported"
                )));
            }
        }
        Ok(())
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Order>> {
    input.trim().lines().map(|s| s.parse()).collect()
}

pub fn task1(orders: &[Order]) -> Result<i32> {
    let mut ship = Ship::new();
    ship.travel(orders);
    Ok(ship.distance())
}

pub fn task2(orders: &[Order]) -> Result<i32> {
    let mut ship = ShipWaypoint::new();
    ship.travel(orders)?;
    Ok(ship.distance())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tasks() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(task1(&parse_input(&input).unwrap()).unwrap(), 25);
        assert_eq!(task2(&parse_input(&input).unwrap()).unwrap(), 286);
    }
}
