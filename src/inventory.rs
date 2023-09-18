use crate::world_object::ItemType;
use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Inventory {
    pub items: HashMap<ItemType, usize>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: HashMap::default(),
        }
    }
    pub fn add(&mut self, item: ItemType, amount: usize) {
        *self.items.entry(item).or_insert(0) += amount;
        dbg!(&self.items);
    }
    pub fn cost(&mut self, item: ItemType, amount: usize) {
        let count = self.items.entry(item).or_default();
        *count -= amount;
        if *count <= 0 {
            self.items.remove_entry(&item);
        }
    }
}
