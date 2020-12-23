use crate::state::environment::Environment;
use crate::widget::common_widget::CommonWidget;

pub trait StateSync<S>: CommonWidget<S> {
    /// Insert local state from the widget into the environment.
    /// Return true if any of the keys from the widget was already
    /// in the local state.
    fn insert_local_state(&self, env: &mut Environment);

    /// Update the state for this widget. Update both local, global and environment state
    fn update_all_widget_state(&mut self, env: &Environment, global_state: &S);

    /// Update only the local state for the widget
    fn update_local_widget_state(&mut self, env: &Environment);

    /*/// When implementing this, all states that are a function of globalState needs to be updated
    /// This is done by calling either get_value or get_value_mut.
    /// Todo: Update this to happen automatically
    /// You also need to update all the local states, with the values from the states list.
    fn update_widget_state(&mut self, env: &mut Environment, global_state: &S);
    */

    /// This should be implemented to synchronize both global and local state.
    /// A general implementation should:
    /// - Update the widget state, both global and local
    /// - Insert its own local state into the environment
    /// - Iterate though its children and sync_state on each
    fn sync_state(&mut self, env: &mut Environment, global_state: &S) {
        self.update_all_widget_state(env, global_state);

        self.insert_local_state(env);

        for child in self.get_children_mut() {
            child.sync_state(env, global_state)
        }

        self.update_local_widget_state(env);
    }
}

pub trait NoLocalStateSync {}

impl<S, T> StateSync<S> for T where T: NoLocalStateSync + CommonWidget<S> {
    fn insert_local_state(&self, _: &mut Environment) {}

    fn update_all_widget_state(&mut self, _: &Environment, _: &S) {}

    fn update_local_widget_state(&mut self, _env: &Environment) {}
}