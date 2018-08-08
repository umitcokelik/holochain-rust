use hash_table::entry::Entry;
use holochain_dna::Dna;
use nucleus::{EntrySubmission, FunctionCall, FunctionResult};
use snowflake;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
// @TODO what is wrapper for?
// https://github.com/holochain/holochain-rust/issues/192
pub struct ActionWrapper {
    action: Action,
    id: snowflake::ProcessUniqueId,
}

impl ActionWrapper {
    /// immutable constructor from &Action
    /// internal snowflake ID is automatically set
    pub fn new(a: &Action) -> Self {
        ActionWrapper {
            action: a.clone(),
            // auto generate id
            id: snowflake::ProcessUniqueId::new(),
        }
    }

    /// read only access to action
    pub fn action(&self) -> Action {
        self.action.clone()
    }
}

impl PartialEq for ActionWrapper {
    fn eq(&self, other: &ActionWrapper) -> bool {
        self.id == other.id
    }
}

impl Eq for ActionWrapper {}

impl Hash for ActionWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(PartialEq, Clone, Hash, Debug)]
pub struct Action {
    signal: Signal,
    id: snowflake::ProcessUniqueId,
}

impl Action {
    /// immutable constructor from &Signal
    /// snowflake ID is auto generated
    pub fn new(signal: &Signal) -> Action {
        Action {
            signal: signal.clone(),
            // auto generate id
            id: snowflake::ProcessUniqueId::new(),
        }
    }

    /// read only access to the internal Signal
    pub fn signal(&self) -> Signal {
        self.signal.clone()
    }
}

impl Eq for Action {}

#[derive(Clone, PartialEq, Hash, Debug)]
pub enum Signal {
    /// entry to Commit
    /// MUST already have passed all lifecycle checks
    Commit(Entry),
    /// hash to Get
    Get(String),

    /// execute a function in a zome WASM
    ExecuteZomeFunction(FunctionCall),
    /// return the result of a zome WASM function call
    ReturnZomeFunctionResult(FunctionResult),

    /// initialize an application from a Dna
    /// not the same as genesis
    /// may call genesis internally
    InitApplication(Dna),
    /// return the result of an InitApplication action
    ReturnInitializationResult(Option<String>),

    /// ???
    // @TODO how does this relate to validating a commit?
    ValidateEntry(EntrySubmission),

    /// add a network peer
    AddPeer(String),
}

#[cfg(test)]
pub mod tests {

    use action::{Action, Signal, ActionWrapper};
    use hash_table::entry::tests::test_entry;
    use hash_table::entry::tests::test_entry_hash;

    /// dummy signal
    pub fn test_signal() -> Signal {
        Signal::Get(test_entry_hash())
    }

    /// dummy action with test_signal()
    pub fn test_action() -> Action {
        Action::new(&test_signal())
    }

    /// dummy action with commit of test_entry()
    pub fn test_action_commit() -> Action {
        Action::new(&Signal::Commit(test_entry()))
    }

    /// dummy action wrapper with test_action()
    pub fn test_action_wrapper() -> ActionWrapper {
        ActionWrapper::new(&test_action())
    }

    #[test]
    /// smoke test signals
    pub fn new_signal() {
        let s1 = test_signal();
        let s2 = test_signal();

        // unlike actions and wrappers, signals are equal to themselves
        assert_eq!(s1, s2);
    }

    #[test]
    /// tests that new actions take a signal and ensure uniqueness
    pub fn new_action() {
        let a1 = test_action();
        let a2 = test_action();

        // snowflake enforces uniqueness
        assert_eq!(a1, a1);
        assert_ne!(a1, a2);
    }

    #[test]
    /// tests read access to action signals
    pub fn action_signal() {
        let a1 = test_action();
        let a2 = test_action();

        assert_eq!(a1.signal(), a2.signal());
        assert_eq!(a1.signal(), test_signal());
    }

    #[test]
    /// tests action wrappers take actions and ensure uniqueness
    pub fn new_action_wrapper() {
        let w1 = test_action_wrapper();
        let w2 = test_action_wrapper();

        // snowflake enforces uniqueness
        assert_eq!(w1, w1);
        assert_ne!(w1, w2);
    }

    #[test]
    /// read access to action from wrapper
    pub fn action_wrapper_action() {
        let a = test_action();
        let w = ActionWrapper::new(&a);

        assert_eq!(w.action(), a);
        // new actions won't be equal due to id
        assert_ne!(w.action(), test_action());
    }

}
