// Create a Protocal with three messages
pub enum Protocol1 {
    Add { f1: i32 },
    Sub { f1: i32 },
    Mul { f1: i32 },
}

pub struct StateMachine<'a> {
    pub current_state: &'a dyn Fn(&mut Self, &Protocol1),
    pub data1: i32,
}

impl<'a> StateMachine<'a> {
    pub fn new(initial_state: &'a dyn Fn(&mut Self, &Protocol1)) -> Self {
        StateMachine {
            current_state: initial_state,
            data1: 0,
        }
    }

    pub fn dispatch_msg(&mut self, msg: &Protocol1) {
        (self.current_state)(self, msg);
    }

    pub fn state_add_process_msg(&mut self, msg: &Protocol1) {
        match *msg {
            Protocol1::Add { f1 } => {
                self.data1 += f1;
            }
            _ => panic!("state_add only supports Add msgs"),
        }
    }

    pub fn state_sub_process_msg(&mut self, msg: &Protocol1) {
        match *msg {
            Protocol1::Sub { f1 } => {
                self.data1 -= f1;
            }
            _ => panic!("state_sub only supports Sub msgs"),
        }
    }

    pub fn state_mul_process_msg(&mut self, msg: &Protocol1) {
        match *msg {
            Protocol1::Mul { f1 } => {
                self.data1 *= f1;
            }
            _ => panic!("state_mul only supports Mul msgs"),
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
