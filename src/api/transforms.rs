use crate::mesh::{vertex, Vertex};
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

/// Transform describes a type which can generate transform matrices for mesh generation.
pub trait Transform {
    // transform should return the matrices that should be multiplied with the current transform in
    // order to apply. The current transform should not be multiplied in this method; it is provided
    // so that transforms which must be aware of the current state (e.g. Scale in case of a
    // translated origin) can generate a correct matrix.
    fn transform(&self, current_transform: Matrix4<f32>) -> Vec<Matrix4<f32>>;
}

impl Transform for &[Matrix4<f32>] {
    fn transform(&self, _: Matrix4<f32>) -> Vec<Matrix4<f32>> {
        self.to_vec()
    }
}

/// Replicates a transform ```n``` times. The transforms will stack, so ```Replicate::n(2,
/// Translate::x(1.0))``` on some rule will result in two invocations of the rule with
/// ```Translate::x(1.0)``` and ```Translate::x(2.0)```.
pub struct Replicate<T> {
    n: usize,
    replicated_transform: T,
}

impl<T: Copy> Copy for Replicate<T> {}

impl<T: Copy> Clone for Replicate<T> {
    fn clone(&self) -> Self {
        *self
    }
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
    fn transform(&self, current_transform: Matrix4<f32>) -> Vec<Matrix4<f32>> {
        let matrices = self.replicated_transform.transform(current_transform);
        let mut current_matrices: Vec<Matrix4<f32>> = matrices.iter().map(|_| identity()).collect();
        let mut replicated_matrices = current_matrices.clone();
        for i in 1..self.n {
            let mut next_matrices: Vec<Matrix4<f32>> = matrices
                .iter()
                .enumerate()
                .map(|(i, m)| m * current_matrices[i])
                .collect();
            current_matrices = next_matrices.clone();
            replicated_matrices.append(&mut next_matrices);
        }
        replicated_matrices
    }
}

/// Translates a rule. This is of course affected by earlier transformations, so
/// ```Translate::x(1.0)``` in a rule transformed by ```Scale::by(2.0)``` results in an absolute
/// translation of 2. This is almost always what you want.
#[derive(Default, Clone, Copy, Debug)]
pub struct Translate {
    x: f32,
    y: f32,
    z: f32,
}

impl Translate {
    pub fn by(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

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
    fn transform(&self, _: Matrix4<f32>) -> Vec<Matrix4<f32>> {
        vec![Matrix4::new(
            1.0, 0.0, 0.0, self.x, //
            0.0, 1.0, 0.0, self.y, //
            0.0, 0.0, 1.0, self.z, //
            0.0, 0.0, 0.0, 1.0,
        )]
    }
}

/// Scales the rule and all following transformations.
#[derive(Default, Clone, Copy, Debug)]
pub struct Scale {
    factor: f32,
}

impl Scale {
    pub fn by(factor: f32) -> Self {
        Self { factor }
    }
}

impl Transform for Scale {
    fn transform(&self, current_transform: Matrix4<f32>) -> Vec<Matrix4<f32>> {
        let translation: Vertex = current_transform * vertex(0.0, 0.0, 0.0);
        let untranslate = Translate::by(-translation.x, -translation.y, -translation.z)
            .transform(current_transform)[0];
        let retranslate = Translate::by(translation.x, translation.y, translation.z)
            .transform(current_transform)[0];
        let coord_scaler = Matrix4::new(
            self.factor,
            0.0,
            0.0,
            0.0, //
            0.0,
            self.factor,
            0.0,
            0.0, //
            0.0,
            0.0,
            self.factor,
            0.0, //
            0.0,
            0.0,
            0.0,
            1.0,
        );
        vec![retranslate * coord_scaler * untranslate]
    }
}
