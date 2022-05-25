use rtweekend::*;
use std::io;
use std::rc::Rc;

fn main() -> io::Result<()> {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;

    // Image
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    let mut world;
    let mut lookfrom;
    let mut lookat;
    let mut vfov = 40.0;
    let mut aperture = 0.0;

    // // Random Scene
    // {
    //     world = random_scene();
    //     lookfrom = point3(13.0, 2.0, 3.0);
    //     lookat = point3(0.0, 0.0, 0.0);
    //     vfov = 20.0;
    //     aperture = 0.1;
    // }

    // Two Spheres
    {
        world = two_spheres();
        lookfrom = point3(13.0, 2.0, 3.0);
        lookat = point3(0.0, 0.0, 0.0);
        vfov = 20.0;
    }

    // Camera
    let vup = vec3(0.0, 1.0, 0.0);
    const DIST_TO_FOCUS: f32 = 10.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
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

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random()) / ((IMAGE_WIDTH - 1) as f32);
                let v = (j as f32 + random()) / ((IMAGE_HEIGHT - 1) as f32);

                let r = camera.get_ray(u, v);

                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");

    Ok(())
}

// Static test scene used for profiling.
fn test_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground = Rc::new(Lambertian::new(color(0.8, 0.8, 0.0)));
    let center = Rc::new(Lambertian::new(color(0.1, 0.2, 0.5)));
    let left = Rc::new(Dielectric::new(1.5));
    let right = Rc::new(Metal::new(color(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        point3(0.0, -100.5, -1.0),
        100.0,
        ground,
    )));
    world.add(Box::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5, center)));
    world.add(Box::new(Sphere::new(
        point3(-1.0, 0.0, -1.0),
        0.5,
        left.clone(),
    )));
    world.add(Box::new(Sphere::new(point3(-1.0, 0.0, -1.0), -0.45, left)));
    world.add(Box::new(Sphere::new(point3(1.0, 0.0, -1.0), 0.5, right)));

    world
}

fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::from_color(
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ));
    let lambertian = Rc::new(Lambertian::new_from_texture(checker));

    world.add(Box::new(Sphere::new(
        point3(0.0, -10.0, 0.0),
        10.0,
        lambertian.clone(),
    )));
    world.add(Box::new(Sphere::new(
        point3(0.0, 10.0, 0.0),
        10.0,
        lambertian,
    )));

    world
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::from_color(
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ));

    let ground_material = Rc::new(Lambertian::new_from_texture(checker));
    world.add(Box::new(Sphere::new(
        point3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let shift = point3(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = point3(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - shift).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Rc::new(Lambertian::new(albedo));
                    let center2 = center + vec3(0.0, random_range(0.0, 0.5), 0.0);
                    world.add(Box::new(MovingSphere::new(
                        center, center2, 0.0, 1.0, 0.2, material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(point3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(color(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        point3(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(color(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(point3(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

fn ray_color(initial: Ray, world: &dyn Hittable, depth: i32) -> Color {
    const BLACK: Color = color(0.0, 0.0, 0.0);

    let mut r = initial;
    let mut c = color(1.0, 1.0, 1.0);

    for _ in 0..depth {
        if let Some(rec) = world.hit(&r, 0.001, f32::INFINITY) {
            if let Some((attenuation, scattered)) = rec.material.scatter(&r, &rec) {
                c *= attenuation;
                r = scattered;
            } else {
                return BLACK;
            }
        } else {
            // If the ray didn't hit anything in the world, return the sky.
            let unit_direction = r.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            let sky = (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);

            return c * sky;
        }
    }

    // If we exited the loop, that means we hit nothing, so return black.
    BLACK
}

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
