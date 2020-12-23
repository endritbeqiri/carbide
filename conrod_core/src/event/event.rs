use crate::event_handler::{KeyboardEvent, MouseEvent, WidgetEvent};
use crate::state::environment::Environment;
use crate::state::state_sync::StateSync;
use crate::widget::common_widget::CommonWidget;

pub trait Event<S>: CommonWidget<S> + StateSync<S> {
    /// A function that will be called when a mouse event occurs.
    /// It will only get called on the events where the cursor is inside.
    /// Return true if the event is consumed, and will thus not be delegated to other
    /// widgets.
    fn handle_mouse_event(&mut self, event: &MouseEvent, consumed: &bool, global_state: &mut S);

    /// A function that will get called when a keyboard event occurs.
    /// This event will be given to all widgets, no matter if they are in focus or not.
    /// This is because the focus will be decided by the widgets themselves.
    fn handle_keyboard_event(&mut self, event: &KeyboardEvent, global_state: &mut S);

    /// This will get called if there are event that are not covered by the other functions.
    /// This will get delegated to all widgets.
    /// It will never get called with mouse or keyboard events.
    /// TODO: Separate touch events.
    fn handle_other_event(&mut self, event: &WidgetEvent);

    fn process_mouse_event(&mut self, event: &MouseEvent, consumed: &bool, env: &mut Environment, global_state: &mut S) {
        self.update_all_widget_state(env, global_state);

        self.handle_mouse_event(event, consumed, global_state);
        if *consumed { return () }

        self.insert_local_state(env);

        for child in self.get_proxied_children() {
            if child.is_inside(event.get_current_mouse_position()) {
                child.process_mouse_event(event, &consumed, env, global_state);
                if *consumed { return () }
            }
        }

        self.update_local_widget_state(env)
    }

    fn process_keyboard_event(&mut self, event: &KeyboardEvent, env: &mut Environment, global_state: &mut S) {
        self.update_all_widget_state(env, global_state);

        self.handle_keyboard_event(event, global_state);

        self.insert_local_state(env);

        for child in self.get_proxied_children() {
            child.process_keyboard_event(event, env, global_state);
        }

        self.update_local_widget_state(env)
    }
}

pub trait NoEvents {}

impl<S, T> Event<S> for T where T: NoEvents + StateSync<S> {
    fn handle_mouse_event(&mut self, _event: &MouseEvent, _consumed: &bool, _global_state: &mut S) {}

    fn handle_keyboard_event(&mut self, _event: &KeyboardEvent, _global_state: &mut S) {}

    fn handle_other_event(&mut self, _event: &WidgetEvent) {}
}