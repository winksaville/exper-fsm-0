//use std::{pin::Pin, ptr::NonNull};

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

type StateFn = for<'sm, 'a> fn(&'sm mut StateMachine<'sm>, &'a Protocol1) -> StateResult;
type EnterFn = for<'sm, 'a> fn(&'sm mut StateMachine<'sm>, &'a Protocol1);
type ExitFn = for<'sm , 'a> fn(&'sm mut StateMachine<'sm>, &'a Protocol1);

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

pub struct StateMachine<'sm> {
    pub state_fns: Vec<StateFns>,
    pub current_state_fns: &'sm StateFns,
    pub previous_state_fns: &'sm StateFns,
    pub current_state_changed: bool,
    pub data1: i32,
}

impl<'sm> Default for StateMachine<'sm> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'sm, 'a> StateMachine<'sm> {
    const STATE_ADD_OR_MUL_IDX: usize = 0;
    const STATE_ANY_IDX: usize = 1;

    pub fn new() -> Self { //StateMachine<'sm> {
        let sfv = vec![
            // STATE_ADD_OR_MUL_IDX
            StateFns {
                parent: None,
                enter: None, //Some(Self::state_enter_add_or_mul),
                process: Self::state_process_add_or_mul,
                exit: None, //Some(Self::state_exit_add_or_mul),
            },
            // STATE_ANY_IDX
            StateFns {
                parent: None,
                enter: None, //Some(Self::state_enter_any),
                process: Self::state_process_any,
                exit: None, //Some(Self::state_exit_any),
            }
        ];

        let initial_state_fns = &sfv[0];
        StateMachine {
            state_fns:  sfv,
            current_state_fns: initial_state_fns,
            previous_state_fns: initial_state_fns,
            current_state_changed: true,
            data1: 0,
        }
    }

    // TODO: This is not correct, it's returning an uninitialized memory
    //fn get_state_fns(&self, _idx: usize) -> NonNull::<Pin<&'static StateFns>> {
    //    // How do I actually index into self.state_fns[idx] and return correct type????
    //    //let sfs = NonNull::<Pin<&'static StateFns>>::as_mut_ptr(&sm.state_fns[_idx]);

    //    let _z = unsafe {
    //        let the_ref = Pin::<&'static StateFns>::new(&self.state_fns[_idx]);
    //        //let x = NonNull::<Pin<&'static StateFns>>::from(the_ref); // Not
    //        NonNull {
    //            pointer: &the_ref as *const _,
    //        }
    //    };

    //    //NonNull::<Pin<&'static StateFns>>::dangling()
    //    _z
    //}
    //    
    //pub fn get_process(&self) -> StateFn {
    //        unsafe { self.current_state_fns.process }
    //}

    //pub fn get_enter(&self) -> Option<EnterFn> {
    //        self.current_state_fns.enter
    //}

    //pub fn get_exit(&self) -> Option<ExitFn> {
    //        unsafe { self.current_state_fns.exit }
    //}

    pub fn dispatch_msg(&'sm mut self, msg: &'a Protocol1) {
        //log::trace!(
        //    "dispatch_msg: current_state_fns_idx={}",
        //    self.current_state_fns as usize // this doesn't compile!!!
        //);

        if self.current_state_changed {
            // Handle state_enter
            if let Some(state_enter) = self.current_state_fns.enter {
                (state_enter)(self, msg)
            }
            self.current_state_changed = false;
        }

        // Invoke the current state funtion processing the result
        let process = self.current_state_fns.process;
        match (process)(self, msg) {
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
                self.previous_state_fns = self.current_state_fns;
                //self.current_state_fns = &self.state_fns[next_state];
                self.current_state_changed = true;
            }
        }

        if self.current_state_changed {
            if let Some(state_exit) = self.current_state_fns.exit {
                (state_exit)(self, msg)
            }
        }
    }

    pub fn state_enter_add_or_mul(&'sm mut self, msg: &'a Protocol1) {
        log::trace!("state_enter_add_or_mul: msg={:?}", msg);
    }

    pub fn state_process_add_or_mul(&'sm mut self, msg: &'a Protocol1) -> StateResult {
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

    pub fn state_exit_add_or_mul(&'sm mut self, msg: &'a Protocol1) {
        log::trace!("state_exit_add_or_mul: msg={:?}", msg);
    }

    pub fn state_enter_any(&'sm mut self, msg: &'a Protocol1) {
        log::trace!("state_enter_any: msg={:?}", msg);
    }

    pub fn state_process_any(&'sm mut self, msg: &'a Protocol1) -> StateResult {
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

    pub fn state_exit_any(&'sm mut self, msg: &'a Protocol1) {
        log::trace!("state_exit_any: msg={:?}", msg);
    }
}
