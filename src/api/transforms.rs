use crate::parameters::Parameters;
use nalgebra::Matrix4;

pub(crate) fn identity() -> Matrix4<f32> {
    Matrix4::new(
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0,
            )
}

pub trait Transform {
    fn transform(&self) -> Vec<Matrix4<f32>>;
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
    fn transform(&self) -> Vec<Matrix4<f32>> {
        let matrices = self.replicated_transform.transform();
        let mut current_matrices: Vec<Matrix4<f32>> = matrices.iter().map(|_| identity()).collect();
        let mut replicated_matrices = current_matrices.clone();
        for i in 0..self.n {
            let mut next_matrices: Vec<Matrix4<f32>> = matrices.iter().enumerate().map(|(i, m)| m * current_matrices[i]).collect();
            current_matrices = next_matrices.clone();
            replicated_matrices.append(&mut next_matrices);
        }
        replicated_matrices
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
    fn transform(&self) -> Vec<Matrix4<f32>> {
        vec![Matrix4::new(
                1.0, 0.0, 0.0, self.x, //
                0.0, 1.0, 0.0, self.y, //
                0.0, 0.0, 1.0, self.z, //
                0.0, 0.0, 0.0, 1.0,
            )]
    }
}
