use bevy::{utils::HashMap, prelude::{Component, Commands, Res, Input, KeyCode, Query, Transform, With, Handle, Image, ResMut, info}, sprite::Sprite, render::render_resource::Texture, time::Time};
use rand::{Rng, random, seq::SliceRandom};

use crate::{loading::TextureAssets, characters::player::{Player, Energy, Weapon, WeaponPattern, TemporaryItems, Inventoty}, ui::UiLog};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WEAPONS : Vec<Weapon> =vec![
        Weapon { name: "Fast Eradictor".into(), pattern: WeaponPattern::Single, cd: 0.1, from_shot: 0.0},
        Weapon { name: "Double Penetrator".into(), pattern: WeaponPattern::TwoShot, cd: 0.15, from_shot: 0.0},
        Weapon { name: "Six Paths Destroyer".into(), pattern: WeaponPattern::SixRay, cd: 0.4, from_shot: 0.0},
        Weapon { name: "Protector MK2".into(), pattern: WeaponPattern::SixAround, cd: 0.3, from_shot: 0.0},
        Weapon { name: "Protector MK4".into(), pattern: WeaponPattern::ManyAround, cd: 0.2, from_shot: 0.0},
    ];
}



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

#[derive(Component, Default)]
pub struct Chest {
    items: Vec<Item>,
    pub opened: bool
}

impl Chest {
    pub fn get_items(&mut self) -> Vec<Item> {
        let res = self.items.clone();
        self.items.clear();
        self.opened = true;
        res
    }
    
    pub fn generate(&mut self) {
        //energy
        let energy = Item {
            count: rand::thread_rng().gen_range(0..100),
            item_type: ItemType::Energy
        };
        self.items.push(energy);
        let r = rand::random::<f32>();
        if r >= 0.8 {
            let item = Item {
                count: 1,
                item_type: ItemType::Turret
            };
            self.items.push(item);
        } else if r <= 0.3 {
            let item = Item {
                count: 1,
                item_type: ItemType::Weapon,
            };
            self.items.push(item);
        }
    }
}

pub fn open_chest(
    time: Res<Time>,
    mut temporary: ResMut<TemporaryItems>,
    mut inventory: ResMut<Inventoty>,
    mut ui_log: ResMut<UiLog>,
    textures: Res<TextureAssets>,
    keys : Res<Input<KeyCode>>,
    mut query_player: Query<(&Transform, &mut Energy), With<Player>>,
    mut query: Query<(&Transform, &mut Chest, &mut Handle<Image>)>
) {
    let (player_tr, mut energy) = query_player.single_mut();
    let player_pos = player_tr.translation;
    for (transform, mut chest, mut texture) in query.iter_mut() {
        if !chest.opened && player_pos.distance_squared(transform.translation) <= 30.0 * 30.0 {
            if keys.just_pressed(KeyCode::F){
                *texture = textures.texture_chest_opened.clone();
                  let times = time.elapsed_seconds_f64();
                let items = chest.get_items();
                let mut message = "New items: ".to_string();
                for item in items.iter() {
                    match item.item_type {
                        ItemType::Energy => {
                            energy.0 += item.count as i32;
                            energy.0 = energy.0.clamp(0, 512);
                            let add = format!("{} energy", item.count);
                            message += &add;
                        },
                        ItemType::Weapon => {
                            let random_weapon = WEAPONS.choose(&mut rand::thread_rng()).unwrap();
                            temporary.weapon = Some(random_weapon.clone());
                            temporary.timestamp = time.elapsed_seconds_f64();
                            message += &format!("Found {}, press R to change", random_weapon.name);
                        },
                        ItemType::Turret => {
                            let random_weapon = WEAPONS.choose(&mut rand::thread_rng()).unwrap();
                            if inventory.turret.is_none(){
                                inventory.turret = Some(random_weapon.clone());
                                message += &format!("Found turret {}", random_weapon.name);
                            } else {
                                temporary.turret = Some(random_weapon.clone());
                                temporary.timestamp = time.elapsed_seconds_f64();
                                message += &format!("New turret {}, press R to change", random_weapon.name);
                            }

                        },
                    }
                }
                ui_log.last_message = message;
                ui_log.message_time_stamp = times;
                break;
            }
        }
    }
}