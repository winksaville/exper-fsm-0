use criterion::{criterion_group, criterion_main, black_box, Criterion};

use expr_fsm_0::state_machine_current_state_fn_ptr::{StateMachine, Protocol1};

pub fn dispatch_add_msg_to_state_add_current_state_fn_ptr(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_add_current_state_fn_ptr", |b| {
        let initial_state = StateMachine::state_add_process_msg;
        let mut sm = StateMachine::new(& initial_state);
        let msg = Protocol1::Add { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_add_msg_to_state_any_current_state_fn_ptr(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_any_current_state_fn_ptr", |b| {
        let initial_state = StateMachine::state_any_process_msg;
        let mut sm = StateMachine::new(& initial_state);
        let msg = Protocol1::Add { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_mul_msg_to_state_mul_current_state_fn_ptr(c: &mut Criterion) {
    c.bench_function("dispatch_mul_msg_to_state_mul_current_state_fn_ptr", |b| {
        let initial_state = StateMachine::state_mul_process_msg;
        let mut sm = StateMachine::new(& initial_state);
        let msg = Protocol1::Mul { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_mul_msg_to_state_any_current_state_fn_ptr(c: &mut Criterion) {
    c.bench_function("dispatch_mul_msg_to_state_any_current_state_fn_ptr", |b| {
        let initial_state = StateMachine::state_any_process_msg;
        let mut sm = StateMachine::new(& initial_state);
        let msg = Protocol1::Mul { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

criterion_group!(benches_current_state_fn_ptrs,
    dispatch_add_msg_to_state_add_current_state_fn_ptr, dispatch_add_msg_to_state_any_current_state_fn_ptr,
    dispatch_mul_msg_to_state_mul_current_state_fn_ptr, dispatch_mul_msg_to_state_any_current_state_fn_ptr);
criterion_main!(benches_current_state_fn_ptrs);
