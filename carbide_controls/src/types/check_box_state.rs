use serde::{Deserialize, Serialize};

use carbide_core::environment::Environment;
use carbide_core::prelude::{ValueRef, ValueRefMut};
use carbide_core::state::{BoolState, MapOwnedState, State, TState};

#[derive(Clone, Debug)]
pub struct CheckBoxState(TState<CheckBoxValue>);

#[derive(Clone, Debug, PartialEq)]
pub enum CheckBoxValue {
    True,
    Intermediate,
    False,
}

impl Default for CheckBoxValue {
    fn default() -> Self {
        CheckBoxValue::False
    }
}

impl State<CheckBoxValue> for CheckBoxState {
    fn capture_state(&mut self, env: &mut Environment) {
        self.0.capture_state(env)
    }

    fn release_state(&mut self, env: &mut Environment) {
        self.0.release_state(env)
    }

    fn value(&self) -> ValueRef<CheckBoxValue> {
        self.0.value()
    }

    fn value_mut(&mut self) -> ValueRefMut<CheckBoxValue> {
        self.0.value_mut()
    }

    fn set_value(&mut self, value: CheckBoxValue) {
        self.0.set_value(value)
    }

    fn update_dependent(&mut self) {
        self.0.update_dependent()
    }
}

impl Into<CheckBoxState> for TState<CheckBoxValue> {
    fn into(self) -> CheckBoxState {
        CheckBoxState(self)
    }
}

impl CheckBoxState {
    fn from_bool(from: &bool, current: &CheckBoxValue, env: &Environment) -> CheckBoxValue {
        if *from {
            CheckBoxValue::True
        } else {
            CheckBoxValue::False
        }
    }

    fn val_to_bool(to: &CheckBoxValue) -> Option<bool> {
        match to {
            CheckBoxValue::True => {
                Some(true)
            }
            CheckBoxValue::Intermediate | CheckBoxValue::False => {
                Some(false)
            }
        }
    }
}

impl Into<CheckBoxState> for BoolState {
    fn into(self) -> CheckBoxState {
        CheckBoxState(
            MapOwnedState::new_with_default_and_rev(
                self,
                CheckBoxState::from_bool,
                CheckBoxState::val_to_bool,
                CheckBoxValue::False,
            ).into()
        )
    }
}

impl Into<TState<CheckBoxValue>> for CheckBoxState {
    fn into(self) -> TState<CheckBoxValue> {
        self.0
    }
}
