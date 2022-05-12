use criterion::{black_box, criterion_group, criterion_main, Criterion};

use expr_fsm_0::enum_state_machine::{Protocol1, StateMachine, States};

pub fn dispatch_add_msg_to_state_add_enum_sm(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_add_enum_sm", |b| {
        let mut sm = StateMachine::new(States::StateAdd);
        let msg = Protocol1::Add { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_add_msg_to_state_any_enum_sm(c: &mut Criterion) {
    c.bench_function("dispatch_add_msg_to_state_any_enum_sm", |b| {
        let mut sm = StateMachine::new(States::StateAny);
        let msg = Protocol1::Add { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_mul_msg_to_state_mul_enum_sm(c: &mut Criterion) {
    c.bench_function("dispatch_mul_msg_to_state_mul_enum_sm", |b| {
        let mut sm = StateMachine::new(States::StateMul);
        let msg = Protocol1::Mul { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

pub fn dispatch_mul_msg_to_state_any_enum_sm(c: &mut Criterion) {
    c.bench_function("dispatch_mul_msg_to_state_any_enum_sm", |b| {
        let mut sm = StateMachine::new(States::StateAny);
        let msg = Protocol1::Mul { f1: 1 };
        b.iter(|| {
            sm.dispatch_msg(black_box(&msg));
        });
    });
}

criterion_group!(
    benches_enum_sm,
    dispatch_add_msg_to_state_add_enum_sm,
    dispatch_add_msg_to_state_any_enum_sm,
    dispatch_mul_msg_to_state_mul_enum_sm,
    dispatch_mul_msg_to_state_any_enum_sm
);
criterion_main!(benches_enum_sm);
