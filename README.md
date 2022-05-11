# expr-fsm-0

A experiment with state machine. I ran into a problem with expr-fsm-1
which uses traits. That version doesn't compile becasuse there ends up
to be two mutable references. So this is an attempt without using traits.

I've tried two techniques one where `current_state: States` and `States` is
and enum. And the other where `current_state: &'a dyn Fn(&mut Self, &Protocol1)`.

Using the Fn Pointer, is "easier" no enum needs to be created and the
dispatch routine doesn't need a `match` statement. I created a benchmark
and they are basically the same speed. Although this is quite contrived.

Using Enum:
```
pub mod state_machine_current_state_enum {

    pub enum Protocol1 {
        Add { f1: i32 },
        Sub { f1: i32 },
        Mul { f1: i32 },
    }

    pub enum States {
        StateAdd,
        StateSub,
        StateMul,
        StateAny,
    }

    pub struct StateMachine {
        pub current_state: States,
        pub data1: i32,
    }

    impl StateMachine {
        pub fn new(initial_state: States) -> Self {
            StateMachine {
                current_state: initial_state,
                data1: 0,
            }
        }

        pub fn dispatch_msg(&mut self, msg: &Protocol1) {
            match self.current_state {
                States::StateAdd => {
                    self.state_add_process_msg(msg);
                }
                States::StateSub => {
                    self.state_sub_process_msg(msg);
                }
                States::StateMul => {
                    self.state_mul_process_msg(msg);
                }
                States::StateAny => {
                    self.state_any_process_msg(msg);
                }
            }
        }

        pub fn state_any_process_msg(&mut self, msg: &Protocol1) {
            match *msg {
                Protocol1::Add { f1 } => {
                    self.data1 += f1;
                }
                Protocol1::Sub { f1 } => {
                    self.data1 -= f1;
                }
                Protocol1::Mul { f1 } => {
                    self.data1 *= f1;
                }
            }

	... More States ...
	}
    }
}
```

Using Fn Pointer:
```
pub mod state_machine_current_state_fn_ptr {

    pub enum Protocol1 {
        Add { f1: i32 },
        Sub { f1: i32 },
        Mul { f1: i32 },
    }

    pub struct StateMachine<'a> {
        pub current_state: &'a dyn Fn(&mut Self, &Protocol1),
        pub data1: i32,
    }

    impl<'a> StateMachine<'a> {
        pub fn new(initial_state: &'a dyn Fn(&mut Self, &Protocol1)) -> Self {
            StateMachine {
                current_state: initial_state,
                data1: 0,
            }
        }

        pub fn dispatch_msg(&mut self, msg: &Protocol1) {
            (self.current_state)(self, msg);
        }

        pub fn state_any_process_msg(&mut self, msg: &Protocol1) {
            match *msg {
                Protocol1::Add { f1 } => {
                    self.data1 += f1;
                }
                Protocol1::Sub { f1 } => {
                    self.data1 -= f1;
                }
                Protocol1::Mul { f1 } => {
                    self.data1 *= f1;
                }
            }
	}

	... More States ...
    }
}
```

The main I used is:
```
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
```

## Building and running

```
wink@3900x 22-05-11T21:43:45.987Z:~/prgs/rust/myrepos/expr-fsm-0 (main)
$ cargo run
   Compiling expr-fsm-0 v0.1.0 (/home/wink/prgs/rust/myrepos/expr-fsm-0)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/expr-fsm-0`
state_machine_current_state_enum
sm.data1=123
sm.data1=246
state_machine_current_state_fn_ptr
sm.data1=123
sm.data1=246
wink@3900x 22-05-11T22:08:20.604Z:~/prgs/rust/myrepos/expr-fsm-0 (main)
```


## Test

TODO

## Benchmark

```
wink@3900x 22-05-11T21:24:19.704Z:~/prgs/rust/myrepos/expr-fsm-0 (main)
$ cargo criterion
   Compiling expr-fsm-0 v0.1.0 (/home/wink/prgs/rust/myrepos/expr-fsm-0)
    Finished bench [optimized] target(s) in 1.25s
Gnuplot not found, using plotters backend
dispatch_add_msg_to_state_add_current_state_enum                                                                             
                        time:   [1.7430 ns 1.7441 ns 1.7454 ns]
                        change: [-6.6180% -5.4456% -4.5977%] (p = 0.00 < 0.05)
                        Performance has improved.

dispatch_add_msg_to_state_any_current_state_enum                                                                             
                        time:   [1.8240 ns 1.8246 ns 1.8254 ns]
                        change: [+2.5249% +2.6857% +2.8421%] (p = 0.00 < 0.05)
                        Performance has regressed.

dispatch_mul_msg_to_state_mul_current_state_enum                                                                             
                        time:   [2.1922 ns 2.2015 ns 2.2132 ns]
                        change: [+0.9558% +1.4504% +1.8852%] (p = 0.00 < 0.05)
                        Change within noise threshold.

dispatch_mul_msg_to_state_any_current_state_enum                                                                             
                        time:   [2.2081 ns 2.2199 ns 2.2337 ns]
                        change: [-2.1118% -1.7525% -1.3912%] (p = 0.00 < 0.05)
                        Performance has improved.

Gnuplot not found, using plotters backend
dispatch_add_msg_to_state_add_current_state_fn_ptr                                                                             
                        time:   [1.9494 ns 1.9797 ns 2.0117 ns]
                        change: [+2.1136% +3.5381% +4.8978%] (p = 0.00 < 0.05)
                        Performance has regressed.

dispatch_add_msg_to_state_any_current_state_fn_ptr                                                                             
                        time:   [1.8330 ns 1.8350 ns 1.8380 ns]
                        change: [-4.2148% -3.0492% -1.8726%] (p = 0.00 < 0.05)
                        Performance has improved.

dispatch_mul_msg_to_state_mul_current_state_fn_ptr                                                                             
                        time:   [2.3117 ns 2.3191 ns 2.3267 ns]
                        change: [-2.8740% -2.3532% -1.8451%] (p = 0.00 < 0.05)
                        Performance has improved.

dispatch_mul_msg_to_state_any_current_state_fn_ptr                                                                             
                        time:   [2.2650 ns 2.2658 ns 2.2668 ns]
                        change: [+2.9005% +3.2917% +3.8221%] (p = 0.00 < 0.05)
                        Performance has regressed.


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
