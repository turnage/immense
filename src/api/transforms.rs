use crate::mesh::{vertex, Mesh, Vertex};
use nalgebra::{Matrix3, Matrix4, Vector3};
use std::iter;

fn identity() -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    )
}

pub type Tf = Transform;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    spatial: Matrix4<f32>,
}

impl Transform {
    pub(crate) fn cons(&self, other: Transform) -> Transform {
        // TODO: determine when translation to origin is necessary if ever.
        Transform {
            spatial: self.spatial * other.spatial,
        }
    }

    pub(crate) fn apply_to(&self, mesh: Mesh) -> Mesh {
        mesh.apply_matrix(self.spatial)
    }

    pub fn tx(x: f32) -> Self {
        Self {
            spatial: Translate::x(x),
            ..Self::default()
        }
    }

    pub fn t(x: f32, y: f32, z: f32) -> Self {
        Self {
            spatial: Translate::by(x, y, z),
            ..Self::default()
        }
    }

    pub fn ty(y: f32) -> Self {
        Self {
            spatial: Translate::y(y),
            ..Self::default()
        }
    }

    pub fn tz(z: f32) -> Self {
        Self {
            spatial: Translate::z(z),
            ..Self::default()
        }
    }

    pub fn s(factor: f32) -> Self {
        Self {
            spatial: Scale::all(factor),
            ..Self::default()
        }
    }

    pub fn rx(x: f32) -> Self {
        Self {
            spatial: Rotate::x(x),
            ..Self::default()
        }
    }

    pub fn ry(y: f32) -> Self {
        Self {
            spatial: Rotate::y(y),
            ..Self::default()
        }
    }

    pub fn rz(z: f32) -> Self {
        Self {
            spatial: Rotate::z(z),
            ..Self::default()
        }
    }

    // Multiplicatively branch transforms.
    fn cross(parents: Vec<Transform>, children: Vec<Transform>) -> Vec<Transform> {
        let mut emitted = vec![];
        emitted.reserve(parents.len() * children.len());
        for parent in parents {
            for child in &children {
                emitted.push(parent.cons(*child));
            }
        }
        emitted
    }

    fn coalesce(default: Option<Transform>, source: impl Iterator<Item = Transform>) -> Self {
        source.fold(default.unwrap_or(Transform::default()), |prefix, suffix| {
            prefix.cons(suffix)
        })
    }

    fn stack(self, n: usize) -> Self {
        Transform::coalesce(Some(self), iter::repeat(self).take(n))
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            spatial: identity(),
        }
    }
}

#[derive(Debug)]
pub enum TransformInput {
    Single(Transform),
    Many(Vec<Transform>),
}

impl Into<Vec<Transform>> for TransformInput {
    fn into(self) -> Vec<Transform> {
        match self {
            TransformInput::Single(transform) => vec![transform],
            TransformInput::Many(transforms) => transforms,
        }
    }
}

impl From<Transform> for TransformInput {
    fn from(transform: Transform) -> Self {
        TransformInput::Single(transform)
    }
}

impl From<Vec<Transform>> for TransformInput {
    fn from(transforms: Vec<Transform>) -> Self {
        TransformInput::Single(Transform::coalesce(None, transforms.into_iter()))
    }
}

impl From<&[Transform]> for TransformInput {
    fn from(transforms: &[Transform]) -> Self {
        TransformInput::Single(Transform::coalesce(None, transforms.iter().map(|t| *t)))
    }
}

impl From<Option<Transform>> for TransformInput {
    fn from(maybe_input: Option<Transform>) -> Self {
        match maybe_input {
            Some(input) => input.into(),
            None => TransformInput::Many(vec![]),
        }
    }
}

impl From<Vec<Replicate>> for TransformInput {
    fn from(replications: Vec<Replicate>) -> TransformInput {
        let mut emitted = vec![];
        for replication in replications.into_iter().map(|r| -> Vec<Transform> {
            let input: TransformInput = r.into();
            input.into()
        }) {
            emitted = if emitted.is_empty() {
                replication
            } else {
                Transform::cross(emitted, replication)
            };
        }
        TransformInput::Many(emitted)
    }
}

