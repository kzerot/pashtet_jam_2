use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::{self, FontId, RichText, Color32, Frame} };

use crate::{GameState, characters::{base_character::Hp, player::{Player, Energy, Weapon, Inventoty}}, map::DayNight};


pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiLog>()
            .add_systems(Update, ui.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct UiLog {
    pub last_message: String,
    pub message_time_stamp: f64
}

pub fn ui(    
    time: Res<Time>,
    mut contexts: EguiContexts,
    ui_log: Res<UiLog>,
    day_night: Res<DayNight>,
    inventory: Res<Inventoty>,
    query: Query<(&Hp, &Energy, &Weapon), With<Player>>
){
    let current_time = time.elapsed_seconds_f64();
    let (hp, energy, weapon) = query.single();

    egui::TopBottomPanel::bottom("Down")
        .frame(Frame{
            fill: Color32::from_rgba_unmultiplied(255, 255, 255, 30),
            ..Default::default()
        })
        .default_height(300.0)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui|{

            ui.vertical_centered_justified(|ui|{
                if hp.0 > 0.0 {
                    let color = if hp.0 > 70.0 {
                        Color32::DARK_GREEN
                    } else if hp.0 > 35.0 {
                        Color32::GOLD
                    } else {
                        Color32::DARK_RED
                    };
    
                    ui.label(RichText::new(format!("Your health status: {} %", hp.0 as i32)).color(color).font(FontId::monospace(24.0)));
                } else {
                    ui.label(RichText::new(format!("You dead")).color(Color32::DARK_RED).font(FontId::monospace(24.0)));
                }
                ui.label(RichText::new(format!("Energy left: {}/500", energy.0)).font(FontId::monospace(24.0)).color(Color32::BLACK));
                ui.label(RichText::new(format!("Current weapon: {}", weapon.name)).font(FontId::monospace(24.0)).color(Color32::BLACK));
                if let Some(turret) = &inventory.turret {
                    ui.label(RichText::new(format!("Turret equiped: {}, E to place", turret.name)).font(FontId::monospace(24.0)).color(Color32::BLACK));
                }
            });
 
        });

    if ui_log.last_message.len() > 0 && current_time - ui_log.message_time_stamp < 5.0 {
        egui::TopBottomPanel::bottom("Messages")
        .frame(Frame{
            fill: Color32::from_rgba_unmultiplied(255, 255, 255, 30),
            ..Default::default()
        })
        .default_height(30.0)
        .exact_height(30.0)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui|{
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(ui_log.last_message.clone()).color(Color32::BLACK).font(FontId::monospace(24.0)));
            });
        });
    }

    
    egui::TopBottomPanel::top("UpPanel")
        .frame(Frame{
            fill: Color32::from_rgba_unmultiplied(255, 255, 255, 30),
            ..Default::default()
        })
        .default_height(300.0)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui|{
            ui.horizontal_wrapped(|ui|{
                if !day_night.is_night {
                    let time_sec_day = day_night.full_day_time * day_night.current_day_time - day_night.time;
                    let time_min_day = (time_sec_day / 60.0) as i32;
                    
                    ui.label(RichText::new(format!("It's day. You are more or less safe. Day alive: {}", day_night.day)).font(FontId::monospace(20.0)).color(Color32::BLACK));
                    ui.label(RichText::new(
                        format!("Light time left: {}:{}", time_min_day, time_sec_day as i32 - time_min_day * 60)
                    ).font(FontId::monospace(20.0)).color(Color32::BLACK));
                } else {
                    let time_sec_night = day_night.full_day_time - day_night.time;
                    let time_min_night = (time_sec_night / 60.0) as i32;
                    ui.label(RichText::new(format!("It's night. You are in danger")).font(FontId::monospace(20.0)).color(Color32::BLACK));
                    ui.label(RichText::new(
                        format!("Night time left: {}:{}", time_min_night, time_sec_night as i32 - time_min_night * 60)
                    ).font(FontId::monospace(20.0)).color(Color32::BLACK));
                }
                
            })
        });
}