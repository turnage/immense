use nalgebra::Matrix4;

#[derive(Copy, Clone, Debug)]
pub struct Parameters {
    pub transform: Matrix4<f32>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
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
}
