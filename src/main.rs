use bevy::{prelude::*, window::WindowResized};
//Window Stuff
/// Marker component for the text that displays the current resolution.
#[derive(Component)]
struct ResolutionText;

/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

// Spawns the camera that draws UI
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Spawns the UI
fn setup_ui(mut commands: Commands) {
    // Node that fills entire background
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            ..default()
        })
        // Text where we display current resolution
        .with_child((
            Text::new("Resolution"),
            TextFont {
                font_size: 42.0,
                ..default()
            },
            ResolutionText,
        ));
}

/// This system shows how to request the window to a new resolution
fn toggle_resolution(
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        let res = resolution.small;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
    }
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut text: Single<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        // When resolution is being changed
        text.0 = format!("{:.1} x {:.1}", e.width, e.height);
    }
}

// Close window on escape
fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

const HOMEBOARD_TEXTURE: &str = "Images/Boards/Homeboard_Long.jpg";
const ACTIONBOARD_TEXTURE: &str = "Images/Boards/Action_Board.jpg";

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&mut Window>,
    assets: Res<Assets<Image>>,
) {
    let homeboard_texture = asset_server.load(HOMEBOARD_TEXTURE);
    // Get window Dimensions
    // Make the HomeBoard fit the window
    // let window_width = window.width();
    let window_height = window.height();

    // Get homeboard dimensions

    commands.spawn(Sprite {
        custom_size: Some(Vec2::new(homeboard_width, window_height)),
        image: homeboard_texture,
        ..default()
    });
    let texture = asset_server.load(ACTIONBOARD_TEXTURE);
    commands.spawn(Sprite {
        //I want the x axis to remain the same as the image
        custom_size: Some(Vec2::new(100.0, window_height)),
        image: texture,
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_ui, setup))
        .add_systems(Update, (on_resize_system, toggle_resolution, close_on_esc))
        // .add_systems(Update, update)
        .run();
}
