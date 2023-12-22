use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
    time::Instant,
};

#[derive(Debug)]
enum Error {
    InvalidModuleLine,
    MissingModule,
}

#[derive(Clone, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

trait Module {
    fn name(&self) -> &str;
    fn add_input(&mut self, from: &Rc<RefCell<dyn Module>>);
    fn connect(&mut self, to: &Rc<RefCell<dyn Module>>);
    fn process(
        &mut self,
        pulse: &Pulse,
        from: &Rc<RefCell<dyn Module>>,
    ) -> Vec<(Rc<RefCell<dyn Module>>, Pulse)>;
    fn reset(&mut self);
}

type RcModule = Rc<RefCell<dyn Module>>;

struct Broadcast {
    n: String,
    outputs: Vec<RcModule>,
}

struct FlipFlop {
    n: String,
    state: bool,
    outputs: Vec<RcModule>,
}

struct Conjunction {
    n: String,
    inputs: HashMap<String, Pulse>,
    outputs: Vec<RcModule>,
}

impl Broadcast {
    fn new(name: &str) -> Broadcast {
        Broadcast {
            n: name.to_string(),
            outputs: Vec::new(),
        }
    }
}

impl FlipFlop {
    fn new(name: &str) -> FlipFlop {
        FlipFlop {
            n: name.to_string(),
            state: false,
            outputs: Vec::new(),
        }
    }
}

impl Conjunction {
    fn new(name: &str) -> Conjunction {
        Conjunction {
            n: name.to_string(),
            inputs: HashMap::new(),
            outputs: Vec::new(),
        }
    }
}

impl Module for Broadcast {
    fn name(&self) -> &str {
        &self.n
    }

    fn add_input(&mut self, _from: &RcModule) {}

    fn connect(&mut self, to: &RcModule) {
        self.outputs.push(to.clone());
    }

    fn process(&mut self, pulse: &Pulse, _from: &RcModule) -> Vec<(RcModule, Pulse)> {
        self.outputs
            .iter()
            .map(|m| (m.clone(), pulse.clone()))
            .collect()
    }

    fn reset(&mut self) {}
}

impl Module for FlipFlop {
    fn name(&self) -> &str {
        &self.n
    }

    fn add_input(&mut self, _from: &RcModule) {}

    fn connect(&mut self, to: &RcModule) {
        self.outputs.push(to.clone());
    }

    fn process(&mut self, pulse: &Pulse, _from: &RcModule) -> Vec<(RcModule, Pulse)> {
        if pulse == &Pulse::High {
            return Vec::new();
        }

        self.state = !self.state;
        let out_pulse = if self.state { Pulse::High } else { Pulse::Low };

        self.outputs
            .iter()
            .map(|m| (m.clone(), out_pulse.clone()))
            .collect()
    }

    fn reset(&mut self) {
        self.state = false;
    }
}

impl Module for Conjunction {
    fn name(&self) -> &str {
        &self.n
    }

    fn add_input(&mut self, from: &RcModule) {
        self.inputs
            .insert(from.borrow().name().to_string(), Pulse::Low);
    }

    fn connect(&mut self, to: &RcModule) {
        self.outputs.push(to.clone());
    }

    fn process(&mut self, pulse: &Pulse, from: &RcModule) -> Vec<(RcModule, Pulse)> {
        self.inputs
            .insert(from.borrow().name().to_string(), pulse.clone());

        let out_pulse = if self.inputs.iter().find(|(_, p)| **p == Pulse::Low) == None {
            Pulse::Low
        } else {
            Pulse::High
        };

        self.outputs
            .iter()
            .map(|m| (m.clone(), out_pulse.clone()))
            .collect()
    }

    fn reset(&mut self) {
        for (_, v) in self.inputs.iter_mut() {
            *v = Pulse::Low;
        }
    }
}

fn push_button_part1(modules: &HashMap<String, RcModule>) -> Result<(usize, usize), Error> {
    let mut signals_low = 0;
    let mut signals_high = 0;
    let mut next_modules = VecDeque::new();

    let broadcast = modules.get("broadcaster").ok_or(Error::MissingModule)?;
    next_modules.push_back((broadcast.clone(), broadcast.clone(), Pulse::Low));

    while let Some((from, to, pulse)) = next_modules.pop_front() {
        match pulse {
            Pulse::Low => signals_low += 1,
            Pulse::High => signals_high += 1,
        }

        for (nm, ns) in to.borrow_mut().process(&pulse, &from) {
            next_modules.push_back((to.clone(), nm, ns));
        }
    }

    Ok((signals_low, signals_high))
}

