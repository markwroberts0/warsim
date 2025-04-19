use bevy_ecs::prelude::*;
use crate::ecs::components::Kinematics;

/// Propagate all entities with kinematics components forward in time
pub fn propagate_system(world: &mut World, dt: f32) {
    let mut query = world.query::<&mut Kinematics>();
    
    for mut kinematics in query.iter_mut(world) {
        // Simple linear propagation: position += velocity * time
        let velocity = kinematics.vel;
        kinematics.pos += velocity * dt;
    }
} 