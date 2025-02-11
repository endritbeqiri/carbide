use std::ops::{Deref, DerefMut};

use crate::Color;
use crate::prelude::{Environment, State};
use crate::prelude::EnvironmentColor;
use crate::state::{ValueRef, ValueRefMut};
use crate::state::StateKey;

#[derive(Clone, Debug)]
pub struct EnvironmentColorState {
    key: StateKey,
    value: Color,
}

impl EnvironmentColorState {
    pub fn new(key: EnvironmentColor) -> Self {
        EnvironmentColorState {
            key: StateKey::Color(key),
            value: Color::Rgba(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl Deref for EnvironmentColorState {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for EnvironmentColorState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl State<Color> for EnvironmentColorState {
    fn capture_state(&mut self, env: &mut Environment) {
        if let Some(color) = env.get_color(&self.key) {
            self.value = color;
        }
    }

    fn release_state(&mut self, _: &mut Environment) {}

    fn value(&self) -> ValueRef<Color> {
        ValueRef::Borrow(&self.value)
    }

    fn value_mut(&mut self) -> ValueRefMut<Color> {
        ValueRefMut::Borrow(&mut self.value)
    }

    fn set_value(&mut self, value: Color) {
        self.value = value;
    }
}
