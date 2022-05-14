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

pub fn dispatch_get_to_state_any_one_thread_fn_ptr_sm(c: &mut Criterion) {
    c.bench_function("dispatch_get_to_state_any_one_thread_fn_ptr_sm", |b| {
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
                _ => panic!("Expected Get response res={:?}", res),
            }
        });
    });
}

criterion_group!(
    benches_fn_ptr_sm,
    dispatch_add_msg_to_state_add_fn_ptr_sm,
    dispatch_add_msg_to_state_any_fn_ptr_sm,
    dispatch_mul_msg_to_state_any_fn_ptr_sm,
    dispatch_get_to_state_any_one_thread_fn_ptr_sm,
);
criterion_main!(benches_fn_ptr_sm);
