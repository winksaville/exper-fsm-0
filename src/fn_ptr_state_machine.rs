// Create a Protocal with three messages
#[derive(Debug)]
pub enum Protocol1 {
    Add { f1: i32 },
    Sub { f1: i32 },
    Mul { f1: i32 },
}

pub struct StateMachine {
    pub current_state: fn(&mut StateMachine, &Protocol1),
    pub data1: i32,
}

impl StateMachine {
    pub fn new(initial_state: fn(&mut Self, &Protocol1)) -> Self {
        StateMachine {
            current_state: initial_state,
            data1: 0,
        }
    }

    pub fn transition_to(&mut self, next_state: fn(&mut Self, &Protocol1)) {
        log::trace!("transition_to+: next_state={:0x}", next_state as usize);
        self.current_state = next_state;
    }

    pub fn dispatch_msg(&mut self, msg: &Protocol1) {
        log::trace!("dispatch_msg+: current_state={:0x}", self.current_state as usize);
        (self.current_state)(self, msg);
    }

    pub fn state_add_or_mul_process_msg(&mut self, msg: &Protocol1) {
        log::trace!("state_add_or_mul_process_msg+: msg={:?}", msg);
        match *msg {
            Protocol1::Add { f1 } => {
                self.data1 += f1;
            }
            Protocol1::Mul { f1 } => {
                self.data1 *= f1;
            }
            _ => panic!("state_add_or_mul only supports Add or Mul msgs"),
        }
        self.transition_to(Self::state_any_process_msg);
    }

    pub fn state_any_process_msg(&mut self, msg: &Protocol1) {
        log::trace!("state_any_process_msg+: msg={:?}", msg);
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
        self.transition_to(Self::state_add_or_mul_process_msg);
    }
}
