use std::fmt::Debug;

use dyn_clone::DynClone;

use carbide_core::__private::Formatter;
use carbide_core::cursor::MouseCursor;
use carbide_core::draw::{Dimension, Position};
use carbide_core::environment::Environment;
use carbide_core::event::{Key, KeyboardEvent, KeyboardEventHandler, MouseButton, MouseEvent, MouseEventHandler, OtherEventHandler, WidgetEvent};
use carbide_core::flags::Flags;
use carbide_core::focus::Focus;
use carbide_core::layout::Layout;
use carbide_core::state::{BoolState, FocusState, State};
use carbide_core::widget::{CommonWidget, Id, Widget, WidgetExt, WidgetIter, WidgetIterMut};

pub trait Action: Fn(&mut Environment) + DynClone {}

impl<I> Action for I where I: Fn(&mut Environment) + Clone {}

dyn_clone::clone_trait_object!(Action);

#[derive(Clone, Widget)]
#[carbide_exclude(MouseEvent, KeyboardEvent, OtherEvent)]
pub struct PlainButton {
    id: Id,
    #[state]
    focus: FocusState,
    child: Box<dyn Widget>,
    position: Position,
    dimension: Dimension,
    click: Box<dyn Action>,
    click_outside: Box<dyn Action>,
    #[state]
    is_hovered: BoolState,
    #[state]
    is_pressed: BoolState,
    hover_cursor: MouseCursor,
    pressed_cursor: Option<MouseCursor>,
}

impl PlainButton {
    pub fn on_click(
        mut self,
        fire: impl Action + 'static,
    ) -> Box<Self> {
        self.click = Box::new(fire);
        Box::new(self)
    }

    pub fn on_click_outside(
        mut self,
        fire: impl Action + 'static,
    ) -> Box<Self> {
        self.click_outside = Box::new(fire);
        Box::new(self)
    }

    pub fn hover<K: Into<BoolState>>(mut self, is_hovered: K) -> Box<Self> {
        self.is_hovered = is_hovered.into();
        Box::new(self)
    }

    pub fn pressed<K: Into<BoolState>>(mut self, pressed: K) -> Box<Self> {
        self.is_pressed = pressed.into();
        Box::new(self)
    }

    pub fn focused<K: Into<FocusState>>(mut self, focused: K) -> Box<Self> {
        self.focus = focused.into();
        Box::new(self)
    }

    pub fn hover_cursor(mut self, cursor: MouseCursor) -> Box<Self> {
        self.hover_cursor = cursor;
        Box::new(self)
    }

    pub fn pressed_cursor(mut self, cursor: MouseCursor) -> Box<Self> {
        self.pressed_cursor = Some(cursor);
        Box::new(self)
    }

    pub fn new(child: Box<dyn Widget>) -> Box<Self> {
        Box::new(PlainButton {
            id: Id::new_v4(),
            focus: Focus::Unfocused.into(),
            child,
            position: Position::new(0.0, 0.0),
            dimension: Dimension::new(100.0, 100.0),
            click: Box::new(|_| {}),
            click_outside: Box::new(|_| {}),
            is_hovered: false.into(),
            is_pressed: false.into(),
            hover_cursor: MouseCursor::Hand,
            pressed_cursor: None,
        })
    }
}

impl OtherEventHandler for PlainButton {
    fn handle_other_event(&mut self, _event: &WidgetEvent, env: &mut Environment) {
        if *self.is_hovered.value() {
            env.set_cursor(self.hover_cursor);
        }
        if *self.is_pressed.value() {
            if let Some(cursor) = self.pressed_cursor {
                env.set_cursor(cursor);
            }
        }
    }
}

impl KeyboardEventHandler for PlainButton {
    fn handle_keyboard_event(&mut self, event: &KeyboardEvent, env: &mut Environment) {
        if self.get_focus() != Focus::Focused {
            return;
        }

        match event {
            KeyboardEvent::Click(Key::Return, _) => {
                (self.click)(env);
            }
            _ => (),
        }
    }
}

impl MouseEventHandler for PlainButton {
    fn handle_mouse_event(&mut self, event: &MouseEvent, _consumed: &bool, env: &mut Environment) {
        match event {
            MouseEvent::Press(MouseButton::Left, mouse_position, _) => {
                if self.is_inside(*mouse_position) {
                    *self.is_pressed.value_mut() = true;
                }
            }
            MouseEvent::Release(MouseButton::Left, mouse_position, _) => {
                if self.is_inside(*mouse_position) {
                    *self.is_pressed.value_mut() = false;
                }
            }
            MouseEvent::Move { to, .. } => {
                if *self.is_hovered.value() {
                    if !self.is_inside(*to) {
                        *self.is_hovered.value_mut() = false;
                        *self.is_pressed.value_mut() = false;
                    }
                } else {
                    if self.is_inside(*to) {
                        *self.is_hovered.value_mut() = true;
                    }
                }
            }
            MouseEvent::Click(MouseButton::Left, mouse_position, _)
            | MouseEvent::NClick(MouseButton::Left, mouse_position, _, _) => {
                if self.is_inside(*mouse_position) {
                    (self.click)(env);
                } else {
                    (self.click_outside)(env);
                }
            }
            _ => (),
        }
    }
}

impl CommonWidget for PlainButton {
    fn id(&self) -> Id {
        self.id
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }

    fn flag(&self) -> Flags {
        Flags::FOCUSABLE
    }

    fn get_focus(&self) -> Focus {
        self.focus.value().clone()
    }

    fn set_focus(&mut self, focus: Focus) {
        *self.focus.value_mut() = focus;
    }

    fn children(&self) -> WidgetIter {
        if self.child.flag() == Flags::PROXY {
            self.child.children()
        } else {
            WidgetIter::single(&self.child)
        }
    }

    fn children_mut(&mut self) -> WidgetIterMut {
        if self.child.flag() == Flags::PROXY {
            self.child.children_mut()
        } else {
            WidgetIterMut::single(&mut self.child)
        }
    }

    fn children_direct(&mut self) -> WidgetIterMut {
        WidgetIterMut::single(&mut self.child)
    }

    fn children_direct_rev(&mut self) -> WidgetIterMut {
        WidgetIterMut::single(&mut self.child)
    }

    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn dimension(&self) -> Dimension {
        self.dimension
    }

    fn set_dimension(&mut self, dimension: Dimension) {
        self.dimension = dimension
    }
}

impl Debug for PlainButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlainButton")
            .field("child", &self.child)
            .finish()
    }
}

impl WidgetExt for PlainButton {}
