pub mod components {
    use bevy_ecs::prelude::*;
    use bevy_math::Vec3;
    
    #[derive(Component, Debug)]
    pub struct Kinematics {
        pub pos: Vec3,
        pub vel: Vec3,
    }
    
    #[derive(Component, Debug)]
    pub struct Aircraft {
        pub name: String,
        pub heading: f32,
        pub speed_ktas: f32,
    }
    
    pub fn create_aircraft(world: &mut World, name: &str, x: f32, y: f32, z: f32, hdg: f32, ktas: f32) -> Entity {
        // Convert heading (degrees) and speed to velocity vector
        let hdg_rad = hdg.to_radians();
        let speed_mps = ktas * 0.514444; // Convert knots to meters per second
        
        let vel = Vec3::new(
            speed_mps * hdg_rad.sin(),
            speed_mps * hdg_rad.cos(),
            0.0, // Initially zero vertical speed
        );
        
        world.spawn((
            Kinematics {
                pos: Vec3::new(x, y, z),
                vel,
            },
            Aircraft {
                name: name.to_string(),
                heading: hdg,
                speed_ktas: ktas,
            },
        )).id()
    }
    
    // Return positions of all entities with kinematics components
    pub fn get_all_positions(world: &World) -> Vec<(Entity, (f32, f32, f32))> {
        let mut positions = Vec::new();
        
        // Collect positions manually without using query.iter directly
        for (entity, kinematics) in world.iter_entities().filter_map(|entity| {
            let kinematics = world.get::<Kinematics>(entity.id())?;
            Some((entity.id(), kinematics))
        }) {
            positions.push((entity, (kinematics.pos.x, kinematics.pos.y, kinematics.pos.z)));
        }
        
        positions
    }
}

use bevy_ecs::prelude::World;

pub fn setup_components(_world: &mut World) {
    // Register any components that need specific initialization
    // This would be used if we needed to set up special resources
} 