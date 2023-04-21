use std::time::Duration;

use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

use crate::{event::*, resources::*, systems::*};

pub fn game() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(500.0, 500.0),
                title: "Snake!".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_startup_system(spawn_snake)
        .add_systems(
            (
                snake_movement_input,
                snake_movement.run_if(on_timer(Duration::from_secs_f32(0.25))),
                spawn_food.run_if(on_timer(Duration::from_secs_f32(1.5))),
                food_eating,
                snake_growth,
                game_over,
            )
                .chain(),
        )
        .add_systems((size_scaling, position_translation).in_base_set(StartupSet::PostStartup))
        .run();
}
