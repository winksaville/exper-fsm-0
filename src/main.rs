use std::{
    sync::mpsc::{self, Receiver, Sender},
    time::SystemTime,
};

use custom_logger::env_logger_init;

fn enum_sm() {
    use expr_fsm_0::enum_state_machine::{Header, Protocol1, StateMachine, States};

    println!("state_machine_current_state_enum");

    // Create a sm and validate it's in the expected state
    let mut sm = StateMachine::default();
    assert!(sm.current_state == States::StateAddOrMul);

    // Dispatch the message and validate it transitioned
    let msg = Protocol1::Add {
        hdr: Header { tx_response: None },
        f1: 123,
    };
    sm.dispatch_msg(&msg);
    assert!(sm.current_state == States::StateAny);
    println!("sm.data1={}", sm.data1);

    // Dispatch the message and validate it transitioned
    assert!(sm.current_state == States::StateAny);
    sm.dispatch_msg(&msg);
    assert!(sm.current_state == States::StateAddOrMul);
    println!("sm.data1={}", sm.data1);

    // Dispatch the message and validate it transitioned
    assert!(sm.current_state == States::StateAddOrMul);
    sm.dispatch_msg(&msg);
    assert!(sm.current_state == States::StateAny);
    println!("sm.data1={}", sm.data1);

    let (tx, rx): (Sender<Protocol1>, Receiver<Protocol1>) = std::sync::mpsc::channel();
    let msg = Protocol1::Get {
        hdr: Header {
            tx_response: Some(tx),
        },
        data1: 0,
    };
    sm.dispatch_msg(&msg);
    let res = rx.recv().unwrap();
    match &res {
        Protocol1::Get { hdr: _, data1 } => {
            println!("Get res={}", data1);
            assert_eq!(*data1, sm.data1);
        }
        _ => panic!("Expected Get response res={:?}", res),
    }
}

fn fn_ptr_sm() {
    use expr_fsm_0::fn_ptr_state_machine::{Header, Protocol1, StateMachine};

    println!("\nstate_machine_current_state_fn_ptr");

    // Create a sm and validate it's in the expected state
    let mut sm = StateMachine::default();
    assert!(sm.current_state as usize == StateMachine::state_process_msg_add_or_mul as usize);

    // Dispatch the message and validate it transitioned
    let msg = Protocol1::Add {
        hdr: Header { tx_response: None },
        f1: 123,
    };
    sm.dispatch_msg(&msg);
    assert!(sm.current_state as usize == StateMachine::state_process_msg_any as usize);
    println!("sm.data1={}", sm.data1);

    // Dispatch the message and validate it transitioned
    sm.dispatch_msg(&msg);
    assert!(sm.current_state as usize == StateMachine::state_process_msg_add_or_mul as usize);
    println!("sm.data1={}", sm.data1);

    // Dispatch the message and validate it transitioned
    sm.dispatch_msg(&msg);
    assert!(sm.current_state as usize == StateMachine::state_process_msg_any as usize);
    println!("sm.data1={}", sm.data1);

    let (tx, rx): (Sender<Protocol1>, Receiver<Protocol1>) = std::sync::mpsc::channel();
    let msg = Protocol1::Get {
        hdr: Header {
            tx_response: Some(tx),
        },
        data1: 0,
    };
    sm.dispatch_msg(&msg);
    let res = rx.recv().unwrap();
    match &res {
        Protocol1::Get { hdr: _, data1 } => {
            println!("Get res={}", data1);
            assert_eq!(*data1, sm.data1);
        }
        _ => panic!("Expected Get response res={:?}", res),
    }
}

fn msg_passing_one_thread_fn_ptr() {
    use expr_fsm_0::fn_ptr_state_machine::{Header, Protocol1, StateMachine};

    println!("\nmsg_passing_one_thread_fn_ptr:+");

    let (tx, rx): (Sender<Protocol1>, Receiver<Protocol1>) = mpsc::channel();

    let mut sm = StateMachine::default();
    let start = SystemTime::now();
    for _ in 0..10 {
        let msg_add = Protocol1::Add {
            hdr: Header { tx_response: None },
            f1: 1,
        };
        tx.send(msg_add).unwrap();
        let msg = rx.recv().unwrap();
        sm.dispatch_msg(&msg);
    }
    let duration = start.elapsed().unwrap();
    assert_eq!(sm.data1, 10);
    println!(
        "msg_passing_one_thread_fn_ptr:- result={} time={}ns",
        sm.data1,
        duration.as_nanos()
    );
}

fn msg_passing_two_threads_fn_ptr() {
    use expr_fsm_0::fn_ptr_state_machine::{Header, Protocol1, StateMachine};

    println!("\nmsg_passing_two_threads_fn_ptr:+");

    let (tx_work, rx_work): (Sender<Protocol1>, Receiver<Protocol1>) = mpsc::channel();
    let start = SystemTime::now();
    let receiver_thread = std::thread::spawn(move || {
        let mut sm = StateMachine::default();
        while let Ok(msg) = rx_work.recv() {
            sm.dispatch_msg(&msg);
        }
    });

    for _ in 0..10 {
        let msg_add = Protocol1::Add {
            hdr: Header { tx_response: None },
            f1: 1,
        };
        tx_work.send(msg_add).unwrap();
    }

    // Get result using a separate channel
    let (tx_result, rx_result): (Sender<Protocol1>, Receiver<Protocol1>) = mpsc::channel();
    let msg_get = Protocol1::Get {
        hdr: Header {
            tx_response: Some(tx_result),
        },
        data1: 0,
    };
    tx_work.send(msg_get).unwrap();
    drop(tx_work);

    let result = rx_result.recv().unwrap();
    drop(rx_result);

    let result_data1 = match result {
        Protocol1::Get { hdr: _, data1 } => data1,
        _ => panic!("Unexpected result={:?}", result),
    };
    let duration = start.elapsed().unwrap();
    assert_eq!(result_data1, 10);

    receiver_thread.join().unwrap();

    println!(
        "msg_passing_two_threads_fn_ptr:- result={} time={}ns",
        result_data1,
        duration.as_nanos()
    );
}

fn main() {
    env_logger_init("info");
    log::debug!("main:+");

    enum_sm();
    fn_ptr_sm();

    msg_passing_one_thread_fn_ptr();
    msg_passing_two_threads_fn_ptr();

    log::debug!("main:-");
}
