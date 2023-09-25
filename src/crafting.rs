use crate::{inventory::Inventory, ui::CraftingButton, world_object::ItemType};
use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;
use std::fs;

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CraftingBook::from_path("assets/crafting_book_desc.ron"))
            .add_systems(Update, crafting_system);
    }
}

#[derive(Resource, Deserialize)]
pub struct CraftingBook {
    pub craftable: Vec<CraftingRecipe>,
}

impl CraftingBook {
    pub fn from_path(path: &str) -> Self {
        let desc_str = fs::read_to_string(path).unwrap();
        ron::de::from_str(&desc_str).unwrap()
    }
}

#[derive(Deserialize)]
pub struct CraftingRecipe {
    pub needed: HashMap<ItemType, usize>,
    pub preducts: ItemType,
}

impl CraftingRecipe {
    pub fn can_craft(&self, inventory: &Inventory) -> bool {
        self.needed
            .iter()
            .all(|(item, cnt)| *inventory.items.get(item).get_or_insert(&0) >= cnt)
    }
}

pub fn cost_and_craft(inventory: &mut Inventory, recipe: &CraftingRecipe) {
    recipe.needed.iter().for_each(|(&item, &cnt)| {
        inventory.cost(item, cnt);
    });
    inventory.add(recipe.preducts, 1);
}

fn crafting_system(
    interaction_query: Query<
        (&Interaction, &CraftingButton),
        (Changed<Interaction>, With<CraftingButton>),
    >,
    crafting_book: Res<CraftingBook>,
    mut inventory_query: Query<&mut Inventory>,
) {
    for (interaction, crafting_button) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                let crafting_index = crafting_button.0;
                let recipe = &crafting_book.craftable[crafting_index];
                let mut inventory = inventory_query.single_mut();
                if recipe.can_craft(&inventory) {
                    info!("crafted: {:?} ×1", recipe.preducts);
                    cost_and_craft(&mut inventory, recipe);
                } else {
                    info!("not enough to crafting the {:?}", recipe.preducts);
                }
            }
            _ => (),
        }
    }
}
