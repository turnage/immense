use nalgebra::Matrix4;

#[derive(Copy, Clone, Debug)]
pub struct Parameters {
    pub depth_budget: usize,
    pub transform: Matrix4<f32>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            depth_budget: 10,
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0,
            ),
        }
    }
}

impl Parameters {
    pub fn scale(self, factor: f32) -> Self {
        Self {
            transform: factor * self.transform,
            ..self
        }
    }

    pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            transform: Matrix4::new(
                1.0, 0.0, 0.0, x, //
                0.0, 1.0, 0.0, y, //
                0.0, 0.0, 1.0, z, //
                0.0, 0.0, 0.0, 1.0,
            ) * self.transform,
            ..self
        }
    }
}
