use std::collections::HashMap;

enum Movements {
    RIGHT,
    LEFT,
    NEUTRAL,
}

struct TransitionFunctionInput {
    state: i32,
    input: String,
}

struct TransitionFunctionOutput {
    state: i32,
    replacement: String,
    movement: Movements,
}

pub struct TransitionFunction {
    input: TransitionFunctionInput,
    output: TransitionFunctionOutput,
}

pub struct TuringMachine {
    pub tape: HashMap<i64, char>,
    pub position: i64,
    pub state: Vec<i32>,
    pub input_alphabet: Vec<char>,
    pub working_alphabet: Vec<char>,
    pub starting_state: i32,
    pub blank: char,
    pub end_states: Vec<i32>,
    pub transitions: Vec<TransitionFunction>,
}

impl TuringMachine {
    pub fn new(blank: Option<char>) -> TuringMachine {
        let mut tm = TuringMachine {
            tape: HashMap::new(),
            position: 0,
            state: Vec::new(),
            input_alphabet: Vec::new(),
            working_alphabet: Vec::new(),
            starting_state: 0,
            blank: blank.unwrap_or('_'),
            end_states: Vec::new(),
            transitions: Vec::new(),
        };
        return tm;
    }
}
