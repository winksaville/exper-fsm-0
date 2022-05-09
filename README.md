# expr-fsm-0

A first experiment with state machine. I ran into a problem with expr-fsm-0
which uses traits, dispatch invokes process and process needs second mutable
reference of StateMachine. Can't have to mutable references in Rust!!! This
is actually based on expr-traits-1 (which was based on expr-fsm-1). Hence this
is expr-fsm-0 :)

So I'm trying simpler yet.


## Building and running


## Test

TODO

## Benchmark

TODO

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
