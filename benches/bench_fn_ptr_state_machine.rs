use criterion::{criterion_group, criterion_main, black_box, Criterion};

use expr_fsm_0::fn_ptr_state_machine::{StateMachine, Protocol1};

pub fn dispatch_add_msg_to_state_add_fn_ptr_sm(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_add_fn_ptr_sm", |b| {
        let initial_state = StateMachine::state_add_process_msg;
        let mut sm = StateMachine::new(& initial_state);
        let msg = Protocol1::Add { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_add_msg_to_state_any_fn_ptr_sm(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_any_fn_ptr_sm", |b| {
        let initial_state = StateMachine::state_any_process_msg;
        let mut sm = StateMachine::new(& initial_state);
        let msg = Protocol1::Add { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_mul_msg_to_state_mul_fn_ptr_sm(c: &mut Criterion) {
    c.bench_function("dispatch_mul_msg_to_state_mul_fn_ptr_sm", |b| {
        let initial_state = StateMachine::state_mul_process_msg;
        let mut sm = StateMachine::new(& initial_state);
        let msg = Protocol1::Mul { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_mul_msg_to_state_any_fn_ptr_sm(c: &mut Criterion) {
    c.bench_function("dispatch_mul_msg_to_state_any_fn_ptr_sm", |b| {
        let initial_state = StateMachine::state_any_process_msg;
        let mut sm = StateMachine::new(& initial_state);
        let msg = Protocol1::Mul { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

criterion_group!(benches_fn_ptr_sm,
    dispatch_add_msg_to_state_add_fn_ptr_sm, dispatch_add_msg_to_state_any_fn_ptr_sm,
    dispatch_mul_msg_to_state_mul_fn_ptr_sm, dispatch_mul_msg_to_state_any_fn_ptr_sm);
criterion_main!(benches_fn_ptr_sm);
