use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Module<'a> {
    Broadcaster {
        dst: Vec<&'a str>,
    },
    FlipFlop {
        flip: bool,
        dst: Vec<&'a str>,
    },
    Conjunction {
        input: HashMap<&'a str, bool>,
        dst: Vec<&'a str>,
    },
}

#[derive(prse::Parse)]
enum InputModule<'a> {
    #[prse = "broadcaster -> {dst:, :}"]
    Broadcaster { dst: Vec<&'a str> },
    #[prse = "%{name} -> {dst:, :}"]
    FlipFlop { name: &'a str, dst: Vec<&'a str> },
    #[prse = "&{name} -> {dst:, :}"]
    Conjunction { name: &'a str, dst: Vec<&'a str> },
}

type Modules<'a> = HashMap<&'a str, Module<'a>>;

pub fn parse_input(input: &str) -> anyhow::Result<Modules> {
    let mut modules = Modules::new();
    let mut back_map = HashMap::new();
    for line in input.lines() {
        match prse::try_parse!(line, "{}")? {
            InputModule::Broadcaster { dst } => {
                for &d in &dst {
                    if let Some(Module::Conjunction { input, .. }) = modules.get_mut(d) {
                        input.insert(d, false);
                    } else {
                        back_map
                            .entry(d)
                            .and_modify(|v: &mut Vec<&str>| v.push("broadcaster"))
                            .or_insert(vec!["broadcaster"]);
                    }
                }
                modules.insert("broadcaster", Module::Broadcaster { dst });
            }
            InputModule::FlipFlop { name, dst } => {
                for d in &dst {
                    if let Some(Module::Conjunction { input, .. }) = modules.get_mut(d) {
                        input.insert(name, false);
                    } else {
                        back_map
                            .entry(d)
                            .and_modify(|v: &mut Vec<&str>| v.push(name))
                            .or_insert(vec![name]);
                    }
                }
                modules.insert(name, Module::FlipFlop { flip: false, dst });
            }
            InputModule::Conjunction { name, dst } => {
                let input = back_map
                    .get(&name)
                    .map(|v| v.iter().map(|&n| (n, false)).collect())
                    .unwrap_or(HashMap::new());
                modules.insert(name, Module::Conjunction { input, dst });
            }
        }
    }
    Ok(modules)
}

#[derive(Debug)]
struct Signal<'a> {
    src: &'a str,
    tgt: &'a str,
    kind: bool,
}

impl<'a> Signal<'a> {
    fn new(src: &'a str, tgt: &'a str, kind: bool) -> Self {
        Self { src, tgt, kind }
    }
}

struct Bus<'a> {
    signals: std::collections::VecDeque<Signal<'a>>,
    lo: u128,
    hi: u128,
}

impl<'a> Module<'a> {
    fn work(&mut self, signal: Signal<'a>, bus: &mut Bus<'a>) {
        match self {
            Module::Broadcaster { dst } => {
                for d in dst {
                    bus.send(signal.tgt, d, signal.kind);
                }
            }
            Module::FlipFlop { flip, dst } => {
                if !signal.kind {
                    *flip = !*flip;
                    for d in dst {
                        bus.send(signal.tgt, d, *flip);
                    }
                }
            }
            Module::Conjunction { input, dst } => {
                input.entry(signal.src).and_modify(|val| *val = signal.kind);
                let not_all_hi = !input.values().all(|&v| v);
                for d in dst {
                    bus.send(signal.tgt, d, not_all_hi);
                }
            }
        }
    }
}

impl<'a> Bus<'a> {
    fn send(&mut self, src: &'a str, tgt: &'a str, kind: bool) {
        self.signals.push_back(Signal::new(src, tgt, kind));
        if kind {
            self.hi += 1;
        } else {
            self.lo += 1;
        }
    }
    fn new() -> Self {
        Self {
            signals: std::collections::VecDeque::new(),
            lo: 0,
            hi: 0,
        }
    }
    fn score(&self) -> u128 {
        self.lo * self.hi
    }
    fn press_button(&mut self) {
        self.send("button", "broadcaster", false);
    }
}

pub fn task1(input: &Modules) -> anyhow::Result<u128> {
    let mut modules = input.clone();
    let mut signals = Bus::new();
    for _ in 0..1000 {
        signals.press_button();
        while let Some(signal) = signals.signals.pop_front() {
            if let Some(module) = modules.get_mut(signal.tgt) {
                module.work(signal, &mut signals);
            }
        }
    }
    Ok(signals.score()) //957445587 - hi
}

pub fn task2(_input: &Modules) -> anyhow::Result<i32> {
    anyhow::bail!("Todo")
}

#[cfg(test)]
mod test {
    use super::{parse_input, task1, task2};

    const FIRST: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const SECOND: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(FIRST).unwrap()).unwrap(), 32000000);
        assert_eq!(task1(&parse_input(SECOND).unwrap()).unwrap(), 11687500);
    }
}
