// Create a Protocal with one message


fn main() {
    {
        println!("state_machine_current_state_enum");
        use expr_fsm_0::state_machine_current_state_enum::{StateMachine, Protocol1, States};

        // Create state machine with its initial state
        let initial_state = States::StateAdd;
        let mut sm = StateMachine::new(initial_state);

        // Create a message and dispatch it to the state machine
        let msg = Protocol1::Add { f1: 123 };
        sm.dispatch_msg(&msg);
        println!("sm.data1={}", sm.data1);
        sm.dispatch_msg(&msg);
        println!("sm.data1={}", sm.data1);
    }

    {
        println!("state_machine_current_state_fn_ptr");
        use expr_fsm_0::state_machine_current_state_fn_ptr::{StateMachine, Protocol1};

        // Create state machine with its initial state
        let initial_state = StateMachine::state_add_process_msg;
        let mut sm = StateMachine::new(& initial_state);

        // Create a message and dispatch it to the state machine
        let msg = Protocol1::Add { f1: 123 };
        sm.dispatch_msg(&msg);
        println!("sm.data1={}", sm.data1);
        sm.dispatch_msg(&msg);
        println!("sm.data1={}", sm.data1);
    }
}
