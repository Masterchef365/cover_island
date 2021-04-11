use anyhow::Result;
use klystron::{
    runtime_3d::{launch, App},
    DrawType, Engine, FramePacket, Object, Vertex, UNLIT_FRAG, UNLIT_VERT, Matrix4
};

struct MyApp {
    skybox: Object,
    rainbow_cube: Object,
    time: f32,
}

impl App for MyApp {
    const NAME: &'static str = "MyApp";

    type Args = ();

    fn new(engine: &mut dyn Engine, _args: Self::Args) -> Result<Self> {
        let unlit_mat = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Triangles)?;

        // Rainbow cube
        let (vertices, indices) = rainbow_cube();
        let rainbow_cube = Object {
            mesh: engine.add_mesh(&vertices, &indices)?,
            material: unlit_mat,
            transform: Matrix4::identity(),
        };

        // Skybox
        let skybox_mat = engine.add_material(&std::fs::read("./shaders/skybox.vert.spv")?, UNLIT_FRAG, DrawType::Triangles)?;
        let (vertices, indices) = skybox_mesh();
        let skybox = Object {
            mesh: engine.add_mesh(&vertices, &indices)?,
            material: skybox_mat,
            transform: Matrix4::identity(),
        };

        Ok(Self {
            time: 0.0,
            rainbow_cube,
            skybox,
        })
    }

    fn next_frame(&mut self, engine: &mut dyn Engine) -> Result<FramePacket> {
        self.rainbow_cube.transform = Matrix4::from_euler_angles(0.0, self.time, 0.0);

        engine.update_time_value(self.time)?;
        self.time += 0.01;
        Ok(FramePacket {
            objects: vec![self.skybox, self.rainbow_cube],
        })
    }
}

fn main() -> Result<()> {
    let vr = std::env::args().skip(1).next().is_some();
    launch::<MyApp>(vr, ())
}

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

fn skybox_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let vert = |[x, y, z] : [f32; 3]| Vertex::new([x, y, z], [x.signum(), y.signum(), z.signum()]);
    let vertices = vec![
        vert([-1.0, -1.0, -1.0]),
        vert([1.0, -1.0, -1.0]),
        vert([1.0, 1.0, -1.0]),
        vert([-1.0, 1.0, -1.0]),
        vert([-1.0, -1.0, 1.0]),
        vert([1.0, -1.0, 1.0]),
        vert([1.0, 1.0, 1.0]),
        vert([-1.0, 1.0, 1.0]),
    ];

    let indices = vec![
        0, 1, 3, 3, 1, 2, 1, 5, 2, 2, 5, 6, 5, 4, 6, 6, 4, 7, 4, 0, 7, 7, 0, 3, 3, 2, 7, 7, 2, 6, 4, 5, 0, 0, 5, 1,
    ];

    (vertices, indices)
}
