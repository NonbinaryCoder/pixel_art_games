use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
    render::{
        camera::{camera_system, CameraProjection, CameraRenderGraph},
        primitives::Frustum,
        view::VisibleEntities,
    },
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_system).add_system_to_stage(
            CoreStage::PostUpdate,
            camera_system::<AreaTrackingProjection>,
        );
    }
}

#[derive(Component)]
pub struct AreaTrackingProjection {
    near: f32,
    far: f32,
    pub tracked_area: Rect,
    screen_aspect: f32,
}

fn startup_system(mut commands: Commands) {
    let projection = AreaTrackingProjection::default();
    let transform = Transform::from_xyz(0.0, 0.0, projection.far - 0.1);
    let view_projection = projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = Frustum::from_view_projection(
        &view_projection,
        &transform.translation,
        &transform.back(),
        projection.far,
    );

    commands.spawn((
        CameraRenderGraph::new("core_2d"),
        projection,
        VisibleEntities::default(),
        frustum,
        transform,
        GlobalTransform::default(),
        Camera::default(),
        Camera2d::default(),
        Tonemapping::Disabled,
    ));
}

impl CameraProjection for AreaTrackingProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        let tracked_aspect = aspect(self.tracked_area.size());
        if tracked_aspect > self.screen_aspect {
            // Collides with top/bottom of screen
            let midpoint = self.tracked_area.center().x;
            let half_width = self.tracked_area.size().y * (1.0 / self.screen_aspect) * 0.5;
            Mat4::orthographic_rh(
                midpoint - half_width,
                midpoint + half_width,
                self.tracked_area.max.y,
                self.tracked_area.min.y,
                self.near,
                self.far,
            )
        } else {
            // Collides with left/right of screen
            let midpoint = self.tracked_area.center().y;
            let half_height = self.tracked_area.size().x * (self.screen_aspect) * 0.5;
            Mat4::orthographic_rh(
                self.tracked_area.min.x,
                self.tracked_area.max.x,
                midpoint + half_height,
                midpoint - half_height,
                self.near,
                self.far,
            )
        }
    }

    fn update(&mut self, width: f32, height: f32) {
        self.screen_aspect = height / width;
    }

    fn far(&self) -> f32 {
        self.far
    }
}

impl Default for AreaTrackingProjection {
    fn default() -> Self {
        AreaTrackingProjection {
            near: 0.0,
            far: 1000.0,
            tracked_area: Rect::new(0.0, 0.0, 0.0, 0.0),
            screen_aspect: 1.0,
        }
    }
}

fn aspect(size: Vec2) -> f32 {
    size.y / size.x
}
