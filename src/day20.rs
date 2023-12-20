// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fmt::Debug;
use std::io::stdin;

pub type ModuleID = u16;
const BROADCASTER_ID: ModuleID = 0;

#[derive(Debug)]
struct IDGenerator {
    cache: HashMap<String, ModuleID>,
    counter: ModuleID,
}

impl Default for IDGenerator {
    fn default() -> Self {
        Self {
            cache: HashMap::default(),
            counter: 1,
        }
    }
}

impl IDGenerator {
    fn get(&mut self, name: &str) -> ModuleID {
        // XXX: when generating an id we cut off the first character (to skip over '%' or '&'),
        //      but this turns "broadcaster" to "roadcaster"
        if name == "broadcaster" || name == "roadcaster" {
            return BROADCASTER_ID;
        } else if let Some(id) = self.cache.get(name) {
            return *id;
        } else {
            let id = self.counter;
            self.cache.insert(name.to_string(), id);
            self.counter += 1;
            return id;
        }
    }
}

pub trait Collector {
    fn on_click(&mut self) {}
    fn on_pulse(&mut self, _from: ModuleID, _to: ModuleID, _is_high: bool) {}
}

#[derive(Debug)]
enum ModuleKind {
    Noop,
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<ModuleID, bool>),
}

impl Default for ModuleKind {
    fn default() -> Self {
        Self::Noop
    }
}

#[derive(Debug)]
pub struct Module {
    id: ModuleID,
    children: Vec<ModuleID>,
    kind: ModuleKind,
}

impl Module {
    pub fn receive(&mut self, from: ModuleID, is_high: bool) {
        match self.kind {
            ModuleKind::Noop => {}
            ModuleKind::Broadcast => {}
            ModuleKind::FlipFlop(ref mut state) => {
                if !is_high {
                    *state = !*state;
                }
            }
            ModuleKind::Conjunction(ref mut inputs) => {
                inputs.insert(from, is_high);
            }
        }
    }

    pub fn send<C: Collector>(
        &mut self,
        received_high: bool,
        modules: &mut [Module],
        collector: &mut C,
    ) {
        match self.kind {
            ModuleKind::Noop => {}

            ModuleKind::Broadcast => self.send_to_children(received_high, modules, collector),

            ModuleKind::FlipFlop(state) => {
                if !received_high {
                    self.send_to_children(state, modules, collector)
                }
            }

            ModuleKind::Conjunction(ref inputs) => {
                let send_high = inputs.values().any(|&v| !v);
                self.send_to_children(send_high, modules, collector);
            }
        }
    }

    fn send_to_children<C: Collector>(
        &mut self,
        send_high: bool,
        modules: &mut [Module],
        collector: &mut C,
    ) {
        for &child in &self.children {
            modules[child as usize].receive(self.id, send_high);
        }
        for &child in &self.children {
            collector.on_pulse(self.id, child, send_high);

            // Silence the borrow checker
            let child_ptr = (&mut modules[child as usize]) as *mut Module;
            let child = unsafe { &mut *child_ptr };

            child.send(send_high, modules, collector);
        }
    }
}

#[derive(Debug)]
pub struct System {
    pub modules: Vec<Module>,
    pub name_to_id: HashMap<String, u16>,
}

impl System {
    pub fn click_button<C: Collector>(&mut self, collector: &mut C) {
        // Silence the borrow checker
        let broadcast_ptr = (&mut self.modules[0]) as *mut Module;
        let broadcast = unsafe { &mut *broadcast_ptr };
        collector.on_click();
        collector.on_pulse(BROADCASTER_ID, BROADCASTER_ID, false);
        broadcast.send(false, &mut self.modules, collector);
    }
}

pub fn load_input() -> System {
    let mut id_generator = IDGenerator::default();
    let mut modules: Vec<Module> = Vec::default();

    for line in stdin().lines() {
        let line = line.expect("failed to read from stdin");
        let (id_str, destinations_str) = line.split_once(" -> ").unwrap();

        // Get current ID. This turns "broadcaster" to "roadcaster", but generator handles that
        let id = id_generator.get(&id_str[1..]);

        // Get destinations ID
        let destinations = if destinations_str.is_empty() {
            vec![]
        } else {
            destinations_str
                .split(", ")
                .map(|id| id_generator.get(id))
                .collect::<Vec<_>>()
        };

        // Ensure enough modules exist
        for missing_id in (modules.len() as u16)..id_generator.counter {
            modules.push(Module {
                id: missing_id,
                children: Vec::default(),
                kind: ModuleKind::Noop,
            });
        }

        // Set children
        modules[id as usize].children = destinations;

        // Set kind
        match &id_str[0..1] {
            "b" => {
                modules[0].kind = ModuleKind::Broadcast;
            }
            "%" => {
                modules[id as usize].kind = ModuleKind::FlipFlop(false);
            }
            "&" => {
                modules[id as usize].kind = ModuleKind::Conjunction(HashMap::default());
            }
            _ => panic!("Unrecognized id: {id_str:?}"),
        }
    }

    // Gather inputs to conjunction gates
    let mut inputs: HashMap<ModuleID, Vec<ModuleID>> = HashMap::default();
    for (id, module) in modules.iter().enumerate() {
        for &child_id in &module.children {
            match modules[child_id as usize].kind {
                ModuleKind::Conjunction(_) => {
                    inputs.entry(child_id).or_default().push(id as ModuleID);
                }
                _ => {}
            }
        }
    }
    for (module_id, children) in inputs.drain() {
        match modules[module_id as usize].kind {
            ModuleKind::Conjunction(ref mut inputs) => {
                for child in children {
                    inputs.insert(child, false);
                }
            }
            _ => {}
        }
    }

    // Convert names to IDs
    let name_to_id = id_generator.cache.drain().collect();

    // Return inputs
    System {
        modules,
        name_to_id,
    }
}
