// Create a Protocal with three message
pub enum Protocol1 {
    Add { f1: i32 },
    Sub { f1: i32 },
    Mul { f1: i32 },
}

pub enum States {
    StateAdd,
    StateSub,
    StateMul,
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

    pub fn dispatch_msg(&mut self, msg: &Protocol1) {
        match self.current_state {
            States::StateAdd => {
                self.state_add_process_msg(msg);
            }
            States::StateSub => {
                self.state_sub_process_msg(msg);
            }
            States::StateMul => {
                self.state_mul_process_msg(msg);
            }
            States::StateAny => {
                self.state_any_process_msg(msg);
            }
        }
    }

    pub fn state_add_process_msg(&mut self, msg: &Protocol1) {
        match *msg {
            Protocol1::Add { f1 } => {
                self.data1 += f1;
            }
            _ => panic!("State1 only supports Add msgs"),
        }
    }

    pub fn state_sub_process_msg(&mut self, msg: &Protocol1) {
        match *msg {
            Protocol1::Sub { f1 } => {
                self.data1 -= f1;
            }
            _ => panic!("StateSub only supports Sub msgs"),
        }
    }

    pub fn state_mul_process_msg(&mut self, msg: &Protocol1) {
        match *msg {
            Protocol1::Mul { f1 } => {
                self.data1 *= f1;
            }
            _ => panic!("StateMul only supports Mul msgs"),
        }
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
    }
}
