use crate::AspectCamera;
use bevy::prelude::*;

pub struct BorderAspectRatioPlugin<T>
where
    T: Component + Copy + Clone,
{
    pub color: Color,
    pub marker: T,
}

#[derive(Resource)]
struct BorderColor(Color);

#[derive(Resource)]
struct BorderMarker<T>(T)
where
    T: Component + Copy + Clone;

#[derive(Component)]
pub struct VisibleArea;

impl<T: Component + Copy + Clone> Plugin for BorderAspectRatioPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(BorderColor(self.color))
            .insert_resource(BorderMarker(self.marker))
            .add_startup_system(setup::<T>)
            .add_system(update_borders);
    }
}

fn setup<T>(mut commands: Commands, marker: Res<BorderMarker<T>>, color: Res<BorderColor>)
where
    T: Component + Copy + Clone,
{
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            },
            marker.0,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Percent(100.)),
                        flex_grow: 1.,
                        ..Default::default()
                    },
                    background_color: color.0.into(),
                    ..Default::default()
                },
                marker.0,
            ));

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Percent(100.)),
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    marker.0,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Auto),
                                flex_grow: 1.,
                                ..Default::default()
                            },
                            background_color: color.0.into(),
                            ..Default::default()
                        },
                        marker.0,
                    ));

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        },
                        VisibleArea,
                        marker.0,
                    ));

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Auto),
                                flex_grow: 1.,
                                ..Default::default()
                            },
                            background_color: color.0.into(),
                            ..Default::default()
                        },
                        marker.0,
                    ));
                });

            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Percent(100.)),
                        flex_grow: 1.,
                        ..Default::default()
                    },
                    background_color: color.0.into(),
                    ..Default::default()
                },
                marker.0,
            ));
        });
}

fn update_borders(
    mut visible_area: Query<&mut Style, With<VisibleArea>>,
    camera: Query<(&OrthographicProjection, &AspectCamera), Changed<OrthographicProjection>>,
) {
    if let Ok((orthographic_projection, aspect_camera)) = camera.get_single() {
        if let Ok(mut style) = visible_area.get_single_mut() {
            style.size = Size::new(
                Val::Px(aspect_camera.width / orthographic_projection.scale),
                Val::Px(aspect_camera.height / orthographic_projection.scale),
            )
        }
    }
}
