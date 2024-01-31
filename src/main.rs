use cgmath::{InnerSpace, Zero};
use color::{Canvas, Color};
use raytrace::{Moon, Raytrace, TraceResult};

mod cli;
mod color;
mod raytrace;
// mod texture;

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;

fn main() {
    let options = cli::Options::from_args();
    println!("{options:?}");

    let fragmets = rasterize(&options);
    let uniform = ShaderUniform {
        camera_pos: Vec3::new(0.0, 0.0, 10.0),
        camera_dir: Vec3::new(0.0, 0.0, -1.0),
        sun_dir: Vec3::new(0.5, 0.2, 0.0),
    };

    let moon = Moon::new(Color::new(0.5, 0.0, 0.0), 6.0, Vec3::zero(), 6554);

    let mut canvas = Canvas::new(options.width(), options.height());
    for fragment in &fragmets {
        let color_part = shader_raytrace_object(fragment.shader_input, &uniform, &moon);
        // let color_part = shader_circle(fragment.shader_input, &uniform, 0.5);
        let pixel = canvas.get_mut(fragment.pixel_x as _, fragment.pixel_y as _);
        *pixel += color_part;
    }

    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            let pixel = canvas.get_mut(x, y);
            let c = 1.0 / options.samples() as f32;
            *pixel *= c;
        }
    }

    print!("{}", canvas.print_ansi_rgb_halfblock());
}

struct RasterizerOutput {
    shader_input: ShaderInput,
    pixel_x: i32,
    pixel_y: i32,
}

fn rasterize(options: &cli::Options) -> Vec<RasterizerOutput> {
    let width = options.width() as i32;
    let height = options.height() as i32;

    let samples: Vec<Vec2> = {
        let samp = |x: i32, y: i32| Vec2::new(x as f32 / 16.0, y as f32 / 16.0);
        match options.samples() {
            1 => vec![samp(0, 0)],
            2 => vec![samp(-4, -4), samp(4, 4)],
            4 => vec![samp(-2, -6), samp(6, -2), samp(-6, 2), samp(2, 6)],
            8 => vec![
                samp(-7, 1),
                samp(-5, -5),
                samp(-3, 5),
                samp(-1, -3),
                samp(1, 3),
                samp(3, -7),
                samp(5, -1),
                samp(7, 7),
            ],
            _ => panic!("Unsupported multisample pattern."),
        }
    };

    println!("{samples:?}");

    let mut fragmets = Vec::new();
    for x in 0..width {
        for y in 0..height {
            for sample in &samples {
                let xf = (x - width / 2) as f32 + sample.x;
                let yf = (y - height / 2) as f32 + sample.y;

                let shader_input = ShaderInput {
                    x: xf / width as f32 * 2.0,
                    y: -yf / height as f32 * 2.0,
                };

                fragmets.push(RasterizerOutput {
                    shader_input,
                    pixel_x: x,
                    pixel_y: y,
                });
            }
        }
    }

    return fragmets;
}

#[derive(Clone, Copy)]
struct ShaderInput {
    x: f32,
    y: f32,
}

struct ShaderUniform {
    camera_pos: Vec3,
    camera_dir: Vec3,
    sun_dir: Vec3,
}

type Shader = dyn Fn(ShaderInput, &ShaderUniform) -> Color;

fn shader_xy(input: ShaderInput, uniform: &ShaderUniform) -> Color {
    Color {
        r: input.x,
        g: input.y,
        b: 0.0,
    }
}


fn shader_circle(input: ShaderInput, uniform: &ShaderUniform, radius: f32) -> Color {
    let ShaderInput { x, y } = input;

    //println!("ker ({x:.2}, {y:.2})");

    if x * x + y * y <= radius * radius {
        return Color::red();
    }
    return Color::white();
}

fn sdf_sphere(point: Vec3, radius: f32) -> f32 {
    point.magnitude() - radius
}

fn shader_sphere_raytrace(input: ShaderInput, uniform: &ShaderUniform, radius: f32) -> Color {
    let ShaderInput { x, y } = input;
    let ShaderUniform {
        camera_pos,
        camera_dir,
        sun_dir,
    } = uniform;
    let camera_dir = camera_dir.normalize();
    //let sun_dir = sun_dir.normalize();

    let up = Vec3::new(0.0, 1.0, 0.0);
    let right = camera_dir.cross(up).normalize();
    let view_dir = Vec3::normalize(camera_dir + up * y + right * x);
    //println!("view: {view_dir:?}");
    let mut cur_point = camera_pos.clone();
    let mut distance = 0.0;
    let mut hit = false;
    for i in 0..5 {
        distance = sdf_sphere(cur_point, radius);
        //println!("#{i}: ({x:.1},{y:.1}), {distance}");
        cur_point += view_dir * distance;

        if distance > 1000.0 * radius {
            break;
        }
        if distance <= 0.01 {
            hit = true;
            break;
        }
    }
    //return Color::new(distance / 10.0, distance.fract(), 0.0);
    //let normal_vec = Vec3::normalize(cur_point);
    //let light_coef = normal_vec.dot(sun_dir);
    //Color::red() * light_coef

    if hit {
        let normal_vec = Vec3::normalize(cur_point);
        let light_coef = -normal_vec.dot(*sun_dir);
        Color::yellow() * light_coef + Color::new(0.1, 0.1, 0.1)
    } else {
        Color::black()
    }
}

fn shader_raytrace_object<T: Raytrace>(
    input: ShaderInput,
    uniform: &ShaderUniform,
    obj: &T,
) -> Color {
    let ShaderInput { x, y } = input;
    let ShaderUniform {
        camera_pos,
        camera_dir,
        sun_dir,
        ..
    } = uniform;
    let camera_dir = camera_dir.normalize();
    let sun_dir = sun_dir.normalize();

    let up = Vec3::new(0.0, 1.0, 0.0);
    let right = camera_dir.cross(up).normalize();
    let view_dir = Vec3::normalize(camera_dir + (right * x) + (up * y));

    let mut cur_point = camera_pos.clone();
    let mut incoming_light = Color::black();
    let mut accumulated_tone = Color::white();

    for _ in 0..50 {
        let hit = obj.trace_step(view_dir, &mut cur_point);
        //println!("{},{}: {:?}", input.x, input.y, hit);
        match hit {
            TraceResult::Continue { emit, transmit } => {
                incoming_light += emit * accumulated_tone;
                accumulated_tone *= transmit;
            }
            TraceResult::Hit { emit, transmit, normal, .. } => {
                let diffuse = emit * normal.dot(-sun_dir).max(0.05);
                incoming_light += diffuse * accumulated_tone;
                accumulated_tone *= transmit;
                break;
            }
        }
    }

    return incoming_light;
}


pub(crate) fn smoothstep(x: f32) -> f32 {
    let t = x.clamp(0.0, 1.0);
    (3.0 - 2.0 * t) * t * t
}