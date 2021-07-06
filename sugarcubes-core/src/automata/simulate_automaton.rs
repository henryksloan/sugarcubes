/// The full details of a configuration being simulated
pub trait Configuration: Clone {
    fn state(&self) -> u32;
}

/// Declares methods for controlling a simulation on an automaton
pub trait SimulateAutomaton {
    type ConfigurationType: Configuration;

    /// Get the list of initial configurations for a given string
    fn initial_configurations(&self, input: &str) -> Vec<Self::ConfigurationType>;

    /// Consumes a configuration and returns all possible subsequent configurations
    fn step(&self, configuration: Self::ConfigurationType) -> Vec<Self::ConfigurationType>;

    /// Consumes a list of configurations and returns the flat list of all possible subsequent configurations
    fn step_all(
        &self,
        configurations: Vec<Self::ConfigurationType>,
    ) -> Vec<Self::ConfigurationType>;

    /// Check whether an input is accepted,
    // Eventually, this should produce more details about the success/failure
    fn check_input(&self, input: &str) -> bool;
}
