/*
 * keeps infected only
 */
type Map = std::collections::HashMap<(i32, i32), i32>;

pub fn parse_input(input: &str) -> (Map, usize, usize) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    println!("{height} {width}");
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    ((x as i32, y as i32), if c == '#' { 2 } else { 0 })
                })
            })
            .collect(),
        width,
        height,
    )
}

struct Carrier {
    x: i32,
    y: i32,
    dir: i32, /* 0 - up, 1 - right, 2 - down, 3 - left */
    infected: usize,
}

impl Carrier {
    fn new(width: usize, height: usize) -> Carrier {
        Carrier {
            x: (width / 2) as i32,
            y: (height / 2) as i32,
            dir: 0,
            infected: 0,
        }
    }
    fn burst(&mut self, map: &mut Map, step: i32) {
        let val = map.entry((self.x, self.y)).or_insert(0);
        //dirty hack
        self.dir = (self.dir + 3 + *val) % 4;
        *val = (*val + step) % 4;
        if *val == 2 {
            self.infected += 1;
        }

        match self.dir {
            0 => self.y -= 1,
            1 => self.x += 1,
            2 => self.y += 1,
            3 => self.x -= 1,
            _ => unreachable!(),
        }
    }
}

pub fn task1((map, width, height): &(Map, usize, usize)) -> usize {
    let mut map = map.clone();
    let mut carrier = Carrier::new(*width, *height);
    for _step in 0..10_000 {
        carrier.burst(&mut map, 2);
    }
    carrier.infected
}

pub fn task2((map, width, height): &(Map, usize, usize)) -> usize {
    let mut map = map.clone();
    let mut carrier = Carrier::new(*width, *height);
    for _step in 0..10_000_000 {
        carrier.burst(&mut map, 1);
    }
    carrier.infected
}
