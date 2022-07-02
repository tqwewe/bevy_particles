#![allow(clippy::type_complexity)]

mod particle;

use bevy::{prelude::*, window::WindowResized};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use image::{imageops::FilterType, io::Reader as ImageReader};
use particle::apply_random_velocity;

use crate::particle::{draw_particles, make_particles_dynamic, reset_particle_position, Particle};

#[derive(Debug, Deref, DerefMut)]
pub struct GridSize(UVec2);

fn main() {
    let grid_size = GridSize(UVec2::new(80, 60)); // 300, 160

    let img = ImageReader::open("cat.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .resize_exact(grid_size.x, grid_size.y, FilterType::Gaussian);

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
        .insert_resource(grid_size)
        .insert_resource(img)
        .insert_resource(WindowDescriptor {
            width: 640.,
            height: 480.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        // .add_system(enable_ccd)
        .add_system(place_wall_resize)
        .add_system(reset_particle_position)
        .add_system(make_particles_dynamic)
        .add_system(apply_random_velocity.after(make_particles_dynamic))
        .add_system(draw_particles.after(reset_particle_position))
        .run();
}

// fn enable_ccd(
//     mut rigid_bodies: ResMut<RigidBodySet>,
//     new_handles: Query<&RigidBodyHandle, Added<RigidBodyHandle>>,
// ) {
//     for handle in new_handles.iter() {
//         if let Some(body) = rigid_bodies.get_mut(handle.into_rapier()) {
//             body.enable_ccd(true);
//         }
//     }
// }

#[derive(Component)]
struct LeftWall;

#[derive(Component)]
struct RightWall;

#[derive(Component)]
struct Floor;

fn setup_system(mut commands: Commands, grid_size: Res<GridSize>, windows: Res<Windows>) {
    let primary_window = windows.primary();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Ground and walls
    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(
                0.0,
                ((primary_window.height() / 2.0) + 20.0) * -1.0,
                0.0,
            )),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(primary_window.width(), 20.0))
        .insert(Floor);

    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(
                ((primary_window.width() / 2.0) + 20.0) * -1.0,
                0.0,
                0.0,
            )),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(20.0, primary_window.height()))
        .insert(LeftWall);

    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new((primary_window.width() / 2.0) + 20.0, 0.0, 0.0)),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(20.0, primary_window.height()))
        .insert(RightWall);

    // Draw grid
    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            let pos_x = x as f32 * primary_window.width() / grid_size.x as f32
                - primary_window.width() / 2.0
                + primary_window.width() / grid_size.x as f32 / 2.0;
            let pos_y = y as f32 * primary_window.height() / grid_size.y as f32
                - primary_window.height() / 2.0
                + primary_window.height() / grid_size.y as f32 / 2.0;

            let shape = shapes::Ellipse {
                radii: Vec2::new(
                    primary_window.width() / grid_size.x as f32 / 2.0 * 0.2,
                    primary_window.height() / grid_size.y as f32 / 2.0 * 0.2,
                ),
                ..default()
            };
            let mut bundle = GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(bevy_prototype_lyon::draw::FillMode::color(Color::WHITE)),
                Transform::default(),
            );
            bundle.transform.translation.x = pos_x;
            bundle.transform.translation.y = pos_y;
            commands
                .spawn_bundle(bundle)
                .insert(Particle { x, y })
                .insert(RigidBody::Fixed)
                .insert(Collider::ball(
                    primary_window.width() / grid_size.x as f32 / 2.0 * 0.2,
                ))
                .insert(Velocity::default());
        }
    }
}

fn place_wall_resize(
    mut query: ParamSet<(
        Query<(&mut Transform, &mut Collider), With<Floor>>,
        Query<(&mut Transform, &mut Collider), With<LeftWall>>,
        Query<(&mut Transform, &mut Collider), With<RightWall>>,
        Query<(&mut Transform, &Particle)>,
    )>,
    mut resize_event: EventReader<WindowResized>,
    grid_size: Res<GridSize>,
    windows: Res<Windows>,
) {
    if resize_event.iter().next().is_some() {
        let primary_window = windows.primary();

        query.p0().for_each_mut(|(mut transform, mut collider)| {
            transform.translation.y = ((primary_window.height() / 2.0) + 20.0) * -1.0;
            *collider = Collider::cuboid(primary_window.width(), 20.0);
        });

        query.p1().for_each_mut(|(mut transform, mut collider)| {
            transform.translation.x = ((primary_window.width() / 2.0) + 20.0) * -1.0;
            *collider = Collider::cuboid(20.0, primary_window.height());
        });

        query.p2().for_each_mut(|(mut transform, mut collider)| {
            transform.translation.x = (primary_window.width() / 2.0) + 20.0;
            *collider = Collider::cuboid(20.0, primary_window.height());
        });

        query.p3().for_each_mut(|(mut transform, particle)| {
            let pos_x = particle.x as f32 * primary_window.width() / grid_size.x as f32
                - primary_window.width() / 2.0
                + primary_window.width() / grid_size.x as f32 / 2.0;
            let pos_y = particle.y as f32 * primary_window.height() / grid_size.y as f32
                - primary_window.height() / 2.0
                + primary_window.height() / grid_size.y as f32 / 2.0;

            transform.translation.x = pos_x;
            transform.translation.y = pos_y;
        });
    }
}
