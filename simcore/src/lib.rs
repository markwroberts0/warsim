mod ecs;
mod systems;

pub use ecs::components::*;
pub use systems::*;

use bevy_ecs::prelude::*;

/// Simple event struct to represent a tick
#[derive(Debug)]
pub struct Tick;

/// Core simulation class
pub struct Simulation {
    world: World,
    current_time: f64,
}

impl Simulation {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Register all relevant components
        ecs::setup_components(&mut world);
        
        Simulation { 
            world, 
            current_time: 0.0,
        }
    }
    
    pub fn add_aircraft(&mut self, name: &str, x: f32, y: f32, z: f32, hdg: f32, ktas: f32) -> Entity {
        ecs::components::create_aircraft(&mut self.world, name, x, y, z, hdg, ktas)
    }
    
    pub fn run_until(&mut self, end_time: f64) {
        // Simple time stepping
        let dt = 1.0; // 1 second time steps
        
        while self.current_time < end_time {
            // Step forward by dt
            self.current_time += dt;
            
            // Run all systems
            systems::kinematics::propagate_system(&mut self.world, dt as f32);
            
            // Stop if we've reached or exceeded the end time
            if self.current_time >= end_time {
                break;
            }
        }
    }
    
    pub fn get_positions(&self) -> Vec<(Entity, (f32, f32, f32))> {
        ecs::components::get_all_positions(&self.world)
    }
}

// For ergonomics in Rust code
pub struct SimulationBuilder {
    sim: Simulation,
    duration: f64,
}

impl SimulationBuilder {
    pub fn new(duration: f64) -> Self {
        SimulationBuilder {
            sim: Simulation::new(),
            duration,
        }
    }
    
    pub fn add_aircraft(mut self, name: &str, x: f32, y: f32, z: f32, hdg: f32, ktas: f32) -> Self {
        self.sim.add_aircraft(name, x, y, z, hdg, ktas);
        self
    }
    
    pub fn run(mut self) -> SimulationResult {
        self.sim.run_until(self.duration);
        SimulationResult { sim: self.sim }
    }
}

pub struct SimulationResult {
    sim: Simulation,
}

impl SimulationResult {
    pub fn positions(&self) -> Vec<(Entity, (f32, f32, f32))> {
        self.sim.get_positions()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = SimulationBuilder::new(60.0)
            .add_aircraft("F-16", 0.0, 0.0, 8000.0, 90.0, 420.0)
            .add_aircraft("Su-35", 150000.0, 0.0, 8500.0, 270.0, 430.0)
            .run();
        
        let positions = result.positions();
        assert_eq!(positions.len(), 2);
    }
}
