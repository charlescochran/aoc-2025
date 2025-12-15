use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;

// #[derive(Debug)]
struct Machine {
    n: usize,
    goal: u32,
    buttons: Vec<u32>,
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Machine(n={}, goal={:0w$b}, buttons={:?})",
            self.n,
            self.goal,
            self.buttons
                .iter()
                .map(|b| format!("{:0w$b}", b, w = self.n))
                .collect::<Vec<_>>(),
            w = self.n,
        )
    }
}

fn parse(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    for line in input.lines() {
        let mut n = 0;
        let mut goal = 0;
        let mut buttons = Vec::new();
        for group in line.split(" ") {
            match &group.chars().next().unwrap() {
                '[' => {
                    for c in group[1..group.len() - 1].chars() {
                        goal = (goal << 1) + if c == '#' { 1 } else { 0 };
                        n += 1;
                    }
                }
                '(' => {
                    buttons.push(
                        group[1..group.len() - 1]
                            .split(",")
                            .map(|c| c.parse::<usize>().unwrap())
                            .fold(0, |acc, x| acc + (1 << (n - x - 1))),
                    );
                }
                '{' => {}
                _ => unreachable!(),
            }
        }
        machines.push(Machine { n, goal, buttons });
    }
    machines
}

fn which_buttons(n: usize, light_idx: usize, buttons: &Vec<u32>) -> HashSet<usize> {
    // Return the indices of buttons that affect the light at light_idx
    buttons
        .iter()
        .enumerate()
        .filter_map(|(i, &button)| ((button & (1 << (n - light_idx - 1))) != 0).then_some(i))
        .collect()
}

fn explore(
    light_idx: usize,
    pressed: HashSet<usize>,
    unpressed: HashSet<usize>,
    m: &Machine,
) -> u32 {
    let mut ans = u32::MAX;
    let button_indices = which_buttons(m.n, light_idx, &m.buttons);
    let light_on = if m.goal & (1 << (m.n - light_idx - 1)) == 0 {
        0
    } else {
        1
    };
    for press in button_indices
        .iter()
        .powerset()
        .filter(|s: &Vec<_>| s.len() % 2 == light_on)
    {
        let press: HashSet<usize> = press.into_iter().copied().collect();
        let unpress: HashSet<_> = button_indices.difference(&press).copied().collect();
        let pressed2: HashSet<_> = pressed.union(&press).copied().collect();
        let unpressed2: HashSet<_> = unpressed.union(&unpress).copied().collect();
        // If we've reached a contradiction, stop exploring this branch
        if !pressed2.is_disjoint(&unpressed2) {
            continue;
        }
        if light_idx + 1 == m.n {
            // Reached the end?
            if pressed2 // Valid solution --> return it
                .iter()
                .map(|&button_idx| m.buttons[button_idx])
                .fold(m.goal, |acc, x| acc ^ x)
                == 0
            {
                return pressed2.len() as u32;
            } else {
                // Invalid solution --> skip
                continue;
            }
        }
        // Haven't reached the end --> recursively explore this branch
        ans = ans.min(explore(light_idx + 1, pressed2, unpressed2, m));
    }
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let machines = parse(&input);

    let start = std::time::Instant::now();
    let ans1: u32 = machines
        .iter()
        .map(|m| explore(0, HashSet::new(), HashSet::new(), m))
        .sum();
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    // println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
