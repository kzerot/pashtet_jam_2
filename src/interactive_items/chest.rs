use bevy::{utils::HashMap, prelude::Component};
use rand::{Rng, random};

#[derive(Clone, Copy)]
pub enum ItemType {
    Energy,
    Weapon,
    Turret
}


#[derive(Clone, Copy)]
pub struct Item {
    pub item_type: ItemType,
    pub count: u32
}

#[derive(Component)]
pub struct Chest {
    items: Vec<Item>,
    pub opened: bool
}

impl Chest {
    pub fn get_items(&mut self) -> Vec<Item> {
        let res = self.items.clone();
        self.items.clear();
        self.opened = false;
        res
    }
    
    pub fn generate(&mut self) {
        //energy
        let energy = Item {
            count: rand::thread_rng().gen_range(0..100),
            item_type: ItemType::Energy
        };
        self.items.push(energy);
        if rand::random::<f32>() >= 0.8 {
            let energy = Item {
                count: 1,
                item_type: ItemType::Weapon
            };
        }
        if rand::random::<f32>() >= 0.6 {
            let energy = Item {
                count: 1,
                item_type: ItemType::Turret
            };
        }
    }
}