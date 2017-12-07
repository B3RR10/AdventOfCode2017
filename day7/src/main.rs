extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Program {
    name: String,
    value: i32,
    children: Vec<String>,
}

impl Program {
    fn new(name: String, value: i32) -> Program {
        Program {
            name: name,
            value: value,
            children: vec![],
        }
    }

    fn with_child(&mut self, program: String) {
        self.children.push(program)
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_programs(lines: &Vec<String>) -> HashMap<String, Program> {
    let mut programs: HashMap<String, Program> = HashMap::new();
    let re = Regex::new(r"(\w+) \((\d+)\)(?: -> (.+))?").unwrap();
    for line in lines {
        let cap = re.captures(line).unwrap();
        let name = &cap.get(1).map_or("", |m| m.as_str());
        let value: i32 = cap.get(2).map_or(0, |m| m.as_str().parse().unwrap());
        let children_str = cap.get(3).map_or("", |m| m.as_str());
        let children_vec: Vec<&str> = children_str.split(", ").filter(|s| !s.is_empty()).collect();
        let mut prog = Program::new(String::from(*name), value);
        for child in children_vec {
            prog.with_child(String::from(child));
        }
        programs.insert(String::from(*name), prog);
    }
    programs
}

/* ------------------------- Part 1 ------------------------- */
fn get_root(lines: &Vec<String>) -> String {
    let programs = parse_programs(&lines);
    let mut lengths: HashMap<&str, i32> = HashMap::new();
    for (name, _) in &programs {
        let length = get_length(name, &programs);
        lengths.insert(&name, length);
    }
    let mut max_value = 0;
    let mut max_name = "";
    for (name, length) in &lengths {
        if *length > max_value {
            max_value = *length;
            max_name = name;
        }
    }
    String::from(max_name)
}

fn get_length(prog: &str, programs: &HashMap<String, Program>) -> i32 {
    let mut children_depth: Vec<i32> = vec![];
    let program = &programs[prog];
    if !programs[&program.name].has_children() {
        return 1;
    }
    for child in &program.children {
        children_depth.push(get_length(child, &programs));
    }
    1 + children_depth.iter().max().unwrap()
}

/* ------------------------- Part 2 ------------------------- */
fn get_right_weigth() {}

fn main() {
    let lines = lines_from_file("input.txt");
    let root = get_root(&lines);
    println!("Root: {}", root);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = vec![
            String::from("pbga (66)"),
            String::from("xhth (57)"),
            String::from("ebii (61)"),
            String::from("havc (66)"),
            String::from("ktlj (57)"),
            String::from("fwft (72) -> ktlj, cntj, xhth"),
            String::from("qoyq (66)"),
            String::from("padx (45) -> pbga, havc, qoyq"),
            String::from("tknk (41) -> ugml, padx, fwft"),
            String::from("jptl (61)"),
            String::from("ugml (68) -> gyxo, ebii, jptl"),
            String::from("gyxo (61)"),
            String::from("cntj (57)"),
        ];
        assert_eq!("tknk", get_root(&input));
    }
}
