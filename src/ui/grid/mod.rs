use bevy::math::{Vec3Swizzles, Vec4Swizzles};
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::render::view::VisibleEntities;

mod render;

pub struct InfiniteGridPlugin;

impl Plugin for InfiniteGridPlugin {
    fn build(&self, app: &mut App) {}

    fn finish(&self, app: &mut App) {
        render::render_app_builder(app);
    }
}

#[derive(Component, Default)]
pub struct InfiniteGrid;

#[derive(Component, Copy, Clone)]
pub struct InfiniteGridSettings {
    pub x_axis_color: Color,
    pub y_axis_color: Color,
    pub minor_line_color: Color,
    pub major_line_color: Color,
}

impl Default for InfiniteGridSettings {
    fn default() -> Self {
        Self {
            x_axis_color: Color::rgb(1.0, 0.2, 0.2),
            y_axis_color: Color::rgb(0.2, 0.2, 1.0),
            minor_line_color: Color::rgb(0.1, 0.1, 0.1),
            major_line_color: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct GridFrustumIntersect {
    pub points: [Vec3; 4],
    pub center: Vec3,
    pub up_dir: Vec3,
    pub width: f32,
    pub height: f32,
}

#[derive(Bundle, Default)]
pub struct InfiniteGridBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub settings: InfiniteGridSettings,
    pub grid: InfiniteGrid,
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
    pub inherited_visibility: InheritedVisibility,
    pub shadow_casters: VisibleEntities,
    pub no_frustum_culling: NoFrustumCulling,
}
