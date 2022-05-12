// Create a Protocal with three messages
#[derive(Debug)]
pub enum Protocol1 {
    Add { f1: i32 },
    Sub { f1: i32 },
    Mul { f1: i32 },
}

pub struct StateMachine {
    pub current_state: fn(&mut StateMachine, &Protocol1),
    pub previous_state: fn(&mut StateMachine, &Protocol1),
    pub current_state_changed: bool,
    pub data1: i32,
}

impl StateMachine {
    pub fn new(initial_state: fn(&mut Self, &Protocol1)) -> Self {
        StateMachine {
            current_state: initial_state,
            previous_state: initial_state,
            current_state_changed: true,
            data1: 0,
        }
    }

    pub fn transition_to(&mut self, next_state: fn(&mut Self, &Protocol1)) {
        log::trace!("transition_to: next_state={:0x}", next_state as usize);
        self.previous_state = self.current_state;
        self.current_state = next_state;
        self.current_state_changed = true;
    }

    pub fn dispatch_msg(&mut self, msg: &Protocol1) {
        log::trace!("dispatch_msg: current_state={:0x}", self.current_state as usize);

        // Handle state_entry
        if self.current_state_changed {
            if self.current_state as usize == StateMachine::state_process_msg_add_or_mul as usize {
                self.state_enter_add_or_mul(msg);
            } else if self.current_state as usize == StateMachine::state_process_msg_any as usize {
                self.state_enter_any(msg);
            }

            self.current_state_changed = false;
        }

        // Dispatch the message to state_process_msg ...
        (self.current_state)(self, msg);

        // Handle state_exit
        if self.current_state_changed {
            if self.previous_state as usize == StateMachine::state_process_msg_add_or_mul as usize {
                self.state_exit_add_or_mul(msg);
            } else if self.previous_state as usize == StateMachine::state_process_msg_any as usize {
                self.state_exit_any(msg);
            }
        }

    }

    pub fn state_enter_add_or_mul(&mut self, msg: &Protocol1) {
        log::trace!("state_enter_add_or_mul: msg={:?}", msg);
    }

    pub fn state_process_msg_add_or_mul(&mut self, msg: &Protocol1) {
        log::trace!("state_process_msg_ add_or_mul: msg={:?}", msg);
        match *msg {
            Protocol1::Add { f1 } => {
                self.data1 += f1;
            }
            Protocol1::Mul { f1 } => {
                self.data1 *= f1;
            }
            _ => panic!("state_process_msg_add_or_mul only supports Add or Mul msgs"),
        }
        self.transition_to(Self::state_process_msg_any);
    }

    pub fn state_exit_add_or_mul(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_add_or_mul: msg={:?}", msg);
    }


    pub fn state_enter_any(&mut self, msg: &Protocol1) {
        log::trace!("state_enter_any: msg={:?}", msg);
    }

    pub fn state_process_msg_any(&mut self, msg: &Protocol1) {
        log::trace!("state_process_msg_any: msg={:?}", msg);
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
        self.transition_to(Self::state_process_msg_add_or_mul);
    }

    pub fn state_exit_any(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_any: msg={:?}", msg);
    }
}
