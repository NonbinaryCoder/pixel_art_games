use std::iter;

use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::render_resource::{Extent3d, PrimitiveTopology, TextureDimension, TextureFormat},
    sprite::Mesh2dHandle,
};

use crate::art::{Pixel, PixelColor};

pub struct MeshGenerationPlugin;

impl Plugin for MeshGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_system);
    }
}

fn startup_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(MulticolorMeshMaterial(materials.add(ColorMaterial::from(
        images.add(Image::new(
            Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            vec![255, 255, 255, 255],
            TextureFormat::Rgba8Unorm,
        )),
    ))));
}

#[derive(Debug, Resource)]
pub struct MulticolorMeshMaterial(Handle<ColorMaterial>);

#[derive(Debug, Component)]
pub struct MulticolorMesh {
    mesh: Mesh2dHandle,
}

impl MulticolorMesh {
    pub fn generate<'w, 's, 'a, 'm>(
        commands: &'a mut Commands<'w, 's>,
        material: &MulticolorMeshMaterial,
        meshes: &'m mut Assets<Mesh>,
    ) -> (EntityCommands<'w, 's, 'a>, MulticolorMeshEditor<'m>) {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, Vec::<[f32; 4]>::new());
        let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

        let editor = MulticolorMeshEditor::new(meshes.get_mut(&mesh_handle.0).unwrap());
        (
            commands.spawn((
                Self {
                    mesh: mesh_handle.clone(),
                },
                ColorMesh2dBundle {
                    mesh: mesh_handle,
                    material: material.0.clone(),
                    ..default()
                },
            )),
            editor,
        )
    }

    pub fn edit<'a>(&self, meshes: &'a mut Assets<Mesh>) -> MulticolorMeshEditor<'a> {
        MulticolorMeshEditor::new(meshes.get_mut(&self.mesh.0).unwrap())
    }
}

#[derive(Debug)]
pub struct MulticolorMeshEditor<'a> {
    positions: &'a mut Vec<[f32; 3]>,
    colors: &'a mut Vec<[f32; 4]>,
}

impl<'a> MulticolorMeshEditor<'a> {
    fn new(mesh: &'a mut Mesh) -> Self {
        let mut iter = mesh.attributes_mut();
        let positions = iter
            .find(|&(id, _)| id == Mesh::ATTRIBUTE_POSITION.id)
            .unwrap();
        let colors = iter
            .find(|&(id, _)| id == Mesh::ATTRIBUTE_COLOR.id)
            .unwrap();
        use bevy::render::mesh::VertexAttributeValues as Vav;
        if let (Vav::Float32x3(positions), Vav::Float32x4(colors)) = (positions.1, colors.1) {
            Self { positions, colors }
        } else {
            panic!()
        }
    }

    pub fn add_pixel(&mut self, pixel: Pixel) -> &mut Self {
        self.add_square(pixel.pos.as_vec2(), 1.0, pixel.color)
    }

    pub fn add_square(&mut self, pos: Vec2, size: f32, color: PixelColor) -> &mut Self {
        self.add_rect(pos, Vec2::splat(size), color)
    }

    pub fn add_rect(&mut self, pos: Vec2, size: Vec2, color: PixelColor) -> &mut Self {
        let extents = size * 0.5;
        self.add_quad(
            [
                pos + extents,
                Vec2::new(pos.x - extents.x, pos.y + extents.y),
                pos - extents,
                Vec2::new(pos.x + extents.x, pos.y - extents.y),
            ],
            color,
        )
    }

    pub fn add_quad(&mut self, positions: [Vec2; 4], color: PixelColor) -> &mut Self {
        let positions = positions.map(|pos| [pos.x, pos.y, 0.0]);
        self.positions.extend([
            positions[2],
            positions[1],
            positions[0],
            positions[0],
            positions[3],
            positions[2],
        ]);
        self.colors
            .extend(iter::repeat(<[f32; 4]>::from(color)).take(6));
        self
    }
}
