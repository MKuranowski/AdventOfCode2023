// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, io::stdin, ops::Range};

#[derive(Debug, Clone, Copy, Default)]
pub struct Part {
    pub x: u16,
    pub m: u16,
    pub a: u16,
    pub s: u16,
}

impl Part {
    pub fn get(self, v: Value) -> u16 {
        match v {
            Value::X => self.x,
            Value::M => self.m,
            Value::A => self.a,
            Value::S => self.s,
        }
    }

    pub fn sum(self) -> usize {
        self.x as usize + self.m as usize + self.a as usize + self.s as usize
    }

    fn parse(x: &str) -> Self {
        let mut p = Part::default();
        let x = &x[1..x.len() - 1]; // get rid of enclosing '{' and '}'
        for part in x.split(',') {
            let (value_letter, level) = part.split_once('=').unwrap();
            let level = u16::from_str_radix(level, 10).unwrap();
            match value_letter {
                "x" => p.x = level,
                "m" => p.m = level,
                "a" => p.a = level,
                "s" => p.s = level,
                _ => panic!("invalid part {x:?}"),
            }
        }
        p
    }
}

#[derive(Debug, Clone)]
pub struct PartRange {
    pub x: Range<u16>,
    pub m: Range<u16>,
    pub a: Range<u16>,
    pub s: Range<u16>,
}

impl PartRange {
    pub fn get(&self, v: Value) -> Range<u16> {
        match v {
            Value::X => self.x.clone(),
            Value::M => self.m.clone(),
            Value::A => self.a.clone(),
            Value::S => self.s.clone(),
        }
    }

    pub fn with(&self, v: Value, range: Range<u16>) -> Self {
        let mut copy = self.clone();
        match v {
            Value::X => copy.x = range,
            Value::M => copy.m = range,
            Value::A => copy.a = range,
            Value::S => copy.s = range,
        };
        copy
    }

    pub fn total(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    X,
    M,
    A,
    S,
}

impl Value {
    fn parse(x: &str) -> Self {
        match x {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("invalid value {:?}", x),
        }
    }
}

#[derive(Debug)]
pub enum Reference {
    Accept,
    Reject,
    To(String),
}

impl Reference {
    fn parse(x: &str) -> Self {
        match x {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::To(x.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Condition {
    Unconditional,
    LessThan(Value, u16),
    GreaterThan(Value, u16),
}

impl Condition {
    pub fn partition(&self, range: PartRange) -> (Option<PartRange>, Option<PartRange>) {
        match self {
            Self::Unconditional => (Some(range), None),
            Self::LessThan(v, cutoff) => {
                let current = range.get(*v);
                if current.contains(cutoff) {
                    (
                        Some(range.clone().with(*v, current.start..*cutoff)),
                        Some(range.with(*v, *cutoff..current.end)),
                    )
                } else if cutoff < &current.start {
                    (None, Some(range))
                } else {
                    // cutoff >= current.end
                    (Some(range), None)
                }
            }
            Self::GreaterThan(v, cutoff) => {
                let current = range.get(*v);
                if current.contains(cutoff) {
                    (
                        Some(range.clone().with(*v, *cutoff + 1..current.end)),
                        Some(range.with(*v, current.start..*cutoff + 1)),
                    )
                } else if cutoff < &current.start {
                    (Some(range), None)
                } else {
                    // cutoff >= current.end
                    (None, Some(range))
                }
            }
        }
    }
}

impl Condition {
    pub fn test(&self, part: &Part) -> bool {
        match self {
            Self::Unconditional => true,
            Self::GreaterThan(value, cutoff) => part.get(*value) > *cutoff,
            Self::LessThan(value, cutoff) => part.get(*value) < *cutoff,
        }
    }

    fn parse(x: &str) -> Self {
        let value = Value::parse(&x[0..1]);
        let cutoff = u16::from_str_radix(&x[2..], 10).unwrap();
        match &x[1..2] {
            "<" => Self::LessThan(value, cutoff),
            ">" => Self::GreaterThan(value, cutoff),
            _ => panic!("invalid condition: {:?}", x),
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub condition: Condition,
    pub reference: Reference,
}

impl Rule {
    pub fn applies(&self, part: &Part) -> Option<&Reference> {
        if self.condition.test(&part) {
            Some(&self.reference)
        } else {
            None
        }
    }

    fn parse(x: &str) -> Self {
        let (condition, reference) = if let Some((condition_str, reference_str)) = x.split_once(':')
        {
            (
                Condition::parse(condition_str),
                Reference::parse(reference_str),
            )
        } else {
            (Condition::Unconditional, Reference::parse(x))
        };
        Self {
            condition,
            reference,
        }
    }
}

#[derive(Debug)]
pub struct Workflow(Vec<Rule>);

impl Workflow {
    pub fn apply(&self, part: &Part) -> &Reference {
        for rule in &self.0 {
            if let Some(reference) = rule.applies(part) {
                return reference;
            }
        }
        panic!("couldn't match a part to a workflow - missing unconditional rule?");
    }

    pub fn apply_range(&self, mut range: PartRange) -> Vec<(PartRange, &Reference)> {
        let mut result = Vec::default();

        for rule in &self.0 {
            let (conforming, non_conforming) = rule.condition.partition(range);

            if let Some(conforming) = conforming {
                result.push((conforming, &rule.reference));
            }

            if let Some(non_conforming) = non_conforming {
                range = non_conforming;
            } else {
                break;
            }
        }

        result
    }

    fn parse(x: &str) -> Self {
        let x = &x[1..x.len() - 1]; // get rid of enclosing '{' and '}'
        Self(x.split(',').map(Rule::parse).collect())
    }
}

#[derive(Debug, Default)]
pub struct System(HashMap<String, Workflow>);

impl System {
    pub fn is_accepted(&self, part: &Part) -> bool {
        let mut to_apply: &str = "in";
        loop {
            match self.0.get(to_apply).unwrap().apply(part) {
                Reference::Accept => return true,
                Reference::Reject => return false,
                Reference::To(next_name) => to_apply = next_name,
            }
        }
    }

    pub fn count_accepted(&self, initial: PartRange) -> usize {
        let mut result = 0;
        let initial_reference = Reference::To("in".to_string());
        let mut candidates = vec![(initial, &initial_reference)];

        while let Some((range, reference)) = candidates.pop() {
            match reference {
                Reference::Accept => {
                    result += range.total();
                }
                Reference::Reject => {}
                Reference::To(next) => {
                    let w = self.0.get(next).unwrap();
                    candidates.append(&mut w.apply_range(range));
                }
            }
        }

        result
    }
}

pub fn load_input() -> (System, Vec<Part>) {
    let mut system = System::default();
    let mut parts = Vec::default();
    let mut parsing_parts = false;

    for line in stdin().lines() {
        let line = line.expect("failed to read from stdin");

        if line.is_empty() {
            parsing_parts = true;
        } else if parsing_parts {
            parts.push(Part::parse(&line));
        } else {
            let (name, workflow) = line.split_at(line.find('{').unwrap());
            system.0.insert(name.to_string(), Workflow::parse(workflow));
        }
    }

    (system, parts)
}
