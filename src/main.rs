// A trait that processes a message and drives the state machine
trait ProcessMsg {
    fn process_msg(&self, sm: &mut StateMachine, msg: &Protocol1);
}

// Create a Protocal with one message
enum Protocol1 {
    Msg1 { f1: i32 },
}

struct StateMachine {
    current_state: Box<dyn ProcessMsg>,
    data1: i32,
}

impl StateMachine {
    fn new(initial_state: Box<dyn ProcessMsg>) -> Self {
        StateMachine {
            current_state: initial_state,
            data1: 0,
        }
    }

    fn dispatch(&mut self, msg: &Protocol1) {
        self.current_state.process_msg(self, msg);
    }
}

struct State1;

impl ProcessMsg for State1 {
    fn process_msg(&self, sm: &mut StateMachine, msg: &Protocol1) {
        match *msg {
            Protocol1::Msg1 { f1 } => {
                sm.data1 += 1;
                println!("State1: process sm.data1={} Msg1::f1={}", sm.data1, f1);
            }
        }
    }
}

fn main() {
    // Create state machine with its initial state
    let initial_state = Box::new(State1);
    let mut sm = StateMachine::new(initial_state);

    // Create a message and dispatch it to the state machine
    let msg = Protocol1::Msg1 { f1: 123 };
    sm.dispatch(&msg);
}
