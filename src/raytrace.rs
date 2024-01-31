use crate::color::{Canvas, Color};
use crate::smoothstep;
use cgmath::InnerSpace;
use noise::{NoiseFn, OpenSimplex};

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;

// TODO: implement this
trait Scene: Raytrace {
    fn sun_dir(&self) -> Vec3;
}

#[derive(Debug)]
pub enum TraceResult {
    Continue {
        emit: Color,
        transmit: Color,
    },
    Hit {
        emit: Color,
        transmit: Color,
        normal: Vec3,
    },
}

pub trait Raytrace {
    fn trace_step(&self, ray: Vec3, pos: &mut Vec3) -> TraceResult;
    fn signed_dist(&self, at: Vec3) -> f32;

    // Calculate normal vector to surface at a point as gradient of sdf of the surface
    fn normal(&self, at: Vec3) -> Vec3 {
        const DELTA: f32 = 0.001;
        let x_step = Vec3::unit_x() * DELTA;
        let y_step = Vec3::unit_y() * DELTA;
        let z_step = Vec3::unit_z() * DELTA;

        let dist_here = self.signed_dist(at);
        let xn = self.signed_dist(at + x_step) - dist_here;
        let yn = self.signed_dist(at + y_step) - dist_here;
        let zn = self.signed_dist(at + z_step) - dist_here;
        Vec3::new(xn, yn, zn).normalize()
    }
}

pub struct Moon {
    radius: f32,
    position: Vec3,
    color: Color,
    surface: OpenSimplex,
}
impl Moon {
    pub fn new(color: Color, radius: f32, position: Vec3, seed: i32) -> Moon {
        Moon {
            color,
            radius,
            position,
            surface: OpenSimplex::new(seed as u32),
        }
    }
}
impl Raytrace for Moon {
    fn trace_step(&self, ray: Vec3, pos: &mut Vec3) -> TraceResult {
        let distance = self.signed_dist(*pos);
        let new_pos = *pos + ray * distance;
        *pos = new_pos;

        if distance <= 0.01 {
            let noise = self.surface.get([pos.x as f64, pos.y as f64, pos.z as f64]);
            return TraceResult::Hit {
                emit: self.color
                    + Color {
                        r: 0.0,
                        g: 0.0,
                        b: noise as f32,
                    },
                transmit: Color::white(),
                normal: self.normal(new_pos),
            };
        }

        let gas_diffuse = Color::white() / (distance + 200.0);
        return TraceResult::Continue {
            emit: gas_diffuse,
            transmit: Color::white() - gas_diffuse,
        };
    }

    fn signed_dist(&self, from: Vec3) -> f32 {
        const BUMP_HEIGHT: f32 = 2.0;
        const BUMP_FREQ: f32 = 1.0;
        let dist = (from - self.position).magnitude() - self.radius;
        if dist > BUMP_HEIGHT {
            return dist;
        }
        let noise = self.surface.get_vec(from * BUMP_FREQ);
        return dist + noise * smoothstep(BUMP_HEIGHT - dist) * BUMP_HEIGHT;
    }
}

trait NoiseFnEx {
    fn get_vec(&self, vec: Vec3) -> f32;
}

impl<T: NoiseFn<f64, 3>> NoiseFnEx for T {
    fn get_vec(&self, vec: Vec3) -> f32 {
        self.get([vec.x as f64, vec.y as f64, vec.z as f64]) as f32
    }
}
