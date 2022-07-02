use std::{fs, io, process, time::Duration};

use bevy::prelude::*;
use bevy_prototype_lyon::{draw, prelude::*};
use bevy_rapier2d::prelude::*;
use image::{
    imageops::{self, FilterType},
    DynamicImage, GenericImageView, ImageBuffer, ImageOutputFormat, Rgb, RgbImage,
};
use rand::Rng;
use v4l::{io::traits::CaptureStream, prelude::MmapStream};
use yuv::convert::RGBConvert;

use crate::GridSize;

#[derive(Component)]
pub struct Particle {
    pub x: u32,
    pub y: u32,
    // size: f32, // number between 0 and 1
}

pub fn draw_particles(
    mut particles: Query<(&mut Path, &mut DrawMode, &mut Collider, &Particle)>,
    grid_size: Res<GridSize>,
    // img: Res<DynamicImage>,
    keyboard_input: Res<Input<KeyCode>>,
    mut stream: ResMut<MmapStream<'static>>,
    windows: Res<Windows>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    let (buf, meta) = stream.as_mut().next().unwrap();
    let pixel_count = (640 * 480) / 8;

    // 307200
    // 76800

    let y_start = 0;
    let y_end = 307200;

    let u_start = y_end;
    let u_end = u_start + 76800;

    let v_start = u_end;
    let v_end = v_start + 76800;

    let y_buf = &buf[y_start..=y_end];
    let u_buf = &buf[u_start..=u_end];
    let v_buf = &buf[v_start..=v_end];

    // println!("{:?}", rgb_from_yuv(149.0, 43.0, 21.0));

    let img = ImageBuffer::from_fn(640, 480, |x, y| {
        let index = x + y * 640;
        let y_col = y_buf[index as usize];

        // let index_2 = index % 480; // index as f32 / 4;
        // let index_2 = (index 2 4) % (640 * 2); // first line is perfect
        // let index_2 = (index 2 4) % (640 * 2);
        // let index_2 = (x as f32 / 2.0 + y as f32 * 640.0).floor();
        // let index_2 = ((x as f32 * 2.0) + (y as f32 * 2.0)) / (640.0 * 2.0); // No scan lines
        // let index_2 = ((x as f32 * 2.0) + (y as f32 * 2.0)) / (640.0 * 2.0); // No scan lines
        // let index_2 = index as f32 % 640.0;
        // let index_2 = index as f32 % 640.0 / 2.0; // good columns
        // let index_2 = (((index as f32 / 2.0) % (640.0)) * y as f32); // latest attempt but untested while ari used the toilet
        let mut index_2 = index as f32 / 2.0 % 640.0;
        // index_2 += (y as f32 / 2.0) * 640.0;
        // if (index % 640 * 2 == 0) {
        //     index_2 = index_2 + 640;
        // }

        // let row_index = (index_2 / 640.0).floor();

        index_2 = index_2 + ((y as f32 / 2.0).floor() * 640.0 / 2.0);

        if index_2 > 76800.0 {
            return Rgb([0, 0, 0]);
        }
        // if x % 640 == 0 {
        //     println!("{}", (y as f32 / 2.0).floor());
        // }

        let u_col = u_buf[index_2 as usize];
        let v_col = v_buf[index_2 as usize];
        // println!("{}", index_2 as usize);
        // std::thread::sleep(Duration::from_millis(50));
        // let mut p = String::new();
        // io::stdin().read_line(&mut p);

        let (r, g, b) = rgb_from_yuv(y_col as f32, u_col as f32, v_col as f32);
        Rgb([r, g, b])

        // println!(
        //     "x: {x}, y: {y} = {:?}",
        //     rgb_from_yuv(y_col as f32, u_col as f32, v_col as f32)
        // );
    });
    // img.save("./img.png");
    // todo!();
    let img = imageops::resize(&img, grid_size.x, grid_size.y, FilterType::Gaussian);

    // for y in 0..480 {
    //     for x in 0..640 {
    //         let index = x + y * 640;
    //         let y_col = y_buf[index];
    //         let u_col = u_buf[(index as f32 / 2.0) as usize];
    //         let v_col = v_buf[(index as f32 / 2.0) as usize];

    //         println!(
    //             "x: {x}, y: {y} = {:?}",
    //             rgb_from_yuv(y_col as f32, u_col as f32, v_col as f32)
    //         );
    //     }
    // }

    // let yyy = y[0];
    // let uuu =
    // // let uuu = (u[0] + u[1]) as f32 / 2.0;
    // println!("{uuu}");
    // // let uuu = u8::from_be_bytes((&u[..1]).try_into().unwrap());
    // // let vvv = u8::from_be_bytes((&v[..1]).try_into().unwrap());
    // // println!("{yyy} {uuu} {vvv}");

    // println!("{:?}", &y[..8]);
    // println!("{:?}", &u[..2]);
    // println!("{:?}", &v[..2]);

    // let y_s = y
    //     .iter()
    //     .map(|num| {
    //         if *num < 64 {
    //             " "
    //         } else if *num < 128 {
    //             "#"
    //         } else {
    //             "@"
    //         }
    //     })
    //     .collect::<Vec<_>>()
    //     .join("");
    // fs::write("./y_s.txt", y_s);
    // let u_s = u
    //     .iter()
    //     .map(|num| {
    //         if *num < 64 {
    //             " "
    //         } else if *num < 128 {
    //             "#"
    //         } else {
    //             "@"
    //         }
    //     })
    //     .collect::<Vec<_>>()
    //     .join("");
    // fs::write("./u_s.txt", u_s);
    // let v_s = v
    //     .iter()
    //     .map(|num| {
    //         if *num < 64 {
    //             " "
    //         } else if *num < 128 {
    //             "#"
    //         } else {
    //             "@"
    //         }
    //     })
    //     .collect::<Vec<_>>()
    //     .join("");
    // fs::write("./v_s.txt", v_s);
    // let rgb_convert = RGBConvert::new(yuv::color::Range::Full, MatrixCoefficients::Identity).unwrap();
    // rgb_convert.to_rgb(px)
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

fn rgb_from_yuv(mut y: f32, mut u: f32, mut v: f32) -> (u8, u8, u8) {
    y -= 16.0;
    u -= 128.0;
    v -= 128.0;
    let r = 1.164 * y + 1.596 * v;
    let g = 1.164 * y - 0.392 * u - 0.813 * v;
    let b = 1.164 * y + 2.017 * u;
    (r as u8, g as u8, b as u8)
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
        velocity.linvel.x = 100_000_000_000.0;
    });
}
