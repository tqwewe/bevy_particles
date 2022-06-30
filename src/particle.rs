use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use rand::Rng;
use v4l::{io::traits::CaptureStream, prelude::MmapStream};

use crate::GridSize;

#[derive(Component)]
pub struct Particle {
    pub x: u32,
    pub y: u32,
    // size: f32, // number between 0 and 1
}

pub fn draw_particles(
    mut particles: Query<(&mut Path, &mut CollisionShape, &Particle)>,
    grid_size: Res<GridSize>,
    img: Res<DynamicImage>,
    keyboard_input: Res<Input<KeyCode>>,
    // stream: ResMut<MmapStream<'static>>,
    windows: Res<Windows>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    // let (buf, meta) = stream.as_mut().next().unwrap();
    // let img = ImageBuffer::from_raw(640, 480, buf).unwrap();

    let primary_window = windows.primary();
    let win_w = primary_window.width();
    let win_h = primary_window.height();

    let mut brightest_pixel = 0;
    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            let pixel = img.get_pixel(x, grid_size.y - y - 1);
            let brightness = pixel.0[0] + pixel.0[1] + pixel.0[2];
            brightest_pixel = brightness.max(brightest_pixel);
        }
    }

    particles.for_each_mut(|(mut path, mut collision_shape, particle)| {
        let pixel = img.get_pixel(particle.x, grid_size.y - particle.y - 1);
        let brightness = (pixel.0[0] + pixel.0[1] + pixel.0[2]) as f32 / (brightest_pixel as f32);
        let radii = f32::min(
            win_w / grid_size.x as f32 / 2.0 * brightness,
            win_h / grid_size.y as f32 / 2.0 * brightness,
        )
        .max(0.5);
        let shape = shapes::Ellipse {
            radii: Vec2::new(radii, radii),
            ..default()
        };
        *path = ShapePath::build_as(&shape);
        *collision_shape = CollisionShape::Sphere { radius: radii };
    });
}

pub fn reset_particle_position(
    mut particles: Query<(&mut Transform, &mut Velocity, &mut RigidBody, &Particle)>,
    grid_size: Res<GridSize>,
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
) {
    if !keyboard_input.just_released(KeyCode::Space) {
        return;
    }

    let primary_window = windows.primary();
    let win_w = primary_window.width();
    let win_h = primary_window.height();

    particles.for_each_mut(|(mut transform, mut velocity, mut rigid_body, particle)| {
        let pos_x = particle.x as f32 * win_w / grid_size.x as f32 - win_w / 2.0
            + win_w / grid_size.x as f32 / 2.0;
        let pos_y = particle.y as f32 * win_h / grid_size.y as f32 - win_h / 2.0
            + win_h / grid_size.y as f32 / 2.0;

        transform.translation.x = pos_x;
        transform.translation.y = pos_y;

        *velocity = Velocity::default();
        *rigid_body = RigidBody::Static;
    });
}

pub fn make_particles_dynamic(
    mut particles: Query<(&mut RigidBody, &mut Velocity), With<Particle>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let mut rng = rand::thread_rng();

    particles.for_each_mut(|(mut rigid_body, mut velocity)| {
        *rigid_body = RigidBody::Dynamic;
        *velocity = Velocity::from_linear(Vec3::new(rng.gen(), rng.gen(), rng.gen()));
    });
}
