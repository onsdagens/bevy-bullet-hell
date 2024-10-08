use bevy::prelude::*;

#[derive(Component)]
pub struct ConfigComponent;

#[derive(Resource, Reflect)]
pub struct ConfigResource {
    pub gamepad: bool,
}

pub fn setup(mut commands: Commands) {
    // load from JSON via serde?
    let config = ConfigResource { gamepad: false };

    println!("Config_gamepad: {}", config.gamepad);
    commands.insert_resource(config);
}
