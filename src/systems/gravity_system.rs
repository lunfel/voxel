use bevy::prelude::*;

use crate::components::gravity::Gravity;

fn apply_gravity(query: &mut Query<(&Gravity, &mut Transform)>) {
    for (gravity, mut transform) in query {
        transform.translation = transform.translation + gravity.0;
    }
}
