use std::fmt::Debug;
use std::rc::Rc;

use crate::Color;
use crate::draw::{Dimension, Position};
use crate::focus::Focus;
pub use crate::state::value_cell::{ValueCell, ValueRef, ValueRefMut};

pub use self::animated_state::*;
pub use self::animation_curve::*;
pub use self::async_state::*;
pub use self::env_state::EnvState;
pub use self::global_state::GlobalState;
pub use self::local_state::LocalState;
pub use self::map_owned_state::*;
pub use self::map_state::*;
pub use self::state::State;
pub use self::state_ext::*;
pub use self::state_key::StateKey;
pub use self::state_sync::StateSync;
pub use self::value_state::ValueState;
pub use self::widget_state::WidgetState;

mod animated_state;
mod animation_curve;
mod env_state;
mod global_state;
mod local_state;
mod map_owned_state;
mod map_state;
mod state;
mod state_key;
mod state_sync;
mod value_cell;
mod value_state;
mod vec_state;
mod widget_state;
mod state_ext;
mod async_state;

pub(crate) type InnerState<T> = Rc<ValueCell<T>>;

pub type ColorState = TState<Color>;
pub type StringState = TState<String>;
pub type ResStringState = TState<Result<String, String>>;
pub type U32State = TState<u32>;
pub type I32State = TState<i32>;
pub type UsizeState = TState<usize>;
pub type BoolState = TState<bool>;
pub type F64State = TState<f64>;
pub type FocusState = TState<Focus>;
pub type PositionState = TState<Position>;
pub type DimensionState = TState<Dimension>;
pub type TState<T> = WidgetState<T>;

pub trait StateContract: Clone + Debug {}

impl<T> StateContract for T where T: Clone + Debug {}
