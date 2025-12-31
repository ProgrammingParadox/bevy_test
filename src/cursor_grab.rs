use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

pub fn cursor_grab(mut cursor_options: Single<&mut CursorOptions>) {
    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    cursor_options.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    cursor_options.visible = false;
}

pub fn toggle_cursor(
    mut cursor_options: Single<&mut CursorOptions>,
    input: Res<ButtonInput<KeyCode>>,
    // buttons: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        cursor_options.visible = !cursor_options.visible;
        cursor_options.grab_mode = match cursor_options.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked => CursorGrabMode::None,

            _ => panic!("unhandled CursorGrabMode!"),
        };
    }
    // if buttons.pressed(MouseButton::Left) {
    //     cursor_options.visible = false;
    //     cursor_options.grab_mode = CursorGrabMode::Locked;
    // }
}

#[cfg(target_os = "windows")]
pub fn cursor_recenter(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    cursor_options: Single<&mut CursorOptions>,
) {
    if cursor_options.grab_mode == CursorGrabMode::None {
        return;
    }

    let mut primary_window = q_windows.single_mut().expect("No primary window found");
    let center = Vec2::new(primary_window.width() / 2.0, primary_window.height() / 2.0);
    primary_window.set_cursor_position(Some(center));
}

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cursor_grab)
            .add_systems(Update, toggle_cursor);

        #[cfg(target_os = "windows")]
        app.add_systems(Update, cursor_recenter);
    }
}
