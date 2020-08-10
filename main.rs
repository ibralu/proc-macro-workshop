// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

// use std::sync::{Arc, Mutex};
// use std::task::Waker;
//
// pub struct TimerFuture {
// state: Arc<Mutex<SharedState>>
// }
// pub struct SharedState {
//     completed: bool,
//     waker: Option<Waker>
// }
fn main() {
    use derive_builder::Builder;

    #[derive(Builder)]
    pub struct Foo {
        pub bar: String,
        pub ha: Option<i32>,
    }
}
