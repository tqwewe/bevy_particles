mod particle;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use image::{imageops::FilterType, io::Reader as ImageReader, DynamicImage, GenericImageView};
use v4l::prelude::*;

use crate::particle::{draw_particles, make_particles_dynamic, reset_particle_position, Particle};

#[derive(Debug, Deref, DerefMut)]
pub struct GridSize(UVec2);

fn main() {
    let grid_size = GridSize(UVec2::new(112, 60)); // 300, 160

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
        // .add_system(enable_ccd)
        .add_system(reset_particle_position)
        .add_system(make_particles_dynamic)
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

fn setup_system(
    mut commands: Commands,
    grid_size: Res<GridSize>,
    img: Res<DynamicImage>,
    windows: Res<Windows>,
) {
    let primary_window = windows.primary();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // let mut brightest_pixel = 0;
    // for y in 0..grid_size.y {
    //     for x in 0..grid_size.x {
    //         let pixel = img.get_pixel(x, grid_size.y - y - 1);
    //         let brightness = (pixel.0[0] + pixel.0[1] + pixel.0[2]);
    //         brightest_pixel = brightness.max(brightest_pixel);
    //     }
    // }

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
        .insert(Collider::cuboid(primary_window.width(), 20.0));

    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(
                ((primary_window.width() / 2.0) + 20.0) * -1.0,
                0.0,
                0.0,
            )),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(20.0, primary_window.height()));

    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new((primary_window.width() / 2.0) + 20.0, 0.0, 0.0)),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(20.0, primary_window.height()));

    // Draw grid
    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            let pixel = img.get_pixel(x, grid_size.y - y - 1);
            // let brightness = (pixel.0[0] + pixel.0[1] + pixel.0[2]) as f32 / brightest_pixel as f32;

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
                DrawMode::Fill(bevy_prototype_lyon::draw::FillMode::color(Color::rgb_u8(
                    pixel.0[0], pixel.0[1], pixel.0[2],
                ))),
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
