use expr_fsm_0::{enum_state_machine, fn_ptr_state_machine};

use custom_logger::env_logger_init;

fn enum_sm() {
    println!("state_machine_current_state_enum");
    use enum_state_machine::{Protocol1, StateMachine, States};

    // Create state machine with its initial state
    let mut sm = StateMachine::default();
    assert!(sm.current_state == States::StateAddOrMul);

    // Create a message and dispatch it to the state machine
    let msg = Protocol1::Add { f1: 123 };
    sm.dispatch_msg(&msg);
    assert!(sm.current_state == States::StateAny);
    println!("sm.data1={}", sm.data1);

    // Transition to a new state and dispatch the message again
    assert!(sm.current_state == States::StateAny);
    sm.dispatch_msg(&msg);
    assert!(sm.current_state == States::StateAddOrMul);
    println!("sm.data1={}", sm.data1);

    assert!(sm.current_state == States::StateAddOrMul);
    sm.dispatch_msg(&msg);
    assert!(sm.current_state == States::StateAny);
    println!("sm.data1={}", sm.data1);
}

fn fn_ptr_sm() {
    println!("\nstate_machine_current_state_fn_ptr");
    use fn_ptr_state_machine::{Protocol1, StateMachine};

    // Create state machine with its initial state
    let mut sm = StateMachine::default();
    assert!(sm.current_state as usize == StateMachine::state_process_msg_add_or_mul as usize);

    // Create a message and dispatch it to the state machine
    let msg = Protocol1::Add { f1: 123 };
    sm.dispatch_msg(&msg);
    assert!(sm.current_state as usize == StateMachine::state_process_msg_any as usize);
    println!("sm.data1={}", sm.data1);

    let msg = Protocol1::Add { f1: 123 };
    // ThTransition to a new state and dispatch the message again
    //let cs: &dyn for<'a> Fn(&'a mut StateMachine, &'a Protocol1) = sm.current_state;
    //let expected_state: &dyn for<'a> Fn(&'a mut StateMachine, &'a Protocol1) = &StateMachine::state_any_process_msg;
    //assert!(cs == expected_state);
    sm.dispatch_msg(&msg);
    assert!(sm.current_state as usize == StateMachine::state_process_msg_add_or_mul as usize);
    println!("sm.data1={}", sm.data1);

    sm.dispatch_msg(&msg);
    assert!(sm.current_state as usize == StateMachine::state_process_msg_any as usize);
    println!("sm.data1={}", sm.data1);
}

fn main() {
    env_logger_init("info");
    log::debug!("main:+");

    enum_sm();
    fn_ptr_sm();

    log::debug!("main:-");
}
