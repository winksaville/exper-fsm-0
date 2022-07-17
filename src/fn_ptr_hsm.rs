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
    pub enter_fns_hdls: Vec<usize>,
    pub exit_fns_hdls: VecDeque<usize>,
    pub current_state_fns_hdl: usize,
    pub previous_state_fns_hdl: usize,
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
    const STATE_BASE_HDL: usize = 0;
    #[allow(unused)]
    const STATE_BASE_INTERMEDIATE_HDL: usize = 0;
    const STATE_ADD_HDL: usize = 2;

    pub fn new() -> Self {
        let initial_state_fns_hdl = Self::STATE_ADD_HDL;
        let mut sm = StateMachine {
            state_fns: [
                // STATE_BASE_HDL
                StateFns {
                    name: "state_base".to_owned(),
                    parent: None,
                    enter: Some(Self::state_enter_base),
                    process: Self::state_base,
                    exit: Some(Self::state_exit_base),
                    active: false,
                },
                // STATE_INTERMEDIATE_HDL
                StateFns {
                    name: "state_intermediate".to_owned(),
                    parent: Some(0),
                    enter: Some(Self::state_enter_intermediate),
                    process: Self::state_intermediate,
                    exit: Some(Self::state_exit_intermediate),
                    active: false,
                },
                // STATE_ADD_HDL
                StateFns {
                    name: "state_process_add".to_owned(),
                    parent: Some(1),
                    enter: Some(Self::state_enter_add),
                    process: Self::state_process_add,
                    exit: Some(Self::state_exit_add),
                    active: false,
                },
            ],
            enter_fns_hdls: Vec::<usize>::with_capacity(MAX_STATE_FNS),
            exit_fns_hdls: VecDeque::<usize>::with_capacity(MAX_STATE_FNS),
            current_state_fns_hdl: initial_state_fns_hdl,
            previous_state_fns_hdl: initial_state_fns_hdl,
            current_state_changed: true,
            data1: 0,
        };

        let name = sm.state_name();
        println!("new:+ inital state={}", name);

        // Initialize so transition to initial state works
        sm.initial_enter_fns_hdls();

        sm
    }

    fn state_name(&self) -> &str {
        &self.state_fns[self.current_state_fns_hdl].name
    }

    // When the state machine starts there will be no fn's to
    // exit so we initialize only the enter_fns_hdls.
    fn initial_enter_fns_hdls(&mut self) {
        let mut enter_hdl = self.current_state_fns_hdl;
        loop {
            log::trace!("initial_enter_fns_hdls: pushishing {} at enter_hdl={}", self.state_fns[enter_hdl].name, enter_hdl);
            self.enter_fns_hdls.push(enter_hdl);
            enter_hdl = if let Some(hdl) = self.state_fns[enter_hdl].parent {
                hdl
            } else {
                break;
            };
        }
    }

    // Starting at self.current_state_fns_hdl generate the
    // list of StateFns that we're going to exit. If exit_sentinel is None
    // then exit from current_state_fns_hdl and all of its parents.
    // If exit_sentinel is Some then exit from the current state_fns_hdl
    // up to but not including the exit_sentinel.
    fn setup_exit_fns_hdls(&mut self, exit_sentinel: Option<usize>) {

        let mut exit_hdl = self.current_state_fns_hdl;
        loop {
            log::trace!("setup_exit_fns_hdls: push_back {} at exit_hdl={}", self.state_fns[exit_hdl].name, exit_hdl);
            self.exit_fns_hdls.push_back(exit_hdl);

            if Some(exit_hdl) == exit_sentinel {
                // This handles the special case there we're transition_to yourself
                return;
            }

            exit_hdl = if let Some(hdl) = self.state_fns[exit_hdl].parent {
                hdl
            } else {
                // No parent we're done
                return;
            };

            if Some(exit_hdl) == exit_sentinel {
                // Reached the exit sentinel so we're done
                return;
            }
        }
    }

    fn setup_exit_enter_fns_hdls(&mut self, next_state_hdl: usize) {
        let mut cur_hdl = next_state_hdl;

        // Setup the enter vector
        let exit_sentinel = loop {
            log::trace!("setup_exit_enter_fns_hdls: pushing {} at enter_hdl={}", self.state_fns[cur_hdl].name, cur_hdl);
            self.enter_fns_hdls.push(cur_hdl);

            cur_hdl = if let Some(hdl) = self.state_fns[cur_hdl].parent {
                hdl
            } else {
                // Exit state_fns[self.current_state_fns_hdl] and all its parents
                break None;
            };

            if self.state_fns[cur_hdl].active {
                // Exit state_fns[self.current_state_fns_hdl] and
                // parents upto but excluding state_fns[cur_hdl]
                break Some(cur_hdl);
            }
        };

        // Setup the exit vector
        self.setup_exit_fns_hdls(exit_sentinel);
    }

    pub fn dispatch_msg_hdl(&mut self, msg: &Protocol1, hdl: usize) {
        log::trace!("dispatch_msg_hdl:+ hdl={}", hdl);

        if self.current_state_changed {
            // Execute the enter functions
            while let Some(enter_hdl) = self.enter_fns_hdls.pop() {
                if let Some(state_enter) = self.state_fns[enter_hdl].enter {
                    log::trace!("dispatch_msg_hdl: entering {} via enter={} at enter_hdl={}", self.state_fns[enter_hdl].name, state_enter as usize, enter_hdl);
                    (state_enter)(self, msg);
                    self.state_fns[enter_hdl].active = true;
                }
            }
            self.current_state_changed = false;
        }

        // Invoke the current state funtion processing the result
        log::trace!("dispatch_msg_hdl: processing with {} via process={} at hdl={}", self.state_fns[hdl].name, self.state_fns[hdl].process as usize, hdl);
        match (self.state_fns[hdl].process)(self, msg) {
            StateResult::NotHandled => {
                log::trace!("dispatch_msg_hdl: NotHandled hdl={}", hdl);
                if let Some(parent_hdl) = self.state_fns[hdl].parent {
                    self.dispatch_msg_hdl(msg, parent_hdl);
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
                log::trace!("dispatch_msg_hdl: Handled hdl={}", hdl);
            }
            StateResult::TransitionTo(next_state_hdl) => {
                log::trace!("dispatch_msg_hdl: transition_to {} at next_state_hdl={}", self.state_fns[next_state_hdl].name, next_state_hdl);
                self.setup_exit_enter_fns_hdls(next_state_hdl);

                self.previous_state_fns_hdl = self.current_state_fns_hdl;
                self.current_state_fns_hdl = next_state_hdl;
                self.current_state_changed = true;

            }
        }

        if self.current_state_changed {
            while let Some(exit_hdl) = self.exit_fns_hdls.pop_front() {
                if let Some(state_exit) = self.state_fns[exit_hdl].exit {
                    log::trace!("dispatch_msg_hdl: exiting {} via exit={} at exit_hdl={}", self.state_fns[exit_hdl].name, state_exit as usize, exit_hdl);
                    (state_exit)(self, msg)
                }
            }
        }

        log::trace!("dispatch_msg_hdl:- hdl={}", hdl);
    }

    pub fn dispatch_msg(&mut self, msg: &Protocol1) {
        log::trace!(
            "dispatch_msg:+ current_state_fns_hdl={}",
            self.current_state_fns_hdl as usize
        );
        self.dispatch_msg_hdl(msg, self.current_state_fns_hdl);
        log::trace!(
            "dispatch_msg:- current_state_fns_hdl={}",
            self.current_state_fns_hdl as usize
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
                StateResult::TransitionTo(Self::STATE_ADD_HDL)
            }
            _ => StateResult::NotHandled,
        }
    }

    pub fn state_exit_add(&mut self, msg: &Protocol1) {
        log::trace!("state_exit_add: msg={:?}", msg);
    }
}
