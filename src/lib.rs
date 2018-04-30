pub type Reducer<TState, TAction> = fn(&TState, TAction) -> TState;
pub type Subscriber<TState> = Box<Fn(&TState)>;

pub struct Store<TState, TAction> {
    reducer: Reducer<TState, TAction>,
    state: TState,
    subscribers: Vec<Subscriber<TState>>,
}

impl<TState, TAction> Store<TState, TAction> {
    pub fn new(reducer: Reducer<TState, TAction>, default_state: TState) -> Store<TState, TAction> {
        Store::<TState, TAction> {
            reducer,
            state: default_state,
            subscribers: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, action: TAction) {
        self.state = (self.reducer)(&self.state, action);
        self.subscribers.iter().for_each(|subscriber| subscriber(&self.state));
    }

    pub fn subscribe(&mut self, subscriber: Subscriber<TState>) {
        self.subscribers.push(subscriber);
    }

    pub fn state(&self) -> &TState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum TestActions {
        INCREMENT(i32),
        DECREMENT(i32),
    }

    fn test_reducer(state: &i32, action: TestActions) -> i32 {
        match action {
            TestActions::INCREMENT(x) => state + x,
            TestActions::DECREMENT(x) => state - x,
        }
    }

    #[test]
    fn new_default_state_works() {
        // Create the store
        let store = Store::<i32, TestActions>::new(test_reducer, 1);

        // Test if the default state is set as desired
        let expected = &1;
        let actual = store.state();
        assert_eq!(expected, actual)
    }

    #[test]
    fn dispatch_increment_once() {
        // Create the store
        let mut store = Store::<i32, TestActions>::new(test_reducer, 1);

        // Dispatch our first action
        store.dispatch(TestActions::INCREMENT(1));

        // Test if the new state has the desired value
        let expected = &2;
        let actual = store.state();
        assert_eq!(expected, actual)
    }

    #[test]
    fn dispatch_decrement_once() {
        // Create the store
        let mut store = Store::<i32, TestActions>::new(test_reducer, 1);

        // Dispatch our first action
        store.dispatch(TestActions::DECREMENT(1));

        // Test if the new state has the desired value
        let expected = &0;
        let actual = store.state();
        assert_eq!(expected, actual)
    }

    #[test]
    fn dispatch_mixed() {
        // Create the store
        let mut store = Store::<i32, TestActions>::new(test_reducer, 0);

        // Dispatch multiple actions
        store.dispatch(TestActions::DECREMENT(12));
        store.dispatch(TestActions::DECREMENT(31));
        store.dispatch(TestActions::INCREMENT(15));
        store.dispatch(TestActions::DECREMENT(78));
        store.dispatch(TestActions::INCREMENT(12));
        store.dispatch(TestActions::INCREMENT(14));

        // Test if the new state has the desired value
        let expected = &-80;
        let actual = store.state();
        assert_eq!(expected, actual)
    }
}
