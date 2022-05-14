#[derive(Debug)]
#[allow(unused)]
pub struct Header<P> {
    pub tx_response: Option<std::sync::mpsc::Sender<P>>,
}

// Create a Protocal with three messages
#[derive(Debug)]
pub enum Protocol1 {
    Add {
        hdr: Header<Protocol1>, // find a fix for RMS (repeating my self)
        f1: i32,
    },
    Mul {
        hdr: Header<Protocol1>,
        f1: i32,
    },
    Get {
        hdr: Header<Protocol1>,
        data1: i32,
    },
}

pub struct StateMachine {
    pub current_state: fn(&mut StateMachine, &Protocol1) -> bool,
    pub previous_state: fn(&mut StateMachine, &Protocol1) -> bool,
    pub current_state_changed: bool,
    pub data1: i32,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine {
    pub fn new() -> Self {
        let initial_state = StateMachine::state_process_msg_add_or_mul;
        StateMachine {
            current_state: initial_state,
            previous_state: initial_state,
            current_state_changed: true,
            data1: 0,
        }
    }

    pub fn transition_to(&mut self, next_state: fn(&mut Self, &Protocol1) -> bool) {
        log::trace!("transition_to: next_state={:0x}", next_state as usize);
        self.previous_state = self.current_state;
        self.current_state = next_state;
        self.current_state_changed = true;
    }

    pub fn dispatch_msg(&mut self, msg: &Protocol1) {
        log::trace!(
            "dispatch_msg: current_state={:0x}",
            self.current_state as usize
        );

        if self.current_state_changed {
            // Handle state_entry
            if self.current_state as usize == StateMachine::state_process_msg_add_or_mul as usize {
                self.state_enter_add_or_mul(msg);
            } else if self.current_state as usize == StateMachine::state_process_msg_any as usize {
                self.state_enter_any(msg);
            }

            self.current_state_changed = false;
        }

        // Dispatch the message to state_process_msg ...
        let processed = (self.current_state)(self, msg);
        if !processed {
            // The suggestion is "good", but this makes extending to
            // additional messages easier
            #[allow(clippy::collapsible_match, clippy::single_match)]
            match msg {
                Protocol1::Get {
                    hdr: Header { tx_response },
                    data1: _,
                } => {
                    if let Some(tx_rsp) = tx_response {
                        let rsp_msg = Protocol1::Get {
                            hdr: Header { tx_response: None },
                            data1: self.data1,
                        };
                        tx_rsp.send(rsp_msg).unwrap();
                    }
                }
                _ => {} // Ignore all other messages
            }
        }

        if self.current_state_changed {
            // Handle state_exit
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

    pub fn state_process_msg_add_or_mul(&mut self, msg: &Protocol1) -> bool {
        log::trace!("state_process_msg_ add_or_mul: msg={:?}", msg);
        let processed = match *msg {
            Protocol1::Add { f1, hdr: _ } => {
                self.data1 += f1;
                true
            }
            Protocol1::Mul { f1, hdr: _ } => {
                self.data1 *= f1;
                true
            }
            _ => false,
        };
        self.transition_to(Self::state_process_msg_any);

        processed
    }

    pub fn state_exit_add_or_mul(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_add_or_mul: msg={:?}", msg);
    }

    pub fn state_enter_any(&mut self, msg: &Protocol1) {
        log::trace!("state_enter_any: msg={:?}", msg);
    }

    pub fn state_process_msg_any(&mut self, msg: &Protocol1) -> bool {
        log::trace!("state_process_msg_any: msg={:?}", msg);
        match &*msg {
            Protocol1::Add { f1, hdr: _ } => {
                self.data1 += f1;
            }
            Protocol1::Mul { f1, hdr: _ } => {
                self.data1 *= f1;
            }
            Protocol1::Get {
                hdr: Header { tx_response },
                data1: _,
            } => {
                if let Some(tx_rsp) = tx_response {
                    let rsp_msg = Protocol1::Get {
                        hdr: Header { tx_response: None },
                        data1: self.data1,
                    };
                    tx_rsp.send(rsp_msg).unwrap();
                }
            }
        }
        self.transition_to(Self::state_process_msg_add_or_mul);

        true
    }

    pub fn state_exit_any(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_any: msg={:?}", msg);
    }
}
