use bevy::prelude::*;
use bevy_prototype_lyon::{draw, prelude::*};
use bevy_rapier2d::prelude::*;
use image::{DynamicImage, GenericImageView};
use rand::Rng;

use crate::GridSize;

#[derive(Component)]
pub struct Particle {
    pub x: u32,
    pub y: u32,
}

pub fn draw_particles(
    mut particles: Query<(&mut Path, &mut DrawMode, &mut Collider, &Particle)>,
    grid_size: Res<GridSize>,
    img: Res<DynamicImage>,
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        return;
    }

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

    particles.for_each_mut(|(mut path, mut draw_mode, mut collision_shape, particle)| {
        let pixel = img.get_pixel(particle.x, grid_size.y - particle.y - 1);
        let brightness = (pixel.0[0] + pixel.0[1] + pixel.0[2]) as f32 / (brightest_pixel as f32);
        let radii = f32::min(
            win_w / grid_size.x as f32 / 2.0 * brightness,
            win_h / grid_size.y as f32 / 2.0 * brightness,
        )
        .max(5.0)
            * 2.0;
        let shape = shapes::Rectangle {
            extents: Vec2::new(radii, radii),
            ..default()
        };
        *path = ShapePath::build_as(&shape);
        *draw_mode = DrawMode::Fill(draw::FillMode::color(Color::rgb_u8(
            pixel.0[0], pixel.0[1], pixel.0[2],
        )));
        *collision_shape = Collider::cuboid(radii, radii);
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
        *rigid_body = RigidBody::Fixed;
    });
}

pub fn make_particles_dynamic(
    mut particles: Query<&mut RigidBody, With<Particle>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    particles.for_each_mut(|mut rigid_body| {
        *rigid_body = RigidBody::Dynamic;
    });
}

pub fn apply_random_velocity(
    mut particles: Query<&mut Velocity, With<Particle>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let mut rng = rand::thread_rng();

    particles.for_each_mut(|mut velocity| {
        velocity.linvel.x = rng.gen::<f32>() * 10.0;
    });
}
