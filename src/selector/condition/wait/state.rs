use bevy::prelude::{Res, State, States, };

use crate::selector::condition::{ReactorSystemConfigs, wait, with};

/// Waits until the state becomes the specified.
///
/// ```
/// use bevy::app::AppExit;
/// use bevy::prelude::*;
/// use bevy_flurx::prelude::*;
///
/// #[derive(Debug, Default, States, Copy, Clone, Hash, Eq, PartialEq)]
/// enum Status{
///     #[default]
///     Running,
///     Finish
/// }
///
/// let mut app = App::new();
/// app.init_state::<Status>();
/// app.add_plugins(FlurxPlugin);
/// app.add_systems(Startup, |world: &mut World|{
///     world.schedule_reactor(|task|async move{
///         let wait_state = task.run(Update, wait::state::becomes(Status::Finish)).await;
///         task.will(Update, once::state::set(Status::Finish)).await;
///         wait_state.await;
///         task.will(Update, once::non_send::init::<AppExit>()).await;
///     });
/// });
/// app.update();
/// app.update();
/// app.update();
/// app.update();
/// assert!(app.world.get_non_send_resource::<AppExit>().is_some());
/// ```
#[inline]
pub fn becomes<S>(state: S) -> impl ReactorSystemConfigs<In=(), Out=()>
    where S: States + 'static
{
    with((), wait::until(move |state_now: Res<State<S>>| {
        state_now.as_ref() == &state
    }))
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, AppExit, First, Startup, Update};
    use bevy::prelude::{States, World};

    use crate::extension::ScheduleReactor;
    use crate::FlurxPlugin;
    use crate::selector::condition::{once, wait};

    #[derive(States, Eq, PartialEq, Default, Copy, Clone, Hash, Debug)]
    enum TestState {
        #[default]
        Phase1,
        Phase2,
    }

    #[test]
    fn wait_until_state_becomes_phase2() {
        let mut app = App::new();
        app
            .add_plugins(FlurxPlugin)
            .init_state::<TestState>()
            .add_systems(Startup, |world: &mut World| {
                world.schedule_reactor(|task| async move {
                    task.will(First, wait::state::becomes(TestState::Phase2)).await;
                    task.will(Update, once::non_send::init::<AppExit>()).await;
                });
            });
        app.update();
        assert!(app.world.get_non_send_resource::<AppExit>().is_none());
        app.insert_state(TestState::Phase2);
        app.update();
        app.update();
        assert!(app.world.get_non_send_resource::<AppExit>().is_some());
    }
}