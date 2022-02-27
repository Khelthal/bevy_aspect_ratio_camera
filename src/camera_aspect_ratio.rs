use bevy::{prelude::*, window::WindowResized};

pub struct CameraAspectRatioPlugin;

#[derive(Component)]
pub struct AspectCamera {
    pub width: f32,
    pub height: f32,
}

impl Plugin for CameraAspectRatioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keep_aspect);
    }
}


fn keep_aspect(
    mut window_resized: EventReader<WindowResized>,
    mut camera: Query<(&mut OrthographicProjection, &AspectCamera)>,
) {
    for window_size in window_resized.iter() {
        if let Ok((mut orthographic_projection, aspect_camera)) = camera.get_single_mut() {
            orthographic_projection.scale = aspect_camera.width / window_size.width;

            let desired_height = window_size.width * (aspect_camera.height / aspect_camera.width);

            if desired_height > window_size.height {
                orthographic_projection.scale = aspect_camera.height / window_size.height;
            }
        }
    }
}
