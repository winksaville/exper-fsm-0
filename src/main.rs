use expr_fsm_0::{enum_state_machine, fn_ptr_state_machine};

fn main() {
    {
        println!("state_machine_current_state_enum");
        use enum_state_machine::{Protocol1, StateMachine, States};

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
        use fn_ptr_state_machine::{Protocol1, StateMachine};

        // Create state machine with its initial state
        let initial_state = StateMachine::state_add_process_msg;
        let mut sm = StateMachine::new(&initial_state);

        // Create a message and dispatch it to the state machine
        let msg = Protocol1::Add { f1: 123 };
        sm.dispatch_msg(&msg);
        println!("sm.data1={}", sm.data1);
        sm.dispatch_msg(&msg);
        println!("sm.data1={}", sm.data1);
    }
}
