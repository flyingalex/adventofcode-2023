use std::collections::HashMap;
use std::path::Path;
use std::vec;
use crate::utils::read_lines;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Type {
    Flip,
    Conjunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    t: Type,
    direction: Vec<String>,
    is_on: bool,
    inputs: HashMap<String, bool>,
}

fn format_data(file: &Path) -> (HashMap<String, Module>, Vec<(String, String, bool)>) {
    let mut modules = HashMap::new();
    let mut start = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let line = strs.split("->").map(|s| s.trim()).collect::<Vec<&str>>();
            if strs.contains("broadcaster") {
                start = line[1].split(',').map(|s| ("broadcaster".to_string(), s.trim().to_string(), false)).collect();
            } else {
                let t = line[0].chars().next().unwrap();
                let name = line[0].chars().skip(1).collect::<String>();
                let mut typ = Type::Conjunction;
                if t == '%' {
                    typ = Type::Flip;
                }
                modules.insert(name, Module {
                    t: typ,
                    direction: line[1].split(',').map(|s| s.trim().to_string()).collect(),
                    is_on: false,
                    inputs: HashMap::new(),
                });
            }
        });
    let mut module_clone = modules.clone();
    for module in modules.iter() {
        if module.1.t == Type::Conjunction {
            for mo in modules.iter() {
                if mo.1.direction.contains(module.0) {
                    module_clone.get_mut(module.0).unwrap().inputs.insert(mo.0.clone(), false);
                }
            }
        }
    }
    (module_clone, start)
}

// start, bool save high(true) or low(false) pulse
fn get_pulses_numbers(modules: &mut HashMap<String, Module>, start: Vec<(String, String, bool)>) -> u64 {
    let mut high_pulse: u64 = 0;
    let mut low_pulse: u64 = 0;

    for _ in 0..1000 {
        low_pulse += 1;
        let mut current_positions = start.clone();
        let mut next_round = vec![];
        loop {
            for pos in current_positions.iter() {
                if pos.2 {
                    high_pulse += 1;
                } else {
                    low_pulse += 1;
                }

                if let Some(module) = modules.get_mut(&pos.1) {
                    if module.t == Type::Flip && !pos.2 {
                        module.is_on = !module.is_on;
                        module.direction.iter().for_each(|s| {
                            next_round.push((pos.1.clone(), s.clone(), module.is_on));
                        });
                    }

                    if module.t == Type::Conjunction {
                        module.inputs.insert(pos.0.clone(), pos.2);
                        // if it remembers high pulses for all inputs, it sends a low pulse
                        let sending_pulse = !module.inputs.iter().all(|i| *i.1);
                        module.direction.iter().for_each(|s| {
                            next_round.push((pos.1.clone(), s.clone(), sending_pulse));
                        });
                    }
                }
            }
            if next_round.is_empty() {
                break;
            }
            current_positions = next_round;
            next_round = vec![];
        }
    }
    high_pulse * low_pulse
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn day20_1_test() {
        let (mut modules, start) = format_data(Path::new("src/day20/day20_input_test.txt"));
        let result = get_pulses_numbers(&mut modules, start);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn day20_1_answer() {
        let (mut modules, start) = format_data(Path::new("src/day20/day20_input.txt"));
        let result = get_pulses_numbers(&mut modules, start);
        assert_eq!(result, 817896682);
    }

    #[test]
    fn day20_2_test() {
        let (mut modules, start) = format_data(Path::new("src/day20/day20_input_test2.txt"));
        let result = get_pulses_numbers(&mut modules, start);
        assert_eq!(result, 11687500);
    }

    #[test]
    fn day20_2_answer() {
        let (mut modules, start) = format_data(Path::new("src/day20/day20_input.txt"));
        let result = get_pulses_numbers(&mut modules, start);
        assert_eq!(result, 391132);
    }
}

