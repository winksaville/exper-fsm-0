#[derive(Debug)]
#[allow(unused)]
pub struct Header<P> {
    pub tx_response: Option<std::sync::mpsc::Sender<P>>,
}

// Create a Protocol with three messages
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

type StateFn = fn(&mut StateMachine, &Protocol1) -> StateResult;
type EnterFn = fn(&mut StateMachine, &Protocol1);
type ExitFn = fn(&mut StateMachine, &Protocol1);

pub enum StateResult {
    NotHandled,
    Handled,
    TransitionTo(usize),
}

pub struct StateFns {
    pub parent: Option<usize>,
    pub enter: Option<EnterFn>,
    pub process: StateFn,
    pub exit: Option<ExitFn>,
}

pub struct StateMachine {
    pub state_fns: [StateFns; 2],
    pub current_state_fns_idx: usize,
    pub previous_state_fns_idx: usize,
    pub current_state_changed: bool,
    pub data1: i32,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine {
    const STATE_ADD_OR_MUL_IDX: usize = 0;
    const STATE_ANY_IDX: usize = 1;

    pub fn new() -> Self {
        let initial_state_fns_idx = 0usize;
        StateMachine {
            state_fns: [
                // STATE_ADD_OR_MUL_IDX
                StateFns {
                    parent: None,
                    enter: Some(Self::state_enter_add_or_mul),
                    process: Self::state_process_add_or_mul,
                    exit: Some(Self::state_exit_add_or_mul),
                },
                // STATE_ANY_IDX
                StateFns {
                    parent: None,
                    enter: Some(Self::state_enter_any),
                    process: Self::state_process_any,
                    exit: Some(Self::state_exit_any),
                }
            ],
            current_state_fns_idx: initial_state_fns_idx,
            previous_state_fns_idx: initial_state_fns_idx,
            current_state_changed: true,
            data1: 0,
        }
    }

    pub fn dispatch_msg(&mut self, msg: &Protocol1) {
        log::trace!(
            "dispatch_msg: current_state_fns_idx={}",
            self.current_state_fns_idx as usize
        );

        if self.current_state_changed {
            // Handle state_enter
            if let Some(state_enter) = self.state_fns[self.current_state_fns_idx].enter {
                (state_enter)(self, msg)
            }
            self.current_state_changed = false;
        }

        // Invoke the current state funtion processing the result
        match (self.state_fns[self.current_state_fns_idx].process)(self, msg) {
            StateResult::NotHandled => {
                // Handle messages we can and ignore all other messages

                // The suggestion clippy makes is "good", but this makes extending to
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
            },
            StateResult::Handled => {
                // Nothing to do
            }
            StateResult::TransitionTo(next_state) => {
                log::trace!("transition_to: next_state={}", next_state);
                self.previous_state_fns_idx = self.current_state_fns_idx;
                self.current_state_fns_idx = next_state;
                self.current_state_changed = true;
            }
        }

        if self.current_state_changed {
            if let Some(state_exit) = self.state_fns[self.previous_state_fns_idx].exit {
                (state_exit)(self, msg)
            }
        }
    }

    pub fn state_enter_add_or_mul(&mut self, msg: &Protocol1) {
        log::trace!("state_enter_add_or_mul: msg={:?}", msg);
    }

    pub fn state_process_add_or_mul(&mut self, msg: &Protocol1) -> StateResult {
        log::trace!("state_process_add_or_mul: msg={:?}", msg);
        match *msg {
            Protocol1::Add { f1, hdr: _ } => {
                self.data1 += f1;
                StateResult::TransitionTo(Self::STATE_ANY_IDX)
            }
            Protocol1::Mul { f1, hdr: _ } => {
                self.data1 *= f1;
                StateResult::TransitionTo(Self::STATE_ANY_IDX)
            }
            _ => StateResult::NotHandled,
        }
    }

    pub fn state_exit_add_or_mul(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_add_or_mul: msg={:?}", msg);
    }

    pub fn state_enter_any(&mut self, msg: &Protocol1) {
        log::trace!("state_enter_any: msg={:?}", msg);
    }

    pub fn state_process_any(&mut self, msg: &Protocol1) -> StateResult {
        log::trace!("state_process_any: msg={:?}", msg);
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

        StateResult::TransitionTo(Self::STATE_ADD_OR_MUL_IDX)
    }

    pub fn state_exit_any(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_any: msg={:?}", msg);
    }
}
