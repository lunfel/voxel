use bevy::prelude::*;

pub struct LoggingPlugin;

impl Plugin for LoggingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LogIntervalTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, update_log_timer);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct LogIntervalTimer(Timer);

fn update_log_timer(time: Res<Time>, mut interval_timer: ResMut<LogIntervalTimer>) {
    interval_timer.tick(time.delta());
}