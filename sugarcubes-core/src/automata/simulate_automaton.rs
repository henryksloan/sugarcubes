/// The full details of a configuration being simulated
pub trait Configuration {
    fn state(&self) -> u32;
}

/// Declares methods for controlling a simulation on an automaton
pub trait SimulateAutomaton {
    type ConfigurationType: Configuration;

    /// Get the list of initial configurations for a given string
    fn initial_configurations(&self, input: &str) -> Vec<Self::ConfigurationType>;

    /// Consumes a configuration and returns all possible subsequent configurations
    fn step(&self, configuration: Self::ConfigurationType) -> Vec<Self::ConfigurationType>;
}
