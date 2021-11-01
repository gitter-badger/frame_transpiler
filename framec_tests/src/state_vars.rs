include!(concat!(env!("OUT_DIR"), "/", "state_vars.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::Environment;

    #[test]
    fn single_variable() {
        let mut sm = StateVars::new();
        assert_eq!(sm.state, StateVarsState::A);
        assert_eq!(sm.state_context.a_context().borrow().state_vars.x, 0);
        sm.x(); // increment x
        sm.x(); // increment x
        assert_eq!(sm.state_context.a_context().borrow().state_vars.x, 2);
    }

    #[test]
    fn multiple_variables() {
        let mut sm = StateVars::new();
        sm.y(); // transition to B
        assert_eq!(sm.state, StateVarsState::B);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.y, 10);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.z, 100);
        sm.y(); // increment y
        sm.y(); // increment y
        sm.z(); // increment z
        sm.y(); // increment y
        assert_eq!(sm.state_context.b_context().borrow().state_vars.y, 13);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.z, 101);
    }

    #[test]
    fn variables_are_reset() {
        let mut sm = StateVars::new();
        sm.x(); // increment x
        sm.x(); // increment x
        assert_eq!(sm.state_context.a_context().borrow().state_vars.x, 2);
        sm.z(); // transition to B
        sm.z(); // increment z
        sm.y(); // increment y
        sm.z(); // increment z
        assert_eq!(sm.state_context.b_context().borrow().state_vars.y, 11);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.z, 102);
        sm.x(); // transition to A
        assert_eq!(sm.state_context.a_context().borrow().state_vars.x, 0);
        sm.y(); // transition to B
        assert_eq!(sm.state_context.b_context().borrow().state_vars.y, 10);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.z, 100);
    }

    /// Helper function to lookup a `u32` value in an environment.
    /// Returns `u32::MAX` if the lookup fails for any reason.
    fn lookup_u32(env: &(impl Environment + ?Sized), name: &str) -> u32 {
        match env.lookup(name) {
            None => u32::MAX,
            Some(any) => *any.downcast_ref().unwrap_or(&u32::MAX),
        }
    }

    #[test]
    /// Tests that state variables behave as expected when accessed via the
    /// runtime interface.
    fn runtime_variables() {
        let mut sm = StateVars::new();
        assert_eq!(lookup_u32(sm.state().variables(), "x"), 0);
        assert!(sm.state().variables().lookup("y").is_none());
        assert!(sm.state().variables().lookup("z").is_none());
        sm.x(); // increment x
        sm.x(); // increment x
        assert_eq!(lookup_u32(sm.state().variables(), "x"), 2);
        sm.z(); // transition to B
        sm.z(); // increment z
        sm.y(); // increment y
        sm.z(); // increment z
        assert!(sm.state().variables().lookup("x").is_none());
        assert_eq!(lookup_u32(sm.state().variables(), "y"), 11);
        assert_eq!(lookup_u32(sm.state().variables(), "z"), 102);
        sm.x(); // transition to A
        assert_eq!(lookup_u32(sm.state().variables(), "x"), 0);
        assert!(sm.state().variables().lookup("y").is_none());
        assert!(sm.state().variables().lookup("z").is_none());
        sm.y(); // transition to B
        assert!(sm.state().variables().lookup("x").is_none());
        assert_eq!(lookup_u32(sm.state().variables(), "y"), 10);
        assert_eq!(lookup_u32(sm.state().variables(), "z"), 100);
    }
}
