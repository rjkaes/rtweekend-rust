use crate::*;

use std::rc::Rc;

const APERTURE: f32 = 0.0;
const SAMPLES_PER_PIXEL: i32 = 100;

type World = Vec<HittableInstance>;

pub struct Scene {
    pub world: World,
    pub background: Vec3,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vfov: f32,
    pub aperture: f32,
    pub samples_per_pixel: i32,
}

// Static test scene used for profiling.
#[allow(dead_code)]
pub fn test() -> Scene {
    let mut world: World = vec![];

    let ground = Rc::new(Lambertian::new(color(0.8, 0.8, 0.0)));
    let center = Rc::new(Lambertian::new(color(0.1, 0.2, 0.5)));
    let left = Rc::new(Dielectric::new(1.5));
    let right = Rc::new(Metal::new(color(0.8, 0.6, 0.2), 0.0));

    world.push(Rc::new(Sphere::new(
        point3(0.0, -100.5, -1.0),
        100.0,
        ground,
    )));
    world.push(Rc::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5, center)));
    world.push(Rc::new(Sphere::new(
        point3(-1.0, 0.0, -1.0),
        0.5,
        left.clone(),
    )));
    world.push(Rc::new(Sphere::new(point3(-1.0, 0.0, -1.0), -0.45, left)));
    world.push(Rc::new(Sphere::new(point3(1.0, 0.0, -1.0), 0.5, right)));

    Scene {
        world,
        background: color(0.7, 0.8, 1.0),
        lookfrom: point3(13.0, 2.0, 3.0),
        lookat: point3(0.0, 0.0, 0.0),
        vfov: 20.0,
        aperture: 0.1,
        samples_per_pixel: SAMPLES_PER_PIXEL,
    }
}

#[allow(dead_code)]
pub fn two_spheres() -> Scene {
    let mut world: World = vec![];

    let checker = Rc::new(CheckerTexture::from_color(
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ));
    let lambertian = Rc::new(Lambertian::new_from_texture(checker));

    world.push(Rc::new(Sphere::new(
        point3(0.0, -10.0, 0.0),
        10.0,
        lambertian.clone(),
    )));
    world.push(Rc::new(Sphere::new(
        point3(0.0, 10.0, 0.0),
        10.0,
        lambertian,
    )));

    Scene {
        world,
        background: color(0.7, 0.8, 1.0),
        lookfrom: point3(13.0, 2.0, 3.0),
        lookat: point3(0.0, 0.0, 0.0),
        vfov: 20.0,
        aperture: APERTURE,
        samples_per_pixel: SAMPLES_PER_PIXEL,
    }
}

#[allow(dead_code)]
pub fn two_perlin_spheres() -> Scene {
    let mut world: World = vec![];

    let pertext = Rc::new(NoiseTexture::new(4.0));
    let lambertian = Rc::new(Lambertian::new_from_texture(pertext));

    world.push(Rc::new(Sphere::new(
        point3(0.0, -1000.0, 0.0),
        1000.0,
        lambertian.clone(),
    )));
    world.push(Rc::new(Sphere::new(
        point3(0.0, 2.0, 0.0),
        2.0,
        lambertian.clone(),
    )));

    Scene {
        world,
        background: color(0.7, 0.8, 1.0),
        lookfrom: point3(13.0, 2.0, 3.0),
        lookat: point3(0.0, 0.0, 0.0),
        vfov: 20.0,
        aperture: APERTURE,
        samples_per_pixel: SAMPLES_PER_PIXEL,
    }
}

