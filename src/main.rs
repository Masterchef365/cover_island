use anyhow::Result;
use klystron::{
    runtime_3d::{launch, App},
    DrawType, Engine, FramePacket, Object, Vertex, UNLIT_FRAG, UNLIT_VERT, Matrix4
};
use klystron_obj::{self as obj, parse_obj};
use std::fs;

struct MyApp {
    skybox: Object,
    islands: Object,
    water: Object,
    time: f32,
}

const WATER_SCALE: f32 = 0.2;
const WATER_SIZE: i32 = 50;
const WATER_WIDTH: f32 = WATER_SIZE as f32 * WATER_SCALE;
const WATER_INSTANCE_WIDTH: i32 = 5;

impl App for MyApp {
    const NAME: &'static str = "MyApp";

    type Args = ();

    fn new(engine: &mut dyn Engine, _args: Self::Args) -> Result<Self> {
        // Islands
        let obj = parse_obj(&*fs::read("./assets/island.obj")?)?;
        let (vertices, indices) = obj::triangles(&obj, obj::ColorMode::Normal)?;
        let islands_mat = engine.add_material(
            &fs::read("./shaders/island.vert.spv")?, 
            &fs::read("./shaders/island.frag.spv")?, 
            DrawType::Triangles
        )?;
        let islands = Object {
            mesh: engine.add_mesh(&vertices, &indices)?,
            material: islands_mat,
            transform: Matrix4::identity(),
        };

        // Water
        let vertices = dense_grid_verts(WATER_SIZE, WATER_SCALE);
        let indices = dense_grid_tri_indices(WATER_SIZE);
        let water_mat = engine.add_material(
            &fs::read("./shaders/water.vert.spv")?, 
            &fs::read("./shaders/water.frag.spv")?, 
            DrawType::Triangles
        )?;
        let water = Object {
            mesh: engine.add_mesh(&vertices, &indices)?,
            material: water_mat,
            transform: Matrix4::identity(),
        };

        // Skybox
        let skybox_mat = engine.add_material(
            &fs::read("./shaders/skybox.vert.spv")?, 
            &fs::read("./shaders/skybox.frag.spv")?, 
            DrawType::Triangles
        )?;
        let (vertices, indices) = fullscreen_quad();
        let skybox = Object {
            mesh: engine.add_mesh(&vertices, &indices)?,
            material: skybox_mat,
            transform: Matrix4::identity(),
        };

        Ok(Self {
            time: 0.0,
            islands,
            skybox,
            water,
        })
    }

    fn next_frame(&mut self, engine: &mut dyn Engine) -> Result<FramePacket> {
        let mut objects = Vec::new();
        objects.push(self.islands);
        objects.push(self.skybox);
        let i = WATER_INSTANCE_WIDTH;
        for x in -i..i {
            for z in -i..i {
                let x = x as f32 * WATER_WIDTH * 2.;
                let z = z as f32 * WATER_WIDTH * 2.;
                let mut water = self.water;
                water.transform = Matrix4::new_translation(&nalgebra::Vector3::new(x, -1. / 16., z));
                objects.push(water);
            }
        }

        engine.update_time_value(self.time)?;
        self.time += 0.01;
        Ok(FramePacket {
            objects,
        })
    }
}

fn main() -> Result<()> {
    let vr = std::env::args().skip(1).next().is_some();
    launch::<MyApp>(vr, ())
}

/*
fn rainbow_cube() -> (Vec<Vertex>, Vec<u16>) {
    let vertices = vec![
        Vertex::new([-1.0, -1.0, -1.0], [0.0, 1.0, 1.0]),
        Vertex::new([1.0, -1.0, -1.0], [1.0, 0.0, 1.0]),
        Vertex::new([1.0, 1.0, -1.0], [1.0, 1.0, 0.0]),
        Vertex::new([-1.0, 1.0, -1.0], [0.0, 1.0, 1.0]),
        Vertex::new([-1.0, -1.0, 1.0], [1.0, 0.0, 1.0]),
        Vertex::new([1.0, -1.0, 1.0], [1.0, 1.0, 0.0]),
        Vertex::new([1.0, 1.0, 1.0], [0.0, 1.0, 1.0]),
        Vertex::new([-1.0, 1.0, 1.0], [1.0, 0.0, 1.0]),
    ];

    let indices = vec![
        3, 1, 0, 2, 1, 3, 2, 5, 1, 6, 5, 2, 6, 4, 5, 7, 4, 6, 7, 0, 4, 3, 0, 7, 7, 2, 3, 6, 2, 7,
        0, 5, 4, 1, 5, 0,
    ];

    (vertices, indices)
}
*/

fn fullscreen_quad() -> (Vec<Vertex>, Vec<u16>) {
    let vert = |[x, y, z] : [f32; 3]| Vertex::new([x, y, z], [x.signum(), y.signum(), z.signum()]);
    let vertices = vec![
        vert([-1.0, -1.0, 0.0]),
        vert([1.0, -1.0, 0.0]),
        vert([1.0, 1.0, 0.0]),
        vert([-1.0, 1.0, 0.0]),
    ];

    let indices = vec![
        3, 1, 0, 2, 1, 3,
    ];

    (vertices, indices)
}

fn dense_grid_verts(size: i32, scale: f32) -> Vec<Vertex> {
    (-size..=size)
        .map(|x| (-size..=size).map(move |y| (x, y)))
        .flatten()
        .map(|(x, y)| {
            let (x, y) = (x as f32, y as f32);
            let size = size as f32;
            Vertex {
                pos: [x * scale, 0., y * scale],
                color: [x / size, y / size, 0.],
            }
        })
        .collect()
}

fn dense_grid_edge_indices(width: u16) -> impl Iterator<Item = u16> {
    (0..width - 1)
        .map(move |x| (0..width - 1).map(move |y| (x, y)))
        .flatten()
        .map(move |(x, y)| x + y * width)
}

/*
fn dense_grid_wire_indices(size: i32) -> Vec<u16> {
    let width = (size * 2 + 1) as u16;
    let mut indices = Vec::new();
    for base in dense_grid_edge_indices(width) {
        indices.push(base);
        indices.push(base + 1);
        indices.push(base);
        indices.push(base + width);
    }
    indices
}
*/

fn dense_grid_tri_indices(size: i32) -> Vec<u16> {
    let width = (size * 2 + 1) as u16;
    let mut indices = Vec::new();
    for base in dense_grid_edge_indices(width) {
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + width);
        indices.push(base + 1);
        indices.push(base + width + 1);
        indices.push(base + width);
    }
    indices
}

