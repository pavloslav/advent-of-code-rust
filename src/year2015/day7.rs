#[derive(Clone)]
pub enum Wire {
    Expression(String),
    Value(u16),
}

#[derive(Clone)]
pub struct Wires {
    map: std::collections::HashMap<String, Wire>,
}

pub fn parse_input(input: &str) -> Wires {
    Wires{ map: input.lines()
              .map(|line| {
                let mut parts = line.split(" -> ");
                let left = parts.next().unwrap().into();
                let right = parts.next().unwrap().into();
                (right, Wire::Expression(left))
             })
             .collect()
    }
}

impl Wires {
    fn calculate(&mut self, name: &String) -> u16 {
        if name.chars().all(|c|c.is_digit(10)) {
            name.parse().unwrap()
        } else {
            let wire = self.map[name].clone();
            match wire {
                Wire::Expression(value) => {
                    let calc_value = if value.contains(" AND ") {
                        let mut ops = value.split(" AND ");
                        self.calculate(&ops.next().unwrap().into()) & self.calculate(&ops.next().unwrap().into())
                    } else if value.contains(" OR ") {
                        let mut ops = value.split(" OR ");
                        self.calculate(&ops.next().unwrap().into()) | self.calculate(&ops.next().unwrap().into())
                    } else if value.starts_with("NOT ") {
                        !self.calculate(&value[4..].into())
                    } else if value.contains(" LSHIFT ") {
                        let mut ops = value.split(" LSHIFT ");
                        self.calculate(&ops.next().unwrap().into()) << ops.next().unwrap().parse::<u16>().unwrap()
                    } else if value.contains(" RSHIFT ") {
                        let mut ops = value.split(" RSHIFT ");
                        self.calculate(&ops.next().unwrap().into()) >> ops.next().unwrap().parse::<u16>().unwrap()
                    } else if value.chars().all(|c|c.is_digit(10)) {
                        value.parse().unwrap()
                    } else{
                        self.calculate(&value)
                    };
                    self.map.insert(name.to_owned(), Wire::Value(calc_value));
                    calc_value
                },
                Wire::Value(x) => x
            }
            }
    }
}

pub fn task1(wires: &Wires) -> u16 {
    let mut wires = wires.clone();
    wires.calculate(&"a".into())
}

pub fn task2(wires: &Wires) -> u16 {
    let mut cwires = wires.clone();
    let a = cwires.calculate(&"a".into());
    let mut cwires = wires.clone();
    cwires.map.insert("b".into(), Wire::Value(a));
    cwires.calculate(&"a".into())
}