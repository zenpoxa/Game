#[allow(dead_code)]
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window{
                    title: String::from("Invasion !"),
                    position: WindowPosition::Centered(MonitorSelection::Current),
                    resolution: Vec2::new(512.0, 512.0).into(),
                    ..Default::default()
                }),..Default::default()
            })
        )
        .run();
}
