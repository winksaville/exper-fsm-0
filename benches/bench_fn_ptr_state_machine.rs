use criterion::{black_box, criterion_group, criterion_main, Criterion};

use expr_fsm_0::fn_ptr_state_machine::{Header, Protocol1, StateMachine};

pub fn dispatch_add_msg_to_state_add_fn_ptr_sm(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_add_fn_ptr_sm", |b| {
        let mut sm = StateMachine::new();
        let msg = Protocol1::Add {
            hdr: Header { tx_response: None },
            f1: 1,
        };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_add_msg_to_state_any_fn_ptr_sm(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_any_fn_ptr_sm", |b| {
        let mut sm = StateMachine::new();
        let msg = Protocol1::Add {
            hdr: Header { tx_response: None },
            f1: 1,
        };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_mul_msg_to_state_any_fn_ptr_sm(c: &mut Criterion) {
    c.bench_function("dispatch_mul_msg_to_state_any_fn_ptr_sm", |b| {
        let mut sm = StateMachine::new();
        let msg = Protocol1::Add {
            hdr: Header { tx_response: None },
            f1: 1,
        };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn std_sync_mpsc_channel_get_one_thread_fn_ptr_sm(c: &mut Criterion) {
    c.bench_function("std_sync_mpsc_channel_get_one_thread_fn_ptr_sm", |b| {
        use std::sync::mpsc::{Receiver, Sender};
        let (tx, rx): (Sender<Protocol1>, Receiver<Protocol1>) = std::sync::mpsc::channel();
        let msg = Protocol1::Get {
            hdr: Header {
                tx_response: Some(tx),
            },
            data1: 0,
        };
        let mut sm = StateMachine::default();
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
            let res = rx.recv().unwrap();
            match &res {
                Protocol1::Get { hdr: _, data1 } => {
                    assert_eq!(*data1, sm.data1);
                }
                _ => panic!("Expected Protocol1::Get response but got: `{:?}`", res),
            }
        });
    });
}

pub fn std_sync_mpsc_channel_get_two_threads_fn_ptr_sm(c: &mut Criterion) {
    use std::sync::{
        mpsc,
        mpsc::{Receiver, Sender},
    };

    c.bench_function("std_sync_mpsc_channel_get_two_threads_fn_ptr_sm", |b| {
        let (tx_work, rx_work): (Sender<Protocol1>, Receiver<Protocol1>) = mpsc::channel();

        let receiver_thread = std::thread::spawn(move || {
            let mut sm = StateMachine::default();
            while let Ok(msg) = rx_work.recv() {
                sm.dispatch_msg(&msg);
            }
        });

        let (tx_result, rx_result): (Sender<Protocol1>, Receiver<Protocol1>) = mpsc::channel();
        b.iter(|| {
            // Create the Get Request Message
            let get_req_msg = Protocol1::Get {
                hdr: Header {
                    tx_response: Some(tx_result.clone()),
                },
                data1: 0,
            };

            // Send it to the state machine on the other thread
            tx_work.send(get_req_msg).unwrap();

            // Get the response value which should be the default value of zero
            let get_rsp_msg = rx_result.recv().unwrap();
            let data1 = match get_rsp_msg {
                Protocol1::Get { hdr: _, data1 } => data1,
                _ => panic!("Expected Protocol1::Get but got: `{:?}`", get_rsp_msg),
            };
            assert_eq!(data1, 0);
        });
        drop(tx_work);

        receiver_thread.join().unwrap();
        drop(rx_result);
    });
}

criterion_group!(
    benches_fn_ptr_sm,
    dispatch_add_msg_to_state_add_fn_ptr_sm,
    dispatch_add_msg_to_state_any_fn_ptr_sm,
    dispatch_mul_msg_to_state_any_fn_ptr_sm,
    std_sync_mpsc_channel_get_one_thread_fn_ptr_sm,
    std_sync_mpsc_channel_get_two_threads_fn_ptr_sm,
);
criterion_main!(benches_fn_ptr_sm);
