use pyo3::prelude::*;
use pyo3::types::PyList;
use simcore::{SimulationBuilder, SimulationResult};

/// SimulationResult Python wrapper
#[pyclass]
struct PySimulationResult {
    inner: SimulationResult,
}

#[pymethods]
impl PySimulationResult {
    /// Get positions as a list of tuples [(x, y, z), ...]
    #[getter]
    fn positions(&self, py: Python) -> PyResult<PyObject> {
        let positions = self.inner.positions();
        let result = PyList::empty(py);
        
        for (_, (x, y, z)) in positions {
            result.append((x, y, z))?;
        }
        
        Ok(result.into())
    }
}

/// Scenario class for Python
#[pyclass]
struct Scenario {
    duration: f64,
    aircraft: Vec<(String, f32, f32, f32, f32, f32)>, // name, x, y, z, hdg, ktas
}

#[pymethods]
impl Scenario {
    /// Create a new scenario
    #[new]
    fn new(duration: f64) -> Self {
        Scenario {
            duration,
            aircraft: Vec::new(),
        }
    }
    
    /// Add an aircraft to the scenario
    #[pyo3(text_signature = "($self, name, x, y, z, hdg, ktas)")]
    fn add_aircraft(&mut self, name: String, x: f32, y: f32, z: f32, hdg: f32, ktas: f32) -> &Self {
        self.aircraft.push((name, x, y, z, hdg, ktas));
        self
    }
    
    /// Run the simulation
    #[pyo3(text_signature = "($self, trials=1)")]
    fn run(&self, trials: Option<usize>) -> PyResult<PySimulationResult> {
        let trials = trials.unwrap_or(1);
        
        if trials != 1 {
            // For now, we ignore the trials count
            // Later we'll use Rayon for parallel runs
        }
        
        // Create a new simulation
        let mut builder = SimulationBuilder::new(self.duration);
        
        // Add all aircraft
        for (name, x, y, z, hdg, ktas) in &self.aircraft {
            builder = builder.add_aircraft(name, *x, *y, *z, *hdg, *ktas);
        }
        
        // Run the simulation
        let result = builder.run();
        
        Ok(PySimulationResult { inner: result })
    }
}

/// Python module
#[pymodule]
fn simcore_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Scenario>()?;
    Ok(())
} 