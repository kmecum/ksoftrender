extern crate image;
extern crate ndarray;
extern crate obj;

use std::cmp;
use std::path::Path;

mod graphics;

mod vec;
use vec::{Vec2i, Vec3f};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const DEPTH: u32 = 255;

const LIGHT_DIR: Vec3f = Vec3f {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};
const EYE: Vec3f = Vec3f {
    x: 1.0,
    y: 1.0,
    z: 3.0,
};
const CENTER: Vec3f = Vec3f {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const UP: Vec3f = Vec3f {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

fn main() {
    let mut imgbuf = image::ImageBuffer::from_pixel(WIDTH, HEIGHT, image::Rgba([0, 0, 0, 255]));
    let texture = image::open("./diablo3_pose_diffuse.tga").unwrap();
    let texture = image::imageops::flip_vertical(&texture);

    let input = obj::Obj::<obj::SimplePolygon>::load(Path::new(r"./diablo3_pose.obj")).unwrap();

    let mut zbuffer = Vec::<f32>::with_capacity((WIDTH * HEIGHT) as usize);
    zbuffer.resize((WIDTH * HEIGHT) as usize, std::f32::NEG_INFINITY);

    let projection = graphics::projection(-1.0 / (EYE - CENTER).magnitude());
    let viewport = graphics::viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3 / 4, HEIGHT * 3 / 4, DEPTH);
    let modelview = graphics::lookat(EYE, CENTER, UP);

    for object in input.objects {
        for group in object.groups {
            for poly in group.polys {
                let mut world_coordinates = [Vec3f::from_points(0.0, 0.0, 0.0); 3];
                let mut screen_coordinates = [Vec3f::from_points(0.0, 0.0, 0.0); 3];
                let mut colors = [Vec3f::from_points(0.0, 0.0, 0.0); 3];

                for i in 0..3 {
                    let v = Vec3f::from_array(input.position[poly[i].0]);

                    world_coordinates[i] = v;
                    screen_coordinates[i] =
                        project(&(viewport.dot(&projection).dot(&modelview).dot(&embed(&v))));
                    match poly[i].1 {
                        Some(c) => {
                            colors[i] = Vec3f::from_array2(input.texture[c]);
                        }
                        _ => (),
                    }
                }

                triangle(&screen_coordinates, &colors, &mut zbuffer, |x, y, uv| {
                    let uv = uv.scale(&Vec3f::from_points(
                        texture.dimensions().0 as f32,
                        texture.dimensions().1 as f32,
                        1.0,
                    ));
                    let color = texture.get_pixel(uv.x as u32, uv.y as u32);

                    plot(&mut imgbuf, x, y, color);
                });
            }
        }
    }

    let imgbuf = image::imageops::flip_vertical(&imgbuf);

    imgbuf.save("output.png").unwrap();
}

fn plot(
    imgbuf: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    x: i32,
    y: i32,
    color: &image::Rgba<u8>,
) {
    let x = x as u32;
    let y = y as u32;

    if x < imgbuf.width() && y < imgbuf.height() {
        imgbuf.put_pixel(x, y, *color);
    }
}

fn barycentric(points: &[Vec3f; 3], p: Vec2i) -> Vec3f {
    let a = Vec3f::from_points(
        points[2].x - points[0].x,
        points[1].x - points[0].x,
        points[0].x - p.x as f32,
    );
    let b = Vec3f::from_points(
        points[2].y - points[0].y,
        points[1].y - points[0].y,
        points[0].y - p.y as f32,
    );

    let u = a.cross(b);
    if u.z.abs() < 1.0 {
        Vec3f::from_points(-1.0, 1.0, 1.0)
    } else {
        Vec3f::from_points(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }
}

fn triangle<F>(points: &[Vec3f; 3], colors: &[Vec3f; 3], zbuffer: &mut [f32], mut plot: F)
where
    F: FnMut(i32, i32, Vec3f),
{
    let mut bboxmin = Vec2i::from_points(std::i32::MAX, std::i32::MAX);
    let mut bboxmax = Vec2i::from_points(0, 0);

    for point in points {
        bboxmin.x = cmp::max(0, cmp::min(bboxmin.x, point.x as i32));
        bboxmin.y = cmp::max(0, cmp::min(bboxmin.y, point.y as i32));
        bboxmax.x = cmp::max(bboxmax.x, point.x as i32) + 1;
        bboxmax.y = cmp::max(bboxmax.y, point.y as i32) + 1;
    }

    for x in bboxmin.x..bboxmax.x {
        for y in bboxmin.y..bboxmax.y {
            let b = barycentric(points, Vec2i::from_points(x, y));
            if b.x >= 0.0 && b.y >= 0.0 && b.z >= 0.0 {
                let z = points[0].z * b.x + points[1].z * b.y + points[2].z * b.z;
                if zbuffer[(x + y * WIDTH as i32) as usize] < z {
                    zbuffer[(x + y * WIDTH as i32) as usize] = z;

                    let r = Vec3f::from_points(colors[0].x, colors[1].x, colors[2].x) * b;
                    let g = Vec3f::from_points(colors[0].y, colors[1].y, colors[2].y) * b;
                    let b = Vec3f::from_points(colors[0].z, colors[1].z, colors[2].z) * b;

                    plot(x, y, Vec3f::from_points(r, g, b));
                }
            }
        }
    }
}

fn project(m: &ndarray::Array2<f32>) -> Vec3f {
    Vec3f {
        x: m[(0, 0)] / m[(3, 0)],
        y: m[(1, 0)] / m[(3, 0)],
        z: m[(2, 0)] / m[(3, 0)],
    }
}

fn embed(v: &Vec3f) -> ndarray::Array2<f32> {
    ndarray::array![[v.x], [v.y], [v.z], [1.0]]
}
