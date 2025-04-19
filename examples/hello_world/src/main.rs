use simcore::{SimulationBuilder};

fn main() {
    println!("Starting simple dogfight simulation");
    
    // Create a simulation with two aircraft
    let result = SimulationBuilder::new(60.0)
        .add_aircraft("F-16", 0.0, 0.0, 8000.0, 90.0, 420.0)
        .add_aircraft("Su-35", 150000.0, 0.0, 8500.0, 270.0, 430.0)
        .run();
    
    // Print the final positions
    println!("Simulation completed. Final positions:");
    for (entity, (x, y, z)) in result.positions() {
        println!("Entity {:?}: Position = ({:.1}, {:.1}, {:.1})", entity, x, y, z);
    }
}
