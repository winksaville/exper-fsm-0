// Create a Protocal with three message
pub enum Protocol1 {
    Add { f1: i32 },
    Sub { f1: i32 },
    Mul { f1: i32 },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum States {
    StateAddOrMul,
    StateAny,
}

pub struct StateMachine {
    pub current_state: States,
    pub data1: i32,
}

impl StateMachine {
    pub fn new(initial_state: States) -> Self {
        StateMachine {
            current_state: initial_state,
            data1: 0,
        }
    }

    pub fn transition_to(&mut self, next_state: States) {
        self.current_state = next_state;
    }

    pub fn dispatch_msg(&mut self, msg: &Protocol1) {
        match self.current_state {
            States::StateAddOrMul => {
                self.state_add_or_mul_process_msg(msg);
            }
            States::StateAny => {
                self.state_any_process_msg(msg);
            }
        }
    }

    pub fn state_add_or_mul_process_msg(&mut self, msg: &Protocol1) {
        match *msg {
            Protocol1::Add { f1 } => {
                self.data1 += f1;
            }
            Protocol1::Mul { f1 } => {
                self.data1 *= f1;
            }
            _ => panic!("state_add_or_mul only supports Add or Mul msgs"),
        }
        //self.transition_to(States::StateAny);
    }

    pub fn state_any_process_msg(&mut self, msg: &Protocol1) {
        match *msg {
            Protocol1::Add { f1 } => {
                self.data1 += f1;
            }
            Protocol1::Sub { f1 } => {
                self.data1 -= f1;
            }
            Protocol1::Mul { f1 } => {
                self.data1 *= f1;
            }
        }
        //self.transition_to(States::StateAddOrMul);
    }
}
