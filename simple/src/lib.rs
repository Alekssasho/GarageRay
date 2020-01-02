mod camera;
mod hitable;
mod material;
mod math;
mod random;
mod ray;
mod texture;

#[macro_use]
extern crate lazy_static;

pub extern crate image;

pub use camera::*;
pub use hitable::*;
use material::*;
pub use math::*;
use random::random_float;
use ray::*;
use texture::*;

use rand::distributions::Distribution;

#[cfg(feature="parallel")]
use rayon::prelude::*;

fn color(ray: &Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {
        let emitted = rec.material.unwrap().emitted(rec.u, rec.v, &rec.p);
        if depth < 50 {
            match rec.material.unwrap().scatter(ray, &rec) {
                Some((attenuation, scattered)) => {
                    emitted + color(&scattered, world, depth + 1).mul_element_wise(attenuation)
                }
                _ => emitted,
            }
        } else {
            emitted
        }
    } else {
        Vec3::zero()
    }
}

#[allow(dead_code)]
fn random_scene() -> Vec<Box<dyn Hitable>> {
    let n = 500;
    let mut list: Vec<Box<dyn Hitable>> = Vec::with_capacity(n + 1);
    list.push(Box::new(Sphere {
        center: vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Box::new(CheckerTexture {
                even: Box::new(ConstantTexture(vec3(0.2, 0.3, 0.1))),
                odd: Box::new(ConstantTexture(vec3(0.9, 0.9, 0.9))),
            }),
        }),
    }));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = vec3(
                a as f32 + 0.9 * random_float(),
                0.2,
                b as f32 + 0.9 * random_float(),
            );
            if (center - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => list.push(Box::new(MovingSphere {
                        center0: center,
                        center1: center + vec3(0.0, 0.5 * random_float(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Box::new(ConstantTexture(vec3(
                                random_float() * random_float(),
                                random_float() * random_float(),
                                random_float() * random_float(),
                            ))),
                        }),
                    })),
                    x if x < 0.95 => list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal {
                            albedo: vec3(
                                0.5 * (1.0 + random_float()),
                                0.5 * (1.0 + random_float()),
                                0.5 * (1.0 + random_float()),
                            ),
                            fuzz: 0.5 * random_float(),
                        }),
                    })),
                    _ => list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric { ref_index: 1.5 }),
                    })),
                }
            }
        }
    }

    list.push(Box::new(Sphere {
        center: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric { ref_index: 1.5 }),
    }));
    list.push(Box::new(Sphere {
        center: vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Box::new(ConstantTexture(vec3(0.4, 0.2, 0.1))),
        }),
    }));
    list.push(Box::new(Sphere {
        center: vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal {
            albedo: vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));
    list
}

#[allow(dead_code)]
fn two_perlin_spheres() -> Vec<Box<dyn Hitable>> {
    let img = image::open("untitled.png").unwrap();
    let image_texture = Box::new(ImageTexture::new(img));
    let noise = Box::new(NoiseTexture { scale: 4.0 });
    let list: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: vec3(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Box::new(Lambertian { albedo: noise }),
        }),
        Box::new(Sphere {
            center: vec3(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Box::new(Lambertian {
                albedo: image_texture,
            }),
        }),
    ];
    list
}

#[allow(dead_code)]
fn simple_light() -> Vec<Box<dyn Hitable>> {
    let noise = Box::new(NoiseTexture { scale: 4.0 });
    let list: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: vec3(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Box::new(Lambertian {
                albedo: noise.clone(),
            }),
        }),
        Box::new(Sphere {
            center: vec3(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Box::new(Lambertian { albedo: noise }),
        }),
        Box::new(Sphere {
            center: vec3(0.0, 7.0, 0.0),
            radius: 2.0,
            material: Box::new(DiffuseLight {
                emit: Box::new(ConstantTexture(vec3(4.0, 4.0, 4.0))),
            }),
        }),
        Box::new(XYRect {
            x0: 3.0,
            x1: 5.0,
            y0: 1.0,
            y1: 3.0,
            k: -2.0,
            material: Box::new(DiffuseLight {
                emit: Box::new(ConstantTexture(vec3(4.0, 4.0, 4.0))),
            }),
        }),
    ];
    list
}

