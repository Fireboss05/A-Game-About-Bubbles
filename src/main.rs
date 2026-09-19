use bevy::{DefaultPlugins, app::prelude::*};

mod main_page;
use main_page::MainPagePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MainPagePlugin));
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);

    app.run();
}

fn setup() {
    println!("Hello, world!");
}

fn update() {
    // println!("Update");
}
