"""
Basic test script for simcore package.
"""
import simcore

if __name__ == "__main__":
    print("SimCore version:", simcore.__version__ if hasattr(simcore, "__version__") else "0.1.0")
    print("Successfully imported simcore module!")
    
    # Create a simple scenario
    scenario = simcore.Scenario(duration=300)
    scenario.add_aircraft("F-16", x=0, y=0, z=8000, hdg=90, ktas=420)
    scenario.add_aircraft("Su-35", x=150000, y=0, z=8500, hdg=270, ktas=430)
    
    print("Created scenario with two aircraft")
    print("Running simulation...")
    result = scenario.run(trials=1)
    
    print("Simulation complete, retrieving positions...")
    positions = result.positions
    print(f"Final positions table:\n{positions}") 