fn push_button_part2(
    modules: &HashMap<String, RcModule>,
    trigger_node: &str,
) -> Result<bool, Error> {
    let mut next_modules = VecDeque::new();

    let broadcast = modules.get("broadcaster").ok_or(Error::MissingModule)?;
    next_modules.push_back((broadcast.clone(), broadcast.clone(), Pulse::Low));

    let mut rx_high = 0;

    while let Some((from, to, pulse)) = next_modules.pop_front() {
        if from.borrow().name() == trigger_node && pulse == Pulse::High {
            rx_high += 1;
        }

        for (nm, ns) in to.borrow_mut().process(&pulse, &from) {
            next_modules.push_back((to.clone(), nm, ns));
        }
    }

    Ok(rx_high == 1)
}

fn parse(input: &str) -> Result<HashMap<String, RcModule>, Error> {
    let mut modules: HashMap<String, RcModule> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let raw_name = parts.next().ok_or(Error::InvalidModuleLine)?;
        if raw_name.starts_with("%") {
            let name = &raw_name[1..];
            modules.insert(name.to_string(), Rc::new(RefCell::new(FlipFlop::new(name))));
        } else if raw_name.starts_with("&") {
            let name = &raw_name[1..];
            modules.insert(
                name.to_string(),
                Rc::new(RefCell::new(Conjunction::new(name))),
            );
        } else {
            modules.insert(
                raw_name.to_string(),
                Rc::new(RefCell::new(Broadcast::new(raw_name))),
            );
        }
    }

    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let raw_name = parts.next().ok_or(Error::InvalidModuleLine)?;
        let from = if raw_name.starts_with("%") {
            &raw_name[1..]
        } else if raw_name.starts_with("&") {
            &raw_name[1..]
        } else {
            raw_name
        };

        let targets = parts
            .next()
            .ok_or(Error::InvalidModuleLine)?
            .split(",")
            .map(|s| s.trim());

        let from_module = modules.get(from).ok_or(Error::MissingModule)?.clone();
        for target in targets {
            if let Some(to_module) = modules.get(target) {
                to_module.borrow_mut().add_input(&from_module);
                from_module.borrow_mut().connect(to_module);
            } else {
                // Not defined yet. Create a connection-less broadcaster.
                let to_module: RcModule = Rc::new(RefCell::new(Broadcast::new(target)));
                modules.insert(target.to_string(), to_module.clone());
                to_module.borrow_mut().add_input(&from_module);
                from_module.borrow_mut().connect(&to_module);
            }
        }
    }

    Ok(modules)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

fn part1(input: &str) -> Result<(), Error> {
    let modules = parse(input)?;
    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let (signals_low, signals_high) = push_button_part1(&modules)?;
        low += signals_low;
        high += signals_high;
    }

    println!("Part 1: {}, {} -> {}", low, high, low * high);
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let modules = parse(input)?;
    let trigger_nodes = vec!["ph", "vn", "kt", "hn"];
    let mut cycle_lens = Vec::new();

    for trigger_node in trigger_nodes {
        for module in modules.values() {
            module.borrow_mut().reset();
        }

        let mut pushes: usize = 1;
        while !push_button_part2(&modules, trigger_node)? {
            pushes += 1;
        }

        println!("Cycle {}: {}", trigger_node, pushes);
        cycle_lens.push(pushes);
    }

    let mut result = cycle_lens[0];
    for i in 1..cycle_lens.len() {
        result = lcm(result, cycle_lens[i]);
    }
    println!("Part 2: {}", result);
    return Ok(());
}

fn main() -> Result<(), Error> {
    let input = include_str!("../rsc/input.txt");

    let start1 = Instant::now();
    part1(input)?;
    println!("Elapsed: {:.2?}\n", start1.elapsed());

    let start2 = Instant::now();
    part2(input)?;
    println!("Elapsed: {:.2?}", start2.elapsed());

    Ok(())
}
