//! This module provides a generic interface to the various ways that name-
//! bindings are realized in the code generated by Frame.
//!
//! Some brief design rationale:
//!
//!  * Environments are defined as a trait rather than a simple HashMap so that
//!    we can directly reuse the various context structs generated by Frame
//!    rather than constructing a bunch of parallel data structures each time
//!    we query the state of the machine.
//!  
//!  * It is common for a particular environment to be absent because there are
//!    no variables of the kind held by that environment. We represent absent
//!    environments by an empty environment rather than using an `Option` type
//!    because it simplifies the interface and because the distinction between
//!    `None` and `Some(EMPTY)` is not significant.

use std::any::Any;

/// Environments associate names (i.e. variables/parameters) with values.
pub trait Environment {
    /// Is this the empty environment?
    fn is_empty(&self) -> bool {
        false
    }
    /// Get the value associated with a name.
    fn lookup(&self, name: &str) -> Option<&dyn Any>;
}

/// The trivial empty environment. This can be used in place of an environment
/// when that environment is absent.
pub const EMPTY: &'static dyn Environment = &EmptyEnvironment {};

struct EmptyEnvironment {}

impl Environment for EmptyEnvironment {
    fn is_empty(&self) -> bool {
        true
    }
    fn lookup(&self, _name: &str) -> Option<&dyn Any> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestArgs {
        x: i32,
        y: bool,
        z: Option<f32>,
    }

    impl Environment for TestArgs {
        fn is_empty(&self) -> bool {
            false
        }
        fn lookup(&self, name: &str) -> Option<&dyn Any> {
            match name {
                "x" => Some(&self.x),
                "y" => Some(&self.y),
                "z" => Some(&self.z),
                _ => None,
            }
        }
    }

    #[test]
    fn empty_environment_is_empty() {
        assert!(EMPTY.is_empty());
    }

    #[test]
    fn empty_environment_returns_none() {
        assert!(EMPTY.lookup("x").is_none());
        assert!(EMPTY.lookup("y").is_none());
        assert!(EMPTY.lookup("z").is_none());
    }

    #[test]
    fn struct_environment_not_empty() {
        let args = TestArgs {
            x: 42,
            y: false,
            z: Some(3.14),
        };
        assert!(!args.is_empty());
    }

    #[test]
    fn struct_environment_lookup_success() {
        let args = TestArgs {
            x: 42,
            y: false,
            z: Some(3.14),
        };

        let opt_x = args.lookup("x");
        assert!(opt_x.is_some());
        let opt_i32 = opt_x.unwrap().downcast_ref::<i32>();
        assert!(opt_i32.is_some());
        assert_eq!(*opt_i32.unwrap(), 42);

        let opt_y = args.lookup("y");
        assert!(opt_y.is_some());
        let opt_bool = opt_y.unwrap().downcast_ref::<bool>();
        assert!(opt_bool.is_some());
        assert_eq!(*opt_bool.unwrap(), false);

        let opt_y = args.lookup("z");
        assert!(opt_y.is_some());
        let opt_bool = opt_y.unwrap().downcast_ref::<Option<f32>>();
        assert!(opt_bool.is_some());
        assert_eq!(*opt_bool.unwrap(), Some(3.14));
    }

    #[test]
    fn struct_environment_lookup_type_error() {
        let args = TestArgs {
            x: 42,
            y: false,
            z: Some(3.14),
        };
        let opt_x = args.lookup("x");
        assert!(opt_x.is_some());
        assert!(opt_x.unwrap().downcast_ref::<bool>().is_none());

        let opt_y = args.lookup("y");
        assert!(opt_y.is_some());
        assert!(opt_y.unwrap().downcast_ref::<i32>().is_none());

        let opt_z = args.lookup("z");
        assert!(opt_z.is_some());
        assert!(opt_y.unwrap().downcast_ref::<f32>().is_none());
        assert!(opt_y.unwrap().downcast_ref::<Option<i32>>().is_none());
    }

    #[test]
    fn struct_environment_lookup_undefined() {
        let args = TestArgs {
            x: 42,
            y: false,
            z: Some(3.14),
        };
        assert!(args.lookup("w").is_none());
    }
}
