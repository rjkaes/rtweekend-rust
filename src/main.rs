use rtweekend::camera::*;
use rtweekend::hittable::*;
use rtweekend::random;
use rtweekend::ray::*;
use rtweekend::scenes;
use rtweekend::vec3::*;

use std::io;

fn main() -> io::Result<()> {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;

    // Image
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const MAX_DEPTH: i32 = 50;

    // let scene = scenes::test();
    // let scene = scenes::random();
    // let scene = scenes::simple_light();
    // let scene = scenes::two_spheres();
    // let scene = scenes::two_perlin_spheres();
    // let scene = scenes::earth();
    let scene = scenes::simple_light();

    // Camera
    const VUP: Vec3 = vec3(0.0, 1.0, 0.0);
    const DIST_TO_FOCUS: f32 = 10.0;

    let camera = Camera::new(
        scene.lookfrom,
        scene.lookat,
        VUP,
        scene.vfov,
        ASPECT_RATIO,
        scene.aperture,
        DIST_TO_FOCUS,
        0.0,
        1.0,
    );

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = color(0.0, 0.0, 0.0);

            for _ in 0..(scene.samples_per_pixel) {
                let u = (i as f32 + random()) / ((IMAGE_WIDTH - 1) as f32);
                let v = (j as f32 + random()) / ((IMAGE_HEIGHT - 1) as f32);

                let r = camera.get_ray(u, v);

                pixel_color += ray_color(r, scene.background, &scene.world, MAX_DEPTH);
            }

            write_color(pixel_color, scene.samples_per_pixel);
        }
    }

    eprintln!("\nDone.");

    Ok(())
}

fn ray_color(r: Ray, background: Color, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(&r, 0.001, f32::INFINITY) {
        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

        if let Some((attenuation, scattered)) = rec.material.scatter(&r, &rec) {
            emitted + attenuation * ray_color(scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        background
    }
}

// FIXME: This is broken now that objects can emit light!
// fn ray_color(initial: Ray, background: Color, world: &dyn Hittable, depth: i32) -> Color {
//     let mut r = initial;
//     let mut c = color(1.0, 1.0, 1.0);

//     for _ in 0..depth {
//         if let Some(rec) = world.hit(&r, 0.001, f32::INFINITY) {
//             let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

//             if let Some((attenuation, scattered)) = rec.material.scatter(&r, &rec) {
//                 c = emitted + attenuation * c;
//                 r = scattered;
//             } else {
//                 return emitted;
//             }
//         } else {
//             return c * background;
//         }
//     }

//     // If we exited the loop, that means we hit nothing, so return black.
//     color(0.0, 0.0, 0.0)
// }

fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / (samples_per_pixel as f32);

    let rs = (r * scale).sqrt();
    let gs = (g * scale).sqrt();
    let bs = (b * scale).sqrt();

    // Scale the 0..1 into 0..255
    // TODO: This is not a perfect scaling
    let ir = (256.0 * f32::clamp(rs, 0.0, 0.999)) as u32;
    let ig = (256.0 * f32::clamp(gs, 0.0, 0.999)) as u32;
    let ib = (256.0 * f32::clamp(bs, 0.0, 0.999)) as u32;

    println!("{} {} {}", ir, ig, ib);
}