#[allow(dead_code)]
fn cornel_box() -> Vec<Box<dyn Hitable>> {
    let red = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture(vec3(0.65, 0.05, 0.05))),
    });
    let white = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture(vec3(0.73, 0.73, 0.73))),
    });
    let green = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture(vec3(0.12, 0.45, 0.15))),
    });
    let light = Box::new(DiffuseLight {
        emit: Box::new(ConstantTexture(vec3(15.0, 15.0, 15.0))),
    });
    let list: Vec<Box<dyn Hitable>> = vec![
        Box::new(FlipNormals(Box::new(YZRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            material: green,
        }))),
        Box::new(YZRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            material: red,
        }),
        Box::new(XZRect {
            x0: 213.0,
            x1: 343.0,
            z0: 227.0,
            z1: 332.0,
            k: 554.0,
            material: light,
        }),
        Box::new(FlipNormals(Box::new(XZRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            material: white.clone(),
        }))),
        Box::new(XZRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            material: white.clone(),
        }),
        Box::new(FlipNormals(Box::new(XYRect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            material: white.clone(),
        }))),
        Box::new(Translate {
            offset: vec3(130.0, 0.0, 65.0),
            hitable: Box::new(RotateY::new(
                Box::new(BoxHitable::new(
                    &vec3(0.0, 0.0, 0.0),
                    &vec3(165.0, 165.0, 165.0),
                    white.clone(),
                )),
                -18.0,
            )),
        }),
        Box::new(Translate {
            offset: vec3(265.0, 0.0, 295.0),
            hitable: Box::new(RotateY::new(
                Box::new(BoxHitable::new(
                    &vec3(0.0, 0.0, 0.0),
                    &vec3(165.0, 330.0, 165.0),
                    white.clone(),
                )),
                15.0,
            )),
        }),
    ];
    list
}

#[allow(dead_code)]
fn cornel_smoke() -> Vec<Box<dyn Hitable>> {
    let red = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture(vec3(0.65, 0.05, 0.05))),
    });
    let white = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture(vec3(0.73, 0.73, 0.73))),
    });
    let green = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture(vec3(0.12, 0.45, 0.15))),
    });
    let light = Box::new(DiffuseLight {
        emit: Box::new(ConstantTexture(vec3(7.0, 7.0, 7.0))),
    });

    let box1 = Box::new(Translate {
        offset: vec3(130.0, 0.0, 65.0),
        hitable: Box::new(RotateY::new(
            Box::new(BoxHitable::new(
                &vec3(0.0, 0.0, 0.0),
                &vec3(165.0, 165.0, 165.0),
                white.clone(),
            )),
            -18.0,
        )),
    });
    let box2 = Box::new(Translate {
        offset: vec3(265.0, 0.0, 295.0),
        hitable: Box::new(RotateY::new(
            Box::new(BoxHitable::new(
                &vec3(0.0, 0.0, 0.0),
                &vec3(165.0, 330.0, 165.0),
                white.clone(),
            )),
            15.0,
        )),
    });

    let list: Vec<Box<dyn Hitable>> = vec![
        Box::new(FlipNormals(Box::new(YZRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            material: green,
        }))),
        Box::new(YZRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            material: red,
        }),
        Box::new(XZRect {
            x0: 113.0,
            x1: 443.0,
            z0: 127.0,
            z1: 432.0,
            k: 554.0,
            material: light,
        }),
        Box::new(FlipNormals(Box::new(XZRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            material: white.clone(),
        }))),
        Box::new(XZRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            material: white.clone(),
        }),
        Box::new(FlipNormals(Box::new(XYRect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            material: white.clone(),
        }))),
        Box::new(ConstantMedium::new(
            box1,
            0.01,
            Box::new(ConstantTexture(vec3(1.0, 1.0, 1.0))),
        )),
        Box::new(ConstantMedium::new(
            box2,
            0.01,
            Box::new(ConstantTexture(vec3(0.0, 0.0, 0.0))),
        )),
    ];
    list
}

