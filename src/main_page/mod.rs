use bevy::prelude::*;

pub mod components;
pub mod styles;


pub struct MainPagePlugin;

impl Plugin for MainPagePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, main_page);
    }

}

fn main_page() {
    println!("main_page");
}

