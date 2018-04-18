extern crate redux_rs;

use redux_rs::*;

fn main() {
    let mut store = Store::new(test_reducer, 0);
    store.subscribe(|state| println!("{}", state));
    store.dispatch(CounterActions::INCREMENT(1));
    store.dispatch(CounterActions::DECREMENT(2));
    store.dispatch(CounterActions::INCREMENT(3));

    println!("{}", store.state());
}

enum CounterActions {
    INCREMENT(i32),
    DECREMENT(i32)
}

fn test_reducer(state: &i32, action: CounterActions) -> i32 {
    match action {
        CounterActions::INCREMENT(x) => state + x,
        CounterActions::DECREMENT(x) => state - x,
    }
}