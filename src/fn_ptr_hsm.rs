use std::collections::VecDeque;

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
    pub name: String,
    pub parent: Option<usize>,
    pub enter: Option<EnterFn>,
    pub process: StateFn,
    pub exit: Option<ExitFn>,
    pub active: bool,
}

const MAX_STATE_FNS: usize = 3;

pub struct StateMachine {
    pub state_fns: [StateFns; MAX_STATE_FNS],
    pub enter_fns_idxs: Vec<usize>,
    pub exit_fns_idxs: VecDeque<usize>,
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
        let mut sm = StateMachine {
            state_fns: [
                // STATE_BASE_IDX
                StateFns {
                    name: "state_base".to_owned(),
                    parent: None,
                    enter: Some(Self::state_enter_base),
                    process: Self::state_base,
                    exit: Some(Self::state_exit_base),
                    active: false,
                },
                // STATE_INTERMEDIATE_IDX
                StateFns {
                    name: "state_intermediate".to_owned(),
                    parent: Some(0),
                    enter: Some(Self::state_enter_intermediate),
                    process: Self::state_intermediate,
                    exit: Some(Self::state_exit_intermediate),
                    active: false,
                },
                // STATE_ADD_IDX
                StateFns {
                    name: "state_process_add".to_owned(),
                    parent: Some(1),
                    enter: Some(Self::state_enter_add),
                    process: Self::state_process_add,
                    exit: Some(Self::state_exit_add),
                    active: false,
                },
            ],
            enter_fns_idxs: Vec::<usize>::with_capacity(MAX_STATE_FNS),
            exit_fns_idxs: VecDeque::<usize>::with_capacity(MAX_STATE_FNS),
            current_state_fns_idx: initial_state_fns_idx,
            previous_state_fns_idx: initial_state_fns_idx,
            current_state_changed: true,
            data1: 0,
        };

        // Initialize so transition to initial state works
        sm.initial_enter_fns_idxs();

        sm
    }

    // When the state machine starts there will be no fn's to
    // exit so we initialize only the enter_fns_idxs.
    fn initial_enter_fns_idxs(&mut self) {
        let mut enter_idx = self.current_state_fns_idx;
        loop {
            log::trace!("initial_enter_fns_idxs: pushishing {} at enter_idx={}", self.state_fns[enter_idx].name, enter_idx);
            self.enter_fns_idxs.push(enter_idx);
            enter_idx = if let Some(idx) = self.state_fns[enter_idx].parent {
                idx
            } else {
                break;
            };
        }
    }

    // Starting at self.current_state_fns_idx generate the
    // list of StateFns that we're going to exit. If exit_sentinel is None
    // then exit from current_state_fns_idx and all of its parents.
    // If exit_sentinel is Some then exit from the current state_fns_idx
    // up to but not including the exit_sentinel.
    fn setup_exit_fns_idxs(&mut self, exit_sentinel: Option<usize>) {

        let mut exit_idx = self.current_state_fns_idx;
        loop {
            log::trace!("setup_exit_fns_idxs: push_back {} at exit_idx={}", self.state_fns[exit_idx].name, exit_idx);
            self.exit_fns_idxs.push_back(exit_idx);

            if Some(exit_idx) == exit_sentinel {
                // This handles the special case there we're transition_to yourself
                return;
            }

            exit_idx = if let Some(idx) = self.state_fns[exit_idx].parent {
                idx
            } else {
                // No parent we're done
                return;
            };

            if Some(exit_idx) == exit_sentinel {
                // Reached the exit sentinel so we're done
                return;
            }
        }
    }

    fn setup_exit_enter_fns_idxs(&mut self, next_state_idx: usize) {
        let mut cur_idx = next_state_idx;

        // Setup the enter vector
        let exit_sentinel = loop {
            log::trace!("setup_exit_enter_fns_idxs: pushing {} at enter_idx={}", self.state_fns[cur_idx].name, cur_idx);
            self.enter_fns_idxs.push(cur_idx);

            cur_idx = if let Some(idx) = self.state_fns[cur_idx].parent {
                idx
            } else {
                // Exit state_fns[self.current_state_fns_idx] and all its parents
                break None;
            };

            if self.state_fns[cur_idx].active {
                // Exit state_fns[self.current_state_fns_idx] and
                // parents upto but excluding state_fns[cur_idx]
                break Some(cur_idx);
            }
        };

        // Setup the exit vector
        self.setup_exit_fns_idxs(exit_sentinel);
    }

    pub fn dispatch_msg_idx(&mut self, msg: &Protocol1, idx: usize) {
        log::trace!("dispatch_msg_idx:+ idx={}", idx);

        if self.current_state_changed {
            // Execute the enter functions
            while let Some(enter_idx) = self.enter_fns_idxs.pop() {
                if let Some(state_enter) = self.state_fns[enter_idx].enter {
                    log::trace!("dispatch_msg_idx: entering {} via enter={} at enter_idx={}", self.state_fns[enter_idx].name, state_enter as usize, enter_idx);
                    (state_enter)(self, msg);
                    self.state_fns[enter_idx].active = true;
                }
            }
            self.current_state_changed = false;
        }

        // Invoke the current state funtion processing the result
        log::trace!("dispatch_msg_idx: processing with {} via process={} at idx={}", self.state_fns[idx].name, self.state_fns[idx].process as usize, idx);
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
            StateResult::TransitionTo(next_state_idx) => {
                log::trace!("dispatch_msg_idx: transition_to {} at next_state_idx={}", self.state_fns[next_state_idx].name, next_state_idx);
                self.setup_exit_enter_fns_idxs(next_state_idx);

                self.previous_state_fns_idx = self.current_state_fns_idx;
                self.current_state_fns_idx = next_state_idx;
                self.current_state_changed = true;

            }
        }

        if self.current_state_changed {
            while let Some(exit_idx) = self.exit_fns_idxs.pop_front() {
                if let Some(state_exit) = self.state_fns[exit_idx].exit {
                    log::trace!("dispatch_msg_idx: exiting {} via exit={} at exit_idx={}", self.state_fns[exit_idx].name, state_exit as usize, exit_idx);
                    (state_exit)(self, msg)
                }
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
