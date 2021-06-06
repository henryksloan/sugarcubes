# Sugarcubes
Sugarcubes is a library and application for automata and formal languages. It is inspired by JFLAP, and is intended to eventually to be an alternative to JFLAP.

![Screenshot of a finite automaton in Sugarcubes](screenshots/Sugarcubes.png)

### Usage
Double-click the background to add a state.  
Double-click a state and drag onto another state (or the same state) to add a transition.   
Click and drag a state to move it.

## Building
To build the desktop frontend, run:

`cargo run --release`

### WASM
To build for WASM, run:

```
# Add the WASM compilation target if you haven't already
rustup target add wasm32-unknown-unknown

cargo build --release --target wasm32-unknown-unknown
```

This will produce a WASM binary in `target/wasm32-unknown-unknown/release/sugarcubes.wasm`, which can be placed in the `docs` directory and hosted as described [in the miniquad docs](https://github.com/not-fl3/miniquad/#wasm).

# Progress
Sugarcubes is in a very early stage of development, having only basic support for editing and simulating finite automata. The first MVP should be a fully usable finite automaton simulator and editor.

## Next steps
* Usable FA edit mode
    * Context menu for deletion, naming, labelling, changing transition symbols
    * Setting states to initial or final
    * State naming, labels (should these JFLAP features be merged?)
    * Undo/redo system
        * Command pattern
        * Possibly include moving a state, other visual things as commands
    * Multiple select mode
        * Click-drag starting on the background
        * Movement and deletion of multiple objects
* Full simulation mode
    * List all current configurations
    * Implement accept/reject detection and display
    * Reset AND rewind buttons
* Saving
    * Investigate starting out with JFLAP-compatibility or a custom file format
* Other simulation actions
    * Fast run
    * Multiple run

## Eventual goals
* More automata
    * Push-down automata
    * Turing machines
    * Possibly others like Mealy machines and Moore machines
* Filetype-compatility with JFLAP
    * Can be implemented incrementally, per-model type
* Regular expressions and grammars
* Operations and conversions like minimization
* Graph visual organization like alignment
* Turing machine blocks
* Built-in tutorials

## Stretch goals/pipe dreams
* Collaborative editing
* Generating URLs of models for easy sharing
