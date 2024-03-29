use bevy::prelude::{In, NextState, ResMut, States, };

use crate::selector::condition::{once, ReactorSystemConfigs, with, WithInput};


/// Once set a next state.
///
/// ```
/// use bevy::app::AppExit;
/// use bevy::prelude::*;
/// use bevy_flurx::prelude::*;
///
/// #[derive(States, Copy, Clone, Hash, Eq, PartialEq, Default, Debug)]
/// enum S{
///     #[default]
///     S1,
///     S2
/// };
///
/// let mut app = App::new();
/// app.add_plugins(FlurxPlugin);
/// app.init_state::<S>();
/// app.add_systems(Startup, |world: &mut World|{
///     world.schedule_reactor(|task| async move {
///         task.will(First, once::state::set(S::S2)).await;
///     });
/// });
/// app.update();
/// assert_eq!(*app.world.resource_ref::<State<S>>().get(), S::S2);
/// ```
pub fn set<S>(state: S) -> impl ReactorSystemConfigs<WithInput, In=S>
    where S: States + 'static
{
    with(state, once::run(|input: In<S>, mut state: ResMut<NextState<S>>| {
        state.set(input.0);
    }))
}