pub fn final_scene() -> Vec<Box<dyn Hitable>> {
    let white = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture(vec3(0.73, 0.73, 0.73))),
    });
    let ground = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture(vec3(0.48, 0.83, 0.53))),
    });

    let mut boxlist: Vec<Box<dyn Hitable>> = vec![];
    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (random_float() + 0.01);
            let z1 = z0 + w;
            boxlist.push(Box::new(BoxHitable::new(
                &vec3(x0, y0, z0),
                &vec3(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let center = vec3(400.0, 400.0, 200.0);
    let boundary = Sphere {
        center: vec3(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Box::new(Dielectric { ref_index: 1.5 }),
    };
    let boundary2 = Sphere {
        center: vec3(0.0, 0.0, 0.0),
        radius: 5000.0,
        material: Box::new(Dielectric { ref_index: 1.5 }),
    };
    let img = image::open("earthmap.jpg").unwrap();
    let image_texture = Box::new(ImageTexture::new(img));
    let emat = Box::new(Lambertian {
        albedo: image_texture,
    });
    let mut boxlist2: Vec<Box<dyn Hitable>> = vec![];
    for _j in 0..1000 {
        boxlist2.push(Box::new(Sphere {
            center: vec3(
                165.0 * random_float(),
                165.0 * random_float(),
                165.0 * random_float(),
            ),
            radius: 10.0,
            material: white.clone(),
        }))
    }

    let list: Vec<Box<dyn Hitable>> = vec![
        Box::new(BVHNode::build(boxlist, 0.0, 1.0)),
        Box::new(XZRect {
            x0: 123.0,
            x1: 423.0,
            z0: 147.0,
            z1: 412.0,
            k: 554.0,
            material: Box::new(DiffuseLight {
                emit: Box::new(ConstantTexture(vec3(7.0, 7.0, 7.0))),
            }),
        }),
        Box::new(MovingSphere {
            center0: center,
            center1: center + vec3(30.0, 0.0, 0.0),
            radius: 50.0,
            time0: 0.0,
            time1: 1.0,
            material: Box::new(Lambertian {
                albedo: Box::new(ConstantTexture(vec3(0.7, 0.3, 0.1))),
            }),
        }),
        Box::new(Sphere {
            center: vec3(260.0, 150.0, 45.0),
            radius: 50.0,
            material: Box::new(Dielectric { ref_index: 1.5 }),
        }),
        Box::new(Sphere {
            center: vec3(0.0, 150.0, 145.0),
            radius: 50.0,
            material: Box::new(Metal {
                albedo: vec3(0.8, 0.8, 0.9),
                fuzz: 10.0,
            }),
        }),
        Box::new(boundary.clone()),
        Box::new(ConstantMedium::new(
            Box::new(boundary),
            0.2,
            Box::new(ConstantTexture(vec3(0.2, 0.4, 0.9))),
        )),
        Box::new(ConstantMedium::new(
            Box::new(boundary2),
            0.0001,
            Box::new(ConstantTexture(vec3(1.0, 1.0, 1.0))),
        )),
        Box::new(Sphere {
            center: vec3(400.0, 200.0, 400.0),
            radius: 100.0,
            material: emat,
        }),
        Box::new(Sphere {
            center: vec3(220.0, 280.0, 300.0),
            radius: 80.0,
            material: Box::new(Lambertian {
                albedo: Box::new(NoiseTexture { scale: 0.1 }),
            }),
        }),
        Box::new(Translate {
            offset: vec3(-100.0, 270.0, 395.0),
            hitable: Box::new(RotateY::new(
                Box::new(BVHNode::build(boxlist2, 0.0, 1.0)),
                15.0,
            )),
        }),
    ];
    list
}

pub fn evaluate_pixel(x: u32, y: u32, width: u32, height: u32, samples: i32, world: &dyn Hitable, camera: &Camera) -> (u8, u8, u8) {
    #[cfg(feature="parallel")]
    let samples_vector: Vec<i32> = (0..samples).collect();
    #[cfg(feature="parallel")]
    let samples_itr = samples_vector.par_iter();

    #[cfg(not(feature="parallel"))]
    let samples_itr = 0..samples;

    let mut color: Vec3 = samples_itr
        .map(|_| {
            let mut rng = rand::thread_rng();
            let uniform_distribution = rand::distributions::Uniform::new(0.0, 1.0);
            let u = (x as f32 + uniform_distribution.sample(&mut rng)) / width as f32;
            let v = ((height - y - 1) as f32 + uniform_distribution.sample(&mut rng))
                / height as f32;
            let ray = camera.get_ray(u, v);
            color(&ray, world, 0)
            })
            .sum::<Vec3>()
            / samples as f32;

        if color.x > 1.0 {
            color.x = 1.0;
        }
        if color.y > 1.0 {
            color.y = 1.0;
        }
        if color.z > 1.0 {
            color.z = 1.0;
        }
        let ir = (255.99 * color.x.sqrt()) as u8;
        let ig = (255.99 * color.y.sqrt()) as u8;
        let ib = (255.99 * color.z.sqrt()) as u8;
        (ir, ig, ib)
}

