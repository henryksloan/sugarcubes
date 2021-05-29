use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonConfiguration, FiniteAutomatonTransition},
    Configuration, SimulateAutomaton, EMPTY_STRING,
};

fn main() {
    // Create an NFA with an initial state, a normal state, and a final state
    let mut fa = FiniteAutomaton::default();
    let state0 = fa.automaton.add_new_state();
    fa.automaton.set_initial(state0);
    let state1 = fa.automaton.add_new_state();
    let state2 = fa.automaton.add_new_state();
    fa.automaton.set_final(state2, true);

    // Connect the initial state nondeterministically to each of the other states
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(state0, state1, 'x'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(state0, state2, EMPTY_STRING));

    // Simulate the NFA on a test string
    let configurations = fa.initial_configurations("xabc");
    let new_configurations: Vec<FiniteAutomatonConfiguration> = configurations
        .into_iter()
        .flat_map(|configuration| fa.step(configuration))
        .collect();
    for configuration in new_configurations {
        println!(
            "{} {}",
            configuration.state(),
            configuration.remaining_string
        );
    }
}
