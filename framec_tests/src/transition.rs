//! Frame supports two different operations for changing the current state of
//! the machine: "change-state" (`->>`) which simply changes to the new state,
//! and "transition" (`->`), which also sends an exit event to the old state
//! and an enter event to the new state.
//!
//! This file tests that these operations work correctly. It also tests that
//! the optional hook methods for each operation are invoked when states are
//! changed, and that transition callbacks registered via the runtime system
//! are invoked.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "transition.rs"));

#[allow(dead_code)]
impl<'a> Transition<'a> {
    pub fn enter(&mut self, state: String) {
        self.enters.push(state);
    }
    pub fn exit(&mut self, state: String) {
        self.exits.push(state);
    }
    pub fn clear_all(&mut self) {
        self.enters.clear();
        self.exits.clear();
        self.hooks.clear();
    }
    pub fn transition_hook(&mut self, old_state: TransitionState, new_state: TransitionState) {
        let s = format!("{:?}->{:?}", old_state, new_state);
        self.hooks.push(s);
    }
    pub fn change_state_hook(&mut self, old_state: TransitionState, new_state: TransitionState) {
        let s = format!("{:?}->>{:?}", old_state, new_state);
        self.hooks.push(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::transition::*;
    use std::sync::Mutex;

    #[test]
    /// Test that transition works and triggers enter and exit events.
    fn transition_events() {
        let mut sm = Transition::new();
        sm.clear_all();
        sm.transit();
        assert_eq!(sm.state, TransitionState::S1);
        assert_eq!(sm.exits, vec!["S0"]);
        assert_eq!(sm.enters, vec!["S1"]);
    }

    #[test]
    /// Test that change-state works and does not trigger events.
    fn change_state_no_events() {
        let mut sm = Transition::new();
        sm.clear_all();
        sm.change();
        assert_eq!(sm.state, TransitionState::S1);
        sm.change();
        assert_eq!(sm.state, TransitionState::S2);
        sm.change();
        assert_eq!(sm.state, TransitionState::S3);
        sm.change();
        assert_eq!(sm.state, TransitionState::S4);
        assert!(sm.exits.is_empty());
        assert!(sm.enters.is_empty());
    }

    #[test]
    /// Test transition that triggers another transition in an enter event
    /// handler.
    fn cascading_transition() {
        let mut sm = Transition::new();
        sm.change();
        sm.clear_all();
        assert_eq!(sm.state, TransitionState::S1);
        sm.transit();
        assert_eq!(sm.state, TransitionState::S3);
        assert_eq!(sm.exits, vec!["S1", "S2"]);
        assert_eq!(sm.enters, vec!["S2", "S3"]);
    }

    #[test]
    /// Test transition that triggers a change-state from an enter event
    /// handler.
    fn cascading_change_state() {
        let mut sm = Transition::new();
        sm.change();
        sm.change();
        sm.change();
        sm.clear_all();
        assert_eq!(sm.state, TransitionState::S3);
        sm.transit();
        assert_eq!(sm.state, TransitionState::S0);
        assert_eq!(sm.exits, vec!["S3"]);
        assert_eq!(sm.enters, vec!["S4"]);
    }

    /// Function to register as a callback to log transitions.
    fn log_transits(log: &Mutex<Log>, info: &TransitionInfo) {
        let old_state = info.old_state.name();
        let new_state = info.new_state.name();
        match info.kind {
            TransitionKind::ChangeState => {
                log.lock()
                    .unwrap()
                    .push(format!("{}->>{}", old_state, new_state));
            }
            TransitionKind::Transition => {
                log.lock()
                    .unwrap()
                    .push(format!("{}->{}", old_state, new_state));
            }
        }
    }

    #[test]
    /// Test transition callbacks.
    fn transition_callback() {
        let transits = Mutex::new(Vec::new());
        let mut sm = Transition::new();
        sm.callback_manager().add_transition_callback(|i| {
            log_transits(&transits, i);
        });
        sm.transit();
        assert_eq!(*transits.lock().unwrap(), vec!["S0->S1"]);
        transits.lock().unwrap().clear();
        sm.transit();
        assert_eq!(*transits.lock().unwrap(), vec!["S1->S2", "S2->S3"]);
    }

    #[test]
    /// Test change-state callbacks.
    fn change_state_callback() {
        let transits = Mutex::new(Vec::new());
        let mut sm = Transition::new();
        sm.callback_manager().add_transition_callback(|i| {
            log_transits(&transits, i);
        });
        sm.change();
        assert_eq!(*transits.lock().unwrap(), vec!["S0->>S1"]);
        transits.lock().unwrap().clear();
        sm.change();
        assert_eq!(*transits.lock().unwrap(), vec!["S1->>S2"]);
        transits.lock().unwrap().clear();
        sm.change();
        assert_eq!(*transits.lock().unwrap(), vec!["S2->>S3"]);
        transits.lock().unwrap().clear();
        sm.transit();
        assert_eq!(*transits.lock().unwrap(), vec!["S3->S4", "S4->>S0"]);
    }

    #[test]
    /// Test transition hook method.
    fn transition_hook() {
        let mut sm = Transition::new();
        sm.transit();
        assert_eq!(sm.hooks, vec!["S0->S1"]);
        sm.clear_all();
        sm.transit();
        assert_eq!(sm.hooks, vec!["S1->S2", "S2->S3"]);
    }

    #[test]
    /// Test change-state hook method.
    fn change_state_hook() {
        let mut sm = Transition::new();
        sm.change();
        assert_eq!(sm.hooks, vec!["S0->>S1"]);
        sm.clear_all();
        sm.change();
        assert_eq!(sm.hooks, vec!["S1->>S2"]);
        sm.clear_all();
        sm.change();
        assert_eq!(sm.hooks, vec!["S2->>S3"]);
        sm.clear_all();
        sm.transit();
        assert_eq!(sm.hooks, vec!["S3->S4", "S4->>S0"]);
    }
}
