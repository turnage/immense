use crate::parameters::Parameters;
use nalgebra::Matrix4;

pub trait Transform {
    fn transform(&self, parameters: Parameters) -> Vec<Parameters>;
}

pub struct Replicate<T> {
    n: usize,
    replicated_transform: T,
}

impl<T> Replicate<T> {
    pub fn n(n: usize, replicated_transform: T) -> Self {
        Self {
            n,
            replicated_transform,
        }
    }
}

impl<T: Transform> Transform for Replicate<T> {
    fn transform(&self, parameters: Parameters) -> Vec<Parameters> {
        let mut current_parameters = self.replicated_transform.transform(parameters);
        let mut emitted_parameters = current_parameters.clone();
        for i in 0..self.n {
            current_parameters = current_parameters
                .into_iter()
                .flat_map(|p| self.replicated_transform.transform(p))
                .collect();
            emitted_parameters.append(&mut current_parameters.clone());
        }
        emitted_parameters
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Translate {
    x: f32,
    y: f32,
    z: f32,
}

impl Translate {
    pub fn x(x: f32) -> Self {
        Self {
            x,
            ..Default::default()
        }
    }
    pub fn y(y: f32) -> Self {
        Self {
            y,
            ..Default::default()
        }
    }
    pub fn z(z: f32) -> Self {
        Self {
            z,
            ..Default::default()
        }
    }
}

impl Transform for Translate {
    fn transform(&self, parameters: Parameters) -> Vec<Parameters> {
        vec![Parameters {
            transform: Matrix4::new(
                1.0, 0.0, 0.0, self.x, //
                0.0, 1.0, 0.0, self.y, //
                0.0, 0.0, 1.0, self.z, //
                0.0, 0.0, 0.0, 1.0,
            ) * parameters.transform,
            ..parameters
        }]
    }
}