/// Replicates a transform ```n``` times. The transforms will stack, so ```Replicate::n(2,
/// Tf::x(1.0))``` on some rule will result in two invocations of the rule with
/// ```Tf::x(1.0)``` and ```Tf::x(2.0)```.
pub struct Replicate {
    n: usize,
    source: TransformInput,
}

impl Replicate {
    pub fn n(n: usize, source: impl Into<TransformInput>) -> Self {
        Self {
            n,
            source: source.into(),
        }
    }
}

impl Into<TransformInput> for Replicate {
    fn into(self) -> TransformInput {
        match self.source {
            TransformInput::Single(transform) => {
                TransformInput::Many((0..self.n).map(|i| transform.stack(i)).collect())
            }
            TransformInput::Many(transforms) => TransformInput::Many({
                let mut emitted = vec![];
                for transform in transforms {
                    for i in 0..self.n {
                        emitted.push(transform.stack(i));
                    }
                }
                emitted
            }),
        }
    }
}

/// Translates a rule. This is of course affected by earlier transformations, so
/// ```Translate::x(1.0)``` in a rule transformed by ```Scale::by(2.0)``` results in an absolute
/// translation of 2. This is almost always what you want.
#[derive(Default, Clone, Copy, Debug)]
struct Translate;

impl Translate {
    pub fn by(x: f32, y: f32, z: f32) -> Matrix4<f32> {
        Matrix4::new(
            1.0, 0.0, 0.0, x, //
            0.0, 1.0, 0.0, y, //
            0.0, 0.0, 1.0, z, //
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn x(x: f32) -> Matrix4<f32> {
        Translate::by(x, 0.0, 0.0)
    }
    pub fn y(y: f32) -> Matrix4<f32> {
        Translate::by(0.0, y, 0.0)
    }
    pub fn z(z: f32) -> Matrix4<f32> {
        Translate::by(0.0, 0.0, z)
    }
}

/// Scales the rule and all following transformations.
#[derive(Default, Clone, Copy, Debug)]
struct Scale;

impl Scale {
    pub fn all(factor: f32) -> Matrix4<f32> {
        Scale::by(factor, factor, factor)
    }

    pub fn by(x: f32, y: f32, z: f32) -> Matrix4<f32> {
        //Translate::by(0.5, 0.5, 0.5)*
        Matrix4::new(
            x, 0.0, 0.0, 0.0, //
            0.0, y, 0.0, 0.0, //
            0.0, 0.0, z, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        )
        //* Translate::by(-0.5, -0.5, -0.5)
    }
}

#[derive(Clone, Copy, Debug)]
struct Rotate;

impl Rotate {
    #[rustfmt::skip]
    pub fn x(x: f32) -> Matrix4<f32> {
        let r = x.to_radians();
        Translate::by(0.0, 0.5, 0.5) * Matrix4::new(
                1.0, 0.0,      0.0,      0.0, //
                0.0, r.cos(),  -r.sin(), 0.0, //
                0.0, r.sin(),  r.cos(),  0.0, //
                0.0, 0.0,      0.0,      1.0
            ) * Translate::by(0.0, -0.5, -0.5)
    }

    #[rustfmt::skip]
    pub fn y(y: f32) -> Matrix4<f32> {
        let r = y.to_radians();
        Matrix4::new(
                r.cos(),  0.0, r.sin(), 0.0, //
                0.0,      1.0, 0.0,     0.0, //
                -r.sin(), 0.0, r.cos(), 0.0, //
                0.0,      0.0, 0.0,     1.0
            )
    }

    #[rustfmt::skip]
    pub fn z(z: f32) -> Matrix4<f32> {
        let r = z.to_radians();
        Translate::by(0.5, 0.5, 0.0) * Matrix4::new(
                r.cos(), -r.sin(), 0.0, 0.0, //
                r.sin(), r.cos(),  0.0, 0.0, //
                0.0,     0.0,      1.0, 0.0, //
                0.0,     0.0,      0.0, 1.0
            ) * Translate::by(-0.5, -0.5, 0.0)
    }
}
