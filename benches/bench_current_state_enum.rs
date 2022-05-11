use criterion::{criterion_group, criterion_main, black_box, Criterion};

use expr_fsm_0::state_machine_current_state_enum::{StateMachine, Protocol1, States};

pub fn dispatch_add_msg_to_state_add_current_state_enum(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_add_current_state_enum", |b| {
        let mut sm = StateMachine::new(States::StateAdd);
        let msg = Protocol1::Add { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_add_msg_to_state_any_current_state_enum(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_any_current_state_enum", |b| {
        let mut sm = StateMachine::new(States::StateAny);
        let msg = Protocol1::Add { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_mul_msg_to_state_mul_current_state_enum(c: &mut Criterion) {
    c.bench_function("dispatch_mul_msg_to_state_mul_current_state_enum", |b| {
        let mut sm = StateMachine::new(States::StateMul);
        let msg = Protocol1::Mul { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_mul_msg_to_state_any_current_state_enum(c: &mut Criterion) {
    c.bench_function("dispatch_mul_msg_to_state_any_current_state_enum", |b| {
        let mut sm = StateMachine::new(States::StateAny);
        let msg = Protocol1::Mul { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

criterion_group!(benches_current_state_enum,
    dispatch_add_msg_to_state_add_current_state_enum, dispatch_add_msg_to_state_any_current_state_enum,
    dispatch_mul_msg_to_state_mul_current_state_enum, dispatch_mul_msg_to_state_any_current_state_enum);
criterion_main!(benches_current_state_enum);
