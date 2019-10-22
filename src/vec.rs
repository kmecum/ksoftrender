use std::ops::Index;
use std::ops::Mul;
use std::ops::Sub;

pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn from_points(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn from_array(v: [f32; 3]) -> Vec3f {
        Vec3f {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }

    pub fn from_array2(v: [f32; 2]) -> Vec3f {
        Vec3f {
            x: v[0],
            y: v[1],
            z: 0.0,
        }
    }

    pub fn from_points(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x, y, z }
    }

    pub fn cross(self, other: Vec3f) -> Vec3f {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Vec3f { x: x, y: y, z: z }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3f {
        let m = self.magnitude();

        Vec3f {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }

    pub fn scale(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Index<usize> for Vec3f {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3f index out of bounds"),
        }
    }
}

impl Mul for Vec3f {
    type Output = f32;

    fn mul(self, other: Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
