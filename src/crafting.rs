use crate::inventory::{Inventory, ItemType};
use bevy::{prelude::*, utils::HashMap};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CraftingBook::new())
            .add_systems(Update, test_crafting_system);
    }
}

#[derive(Resource)]
pub struct CraftingBook {
    craftable: Vec<CraftingRecipe>,
}

impl CraftingBook {
    pub fn new() -> Self {
        CraftingBook {
            craftable: vec![CraftingRecipe::create(ItemType::Axe)],
        }
    }
}

pub struct CraftingRecipe {
    pub needed: HashMap<ItemType, usize>,
    pub preducts: ItemType,
}

impl CraftingRecipe {
    pub fn create(preducts: ItemType) -> Self {
        let mut needed = HashMap::default();
        match preducts {
            ItemType::None => todo!(),
            ItemType::Arrow => todo!(),
            ItemType::Axe => {
                needed.insert(ItemType::Arrow, 1);
                needed.insert(ItemType::Twig, 2);
            }
            ItemType::Twig => todo!(),
        }
        CraftingRecipe { needed, preducts }
    }
}

pub fn test_crafting_system(
    mut inventory: Query<&mut Inventory>,
    keyboard_input: Res<Input<KeyCode>>,
    crafting_book: Res<CraftingBook>,
) {
    let mut inventory = inventory.single_mut();
    if keyboard_input.just_pressed(KeyCode::E) {
        info!("crafting");
        for recipe in crafting_book.craftable.iter() {
            if check_can_craft(&inventory, recipe) {
                info!("crafted: {:?} x1", recipe.preducts);
                cost_and_craft(&mut inventory, recipe);
            }
        }
    }
}

fn check_can_craft(inventory: &Inventory, recipe: &CraftingRecipe) -> bool {
    recipe
        .needed
        .iter()
        .all(|(item, cnt)| *inventory.items.get(item).get_or_insert(&0) >= cnt)
}

fn cost_and_craft(inventory: &mut Inventory, recipe: &CraftingRecipe) {
    recipe.needed.iter().for_each(|(&item, &cnt)| {
        inventory.cost(item, cnt);
    });
    inventory.add(recipe.preducts, 1);
}
