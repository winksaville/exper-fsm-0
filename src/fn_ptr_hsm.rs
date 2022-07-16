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
    pub state_fns: [StateFns; 3],
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
    #[allow(unused)]
    const STATE_BASE_IDX: usize = 0;
    #[allow(unused)]
    const STATE_BASE_INTERMEDIATE_IDX: usize = 0;
    const STATE_ADD_IDX: usize = 2;

    pub fn new() -> Self {
        let initial_state_fns_idx = Self::STATE_ADD_IDX;
        StateMachine {
            state_fns: [
                // STATE_BASE_IDX
                StateFns {
                    parent: None,
                    enter: Some(Self::state_enter_base),
                    process: Self::state_base,
                    exit: Some(Self::state_exit_base),
                },
                // STATE_INTERMEDIATE_IDX
                StateFns {
                    parent: Some(0),
                    enter: Some(Self::state_enter_intermediate),
                    process: Self::state_intermediate,
                    exit: Some(Self::state_exit_intermediate),
                },
                // STATE_ADD_IDX
                StateFns {
                    parent: Some(1),
                    enter: Some(Self::state_enter_add),
                    process: Self::state_process_add,
                    exit: Some(Self::state_exit_add),
                },
            ],
            current_state_fns_idx: initial_state_fns_idx,
            previous_state_fns_idx: initial_state_fns_idx,
            current_state_changed: true,
            data1: 0,
        }
    }

    pub fn dispatch_msg_idx(&mut self, msg: &Protocol1, idx: usize) {
        log::trace!("dispatch_msg_idx:+ idx={}", idx);

        if self.current_state_changed {
            // Handle state_enter
            if let Some(state_enter) = self.state_fns[idx].enter {
                log::trace!("dispatch_msg_idx: state changed call state_enter={} idx={}", state_enter as usize, idx);
                (state_enter)(self, msg)
            }
            self.current_state_changed = false;
        }

        // Invoke the current state funtion processing the result
        match (self.state_fns[idx].process)(self, msg) {
            StateResult::NotHandled => {
                log::trace!("dispatch_msg_idx: NotHandled idx={}", idx);
                if let Some(parent_idx) = self.state_fns[idx].parent {
                    self.dispatch_msg_idx(msg, parent_idx);
                } else {
                    // No parent
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
                // Handle messages we can and ignore all other messages

                // The suggestion clippy makes is "good", but this makes extending to
                // additional messages easier
            },
            StateResult::Handled => {
                // Nothing to do
                log::trace!("dispatch_msg_idx: Handled idx={}", idx);
            }
            StateResult::TransitionTo(next_state) => {
                log::trace!("dispatch_msg_idx: transition_to next_state={}", next_state);
                self.previous_state_fns_idx = self.current_state_fns_idx;
                self.current_state_fns_idx = next_state;
                self.current_state_changed = true;
            }
        }

        if self.current_state_changed {
            log::trace!("dispatch_msg_idx: state changed idx={}", idx);
            if let Some(state_exit) = self.state_fns[self.previous_state_fns_idx].exit {
                log::trace!("dispatch_msg_idx: state changed call state_exit={} idx={}", state_exit as usize, idx);
                (state_exit)(self, msg)
            }
        }

        log::trace!("dispatch_msg_idx:- idx={}", idx);
    }

    pub fn dispatch_msg(&mut self, msg: &Protocol1) {
        log::trace!(
            "dispatch_msg:+ current_state_fns_idx={}",
            self.current_state_fns_idx as usize
        );
        self.dispatch_msg_idx(msg, self.current_state_fns_idx);
        log::trace!(
            "dispatch_msg:- current_state_fns_idx={}",
            self.current_state_fns_idx as usize
        );
    }

    pub fn state_enter_base(&mut self, msg: &Protocol1) {
        log::trace!("state_enter_base: msg={:?}", msg);
    }

    pub fn state_base(&mut self, msg: &Protocol1) -> StateResult {
        log::trace!("state_base: msg={:?}", msg);
        match &*msg {
            // Implement Get
            Protocol1::Get {
                hdr: Header { tx_response },
                data1: _,
            } => {
                if let Some(tx_rsp) = tx_response {
                    let rsp_msg = Protocol1::Get {
                        hdr: Header { tx_response: None },
                        data1: self.data1,
                    };
                    log::trace!("state_base: respond rsp_msg={:?}", rsp_msg);
                    tx_rsp.send(rsp_msg).unwrap();
                }
            }

            // Ignore any other messages, possibly panic!?
            _ => (),
        }

        StateResult::Handled
    }

    pub fn state_exit_base(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_base: msg={:?}", msg);
    }


    pub fn state_enter_intermediate(&mut self, msg: &Protocol1) {
        log::trace!("state_enter_intermediate: msg={:?}", msg);
    }

    pub fn state_intermediate(&mut self, msg: &Protocol1) -> StateResult {
        log::trace!("state_intermediate: msg={:?}", msg);
        StateResult::NotHandled
    }

    pub fn state_exit_intermediate(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_intermediate: msg={:?}", msg);
    }

    pub fn state_enter_add(&mut self, msg: &Protocol1) {
        log::trace!("state_enter_add: msg={:?}", msg);
    }

    pub fn state_process_add(&mut self, msg: &Protocol1) -> StateResult {
        log::trace!("state_process_add: msg={:?}", msg);
        match *msg {
            Protocol1::Add { f1, hdr: _ } => {
                self.data1 += f1;
                StateResult::TransitionTo(Self::STATE_ADD_IDX)
            }
            _ => StateResult::NotHandled,
        }
    }

    pub fn state_exit_add(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_add: msg={:?}", msg);
    }
}
