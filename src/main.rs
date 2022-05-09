// Create a Protocal with one message
enum Protocol1 {
    Msg1 { f1: i32 },
}

enum States {
    State1,
}

struct StateMachine {
    current_state: States,
    data1: i32,
}

impl StateMachine {
    fn new(initial_state: States) -> Self {
        StateMachine {
            current_state: initial_state,
            data1: 0,
        }
    }

    fn process_msg(&mut self, msg: &Protocol1) {
        match self.current_state {
            States::State1 => {
                match *msg {
                    Protocol1::Msg1 { f1 } => {
                        self.data1 += 1;
                        println!("State1: process sm.data1={} Msg1::f1={}", self.data1, f1);
                    }
                }
            }
        }
    }
}

fn main() {
    // Create state machine with its initial state
    let initial_state = States::State1;
    let mut sm = StateMachine::new(initial_state);

    // Create a message and dispatch it to the state machine
    let msg = Protocol1::Msg1 { f1: 123 };
    sm.process_msg(&msg);
}
