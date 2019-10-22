extern crate ndarray;

use super::vec::Vec3f;

pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> ndarray::Array2<f32> {
    let z = (eye - center).normalize();
    let x = up.cross(z).normalize();
    let y = z.cross(x).normalize();

    let mut mv = identity(4);
    for i in 0..3 {
        mv[(0, i)] = x[i];
        mv[(1, i)] = y[i];
        mv[(2, i)] = z[i];
        mv[(i, 3)] = -center[i];
    }

    mv
}

pub fn projection(coefficient: f32) -> ndarray::Array2<f32> {
    let mut p = identity(4);
    p[(3, 2)] = coefficient;

    p
}

pub fn viewport(x: u32, y: u32, w: u32, h: u32, d: u32) -> ndarray::Array2<f32> {
    let x = x as f32;
    let y = y as f32;
    let w = w as f32;
    let h = h as f32;
    let d = d as f32;

    let mut v = identity(4);
    v[(0, 3)] = x + w / 2.0;
    v[(1, 3)] = y + h / 2.0;
    v[(2, 3)] = d / 2.0;

    v[(0, 0)] = w / 2.0;
    v[(1, 1)] = h / 2.0;
    v[(2, 2)] = d / 2.0;

    v
}

fn identity(n: usize) -> ndarray::Array2<f32> {
    ndarray::Array::from_shape_fn((n, n), |d| if d.0 == d.1 { 1.0 } else { 0.0 })
}
