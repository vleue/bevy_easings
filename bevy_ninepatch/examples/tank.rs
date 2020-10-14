use bevy::diagnostic::*;
use bevy::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(deep.system())
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(UiCameraComponents::default());

    commands
        .spawn(NodeComponents {
            style: Style {
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(1., false));
}

fn deep(mut commands: Commands, entity: Entity, timer: &Timer) {
    if timer.just_finished {
        commands
            .spawn(NodeComponents {
                style: Style {
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Timer::from_seconds(1., false));
        let child = commands.current_entity().unwrap();

        commands.push_children(entity, &[child]);
    }
}