#[allow(dead_code)]
pub fn random() -> Scene {
    let mut world: World = vec![];

    let checker = Rc::new(CheckerTexture::from_color(
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ));

    let ground_material = Rc::new(Lambertian::new_from_texture(checker));
    world.push(Rc::new(Sphere::new(
        point3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let shift = point3(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = super::random();
            let center = point3(
                a as f32 + 0.9 * super::random(),
                0.2,
                b as f32 + 0.9 * super::random(),
            );

            if (center - shift).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Rc::new(Lambertian::new(albedo));
                    let center2 = center + vec3(0.0, random_range(0.0, 0.5), 0.0);
                    world.push(Rc::new(MovingSphere::new(
                        center, center2, 0.0, 1.0, 0.2, material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.push(Rc::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Rc::new(Dielectric::new(1.5));
                    world.push(Rc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.push(Rc::new(Sphere::new(point3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(color(0.4, 0.2, 0.1)));
    world.push(Rc::new(Sphere::new(point3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(color(0.7, 0.6, 0.5), 0.0));
    world.push(Rc::new(Sphere::new(point3(4.0, 1.0, 0.0), 1.0, material3)));

    Scene {
        world,
        background: color(0.7, 0.8, 1.0),
        lookfrom: point3(13.0, 2.0, 3.0),
        lookat: point3(0.0, 0.0, 0.0),
        vfov: 20.0,
        aperture: 0.1,
        samples_per_pixel: 500,
    }
}

#[allow(dead_code)]
pub fn earth() -> Scene {
    let mut world: World = vec![];

    let texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let surface = Rc::new(Lambertian::new_from_texture(texture));
    world.push(Rc::new(Sphere::new(point3(0.0, 0.0, 0.0), 2.0, surface)));

    Scene {
        world,
        background: color(0.7, 0.8, 1.0),
        lookfrom: point3(13.0, 2.0, 3.0),
        lookat: point3(0.0, 0.0, 0.0),
        vfov: 20.0,
        aperture: APERTURE,
        samples_per_pixel: SAMPLES_PER_PIXEL,
    }
}

#[allow(dead_code)]
pub fn simple_light() -> Scene {
    let mut world: World = vec![];

    let pertext = Rc::new(NoiseTexture::new(4.0));
    world.push(Rc::new(Sphere::new(
        point3(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_from_texture(pertext.clone())),
    )));
    world.push(Rc::new(Sphere::new(
        point3(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new_from_texture(pertext.clone())),
    )));

    let difflight = Rc::new(DiffuseLight::new(color(4.0, 4.0, 4.0)));
    world.push(Rc::new(rect::XY::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    Scene {
        world,
        samples_per_pixel: 400,
        background: color(0.0, 0.0, 0.0),
        lookfrom: point3(26.0, 3.0, 6.0),
        lookat: point3(0.0, 2.0, 0.0),
        vfov: 20.0,
        aperture: APERTURE,
    }
}

pub fn cornell_box() -> Scene {
    let mut world: World = vec![];

    let red = Rc::new(Lambertian::new(color(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(color(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(color(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(color(15.0, 15.0, 15.0)));

    world.push(Rc::new(rect::YZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.push(Rc::new(rect::YZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.push(Rc::new(rect::XZ::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.push(Rc::new(rect::XZ::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.push(Rc::new(rect::XZ::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.push(Rc::new(rect::XY::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let cube1 = Cube::new(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 330.0, 165.0),
        white.clone(),
    );
    let cube1_rotated = RotateY::new(Rc::new(cube1), 15.0);
    let cube1_translated = Translate::new(Rc::new(cube1_rotated), vec3(265.0, 0.0, 295.0));
    world.push(Rc::new(cube1_translated));

    let cube2 = Cube::new(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 165.0, 165.0),
        white.clone(),
    );
    let cube2_rotated = RotateY::new(Rc::new(cube2), -18.0);
    let cube2_translated = Translate::new(Rc::new(cube2_rotated), vec3(130.0, 0.0, 65.0));
    world.push(Rc::new(cube2_translated));

    Scene {
        world,
        samples_per_pixel: 200,
        background: color(0.0, 0.0, 0.0),
        lookfrom: point3(278.0, 278.0, -800.0),
        lookat: point3(278.0, 278.0, 0.0),
        vfov: 40.0,
        aperture: APERTURE,
    }
}

pub fn cornell_smoke() -> Scene {
    let mut world: World = vec![];

    let red = Rc::new(Lambertian::new(color(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(color(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(color(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(color(7.0, 7.0, 7.0)));

    world.push(Rc::new(rect::YZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.push(Rc::new(rect::YZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.push(Rc::new(rect::XZ::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    world.push(Rc::new(rect::XZ::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.push(Rc::new(rect::XZ::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.push(Rc::new(rect::XY::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let cube1 = Cube::new(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 330.0, 165.0),
        white.clone(),
    );
    let cube1_rotated = RotateY::new(Rc::new(cube1), 15.0);
    let cube1_translated = Translate::new(Rc::new(cube1_rotated), vec3(265.0, 0.0, 295.0));
    let cube1_smoke =
        ConstantMedium::with_color(Rc::new(cube1_translated), 0.01, color(0.0, 0.0, 0.0));
    world.push(Rc::new(cube1_smoke));

    let cube2 = Cube::new(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 165.0, 165.0),
        white.clone(),
    );
    let cube2_rotated = RotateY::new(Rc::new(cube2), -18.0);
    let cube2_translated = Translate::new(Rc::new(cube2_rotated), vec3(130.0, 0.0, 65.0));
    let cube2_smoke =
        ConstantMedium::with_color(Rc::new(cube2_translated), 0.01, color(1.0, 1.0, 1.0));
    world.push(Rc::new(cube2_smoke));

    Scene {
        world,
        samples_per_pixel: 200,
        background: color(0.0, 0.0, 0.0),
        lookfrom: point3(278.0, 278.0, -800.0),
        lookat: point3(278.0, 278.0, 0.0),
        vfov: 40.0,
        aperture: APERTURE,
    }
}

pub fn final_scene_the_next_week() -> Scene {
    let mut world: World = vec![];

    let ground = Rc::new(Lambertian::new(color(0.48, 0.83, 0.53)));

    const BOXES_PER_SIDE: usize = 20;
    let mut boxes1: Vec<HittableInstance> = Vec::with_capacity(BOXES_PER_SIDE * BOXES_PER_SIDE);

    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = super::random_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(Rc::new(Cube::new(
                point3(x0, y0, z0),
                point3(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    world.push(Rc::new(BVHNode::new(boxes1.as_slice(), 0.0, 1.0)));

    let light = Rc::new(DiffuseLight::new(color(7.0, 7.0, 7.0)));
    world.push(Rc::new(rect::XZ::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = point3(400.0, 400.0, 200.0);
    let center2 = center1 + vec3(30.0, 0.0, 0.0);
    let moving_sphere_material = Rc::new(Lambertian::new(color(0.7, 0.3, 0.1)));
    world.push(Rc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    world.push(Rc::new(Sphere::new(
        point3(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.push(Rc::new(Sphere::new(
        point3(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(color(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary1 = Rc::new(Sphere::new(
        point3(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.push(boundary1.clone());
    world.push(Rc::new(ConstantMedium::with_color(
        boundary1.clone(),
        0.2,
        color(0.2, 0.4, 0.9),
    )));
    let boundary2 = Rc::new(Sphere::new(
        point3(0.0, 0.0, 0.0),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.push(Rc::new(ConstantMedium::with_color(
        boundary2.clone(),
        0.0001,
        color(1.0, 1.0, 1.0),
    )));

    let emat = Rc::new(Lambertian::new_from_texture(Rc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.push(Rc::new(Sphere::new(
        point3(400.0, 200.0, 400.0),
        100.0,
        emat.clone(),
    )));
    let pertext = Rc::new(NoiseTexture::new(0.1));
    world.push(Rc::new(Sphere::new(
        point3(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::new_from_texture(pertext)),
    )));

    let mut boxes2: Vec<HittableInstance> = Vec::with_capacity(1000);
    let white = Rc::new(Lambertian::new(color(0.73, 0.73, 0.73)));

    for _ in 0..1000 {
        boxes2.push(Rc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.push(Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(BVHNode::new(boxes2.as_slice(), 0.0, 1.0)),
            15.0,
        )),
        vec3(-100.0, 270.0, 395.0),
    )));

    Scene {
        world,
        samples_per_pixel: 10_000,
        background: color(0.0, 0.0, 0.0),
        lookfrom: point3(478.0, 278.0, -600.0),
        lookat: point3(278.0, 278.0, 0.0),
        vfov: 40.0,
        aperture: APERTURE,
    }
}
