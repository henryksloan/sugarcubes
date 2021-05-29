/// The full details of a configuration being simulated
pub trait Configuration {
    fn state(&self) -> u32;
}

/// Declares methods for controlling a simulation on an automaton
pub trait SimulateAutomaton {
    type ConfigurationType: Configuration;

    /// Consumes a configuration and returns all possible subsequent configurations
    fn step(&self, configuration: Self::ConfigurationType) -> Vec<Self::ConfigurationType>;
}
