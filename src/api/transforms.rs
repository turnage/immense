use crate::mesh::{vertex, Vertex};
use crate::parameters::Parameters;
use nalgebra::Matrix4;
use std::fmt;
use std::rc::Rc;

pub(crate) fn identity() -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Wraps a transform matrix in a temporary relocation to the origin.
fn about_origin(current_transform: Matrix4<f32>, next_transform: Matrix4<f32>) -> Matrix4<f32> {
    let translation: Vertex = current_transform * vertex(0.0, 0.0, 0.0);
    let untranslate = Translate::by(-translation.x, -translation.y, -translation.z)
        .transform(current_transform)[0];
    let retranslate =
        Translate::by(translation.x, translation.y, translation.z).transform(current_transform)[0];
    retranslate * next_transform * untranslate
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

#[derive(Default, Clone, Copy, Debug)]
struct Identity;

impl Transform for Identity {
    fn transform(&self, _: Matrix4<f32>) -> Vec<Matrix4<f32>> {
        vec![identity()]
    }
}

/// A sequence of transforms.
#[derive(Clone)]
pub struct Seq {
    transforms: Vec<Rc<Transform>>,
}

impl Default for Seq {
    fn default() -> Seq {
        Self {
            transforms: vec![Rc::new(Identity {})],
        }
    }
}

impl fmt::Debug for Seq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Seq {{ transforms: {} }}", self.transforms.len(),)
    }
}

impl Seq {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, transform: impl Transform + 'static) -> Self {
        self.transforms.push(Rc::new(transform));
        self
    }

    fn descend<'a>(
        current_transform: Matrix4<f32>,
        emit_queue: Option<Matrix4<f32>>,
        mut next_transforms: impl Iterator<Item = &'a Rc<Transform>> + Clone,
    ) -> Vec<Matrix4<f32>> {
        match next_transforms.next() {
            Some(tf) => {
                let mut emitted = vec![];
                for t in tf.transform(current_transform) {
                    emitted.append(&mut Seq::descend(
                        current_transform * t,
                        Some(emit_queue.unwrap_or(identity()) * t),
                        next_transforms.clone(),
                    ));
                }
                emitted
            }
            None => emit_queue.map(|e| vec![e]).unwrap_or(vec![]),
        }
    }
}

impl Transform for Seq {
    fn transform(&self, current_transform: Matrix4<f32>) -> Vec<Matrix4<f32>> {
        Seq::descend(current_transform, None, self.transforms.iter())
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

impl<T: Transform + Clone + 'static> Transform for Replicate<T> {
    fn transform(&self, current_transform: Matrix4<f32>) -> Vec<Matrix4<f32>> {
        let mut emitted = vec![Seq::new().push(Identity {})];
        for i in 1..self.n {
            emitted.push((0..i).fold(Seq::new(), |seq, _| {
                seq.push(self.replicated_transform.clone())
            }));
        }
        emitted
            .into_iter()
            .flat_map(|seq| seq.transform(current_transform))
            .collect()
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
        #[rustfmt::skip]
        let coord_scaler = Matrix4::new(
            self.factor, 0.0, 0.0, 0.0, //
            0.0, self.factor, 0.0, 0.0, //
            0.0, 0.0, self.factor, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        );
        vec![about_origin(current_transform, coord_scaler)]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Rotate {
    X(f32),
    Y(f32),
    Z(f32),
}

impl Rotate {
    pub fn x(x: f32) -> Self {
        Rotate::X(x.to_radians())
    }

    pub fn y(y: f32) -> Self {
        Rotate::Y(y.to_radians())
    }

    pub fn z(z: f32) -> Self {
        Rotate::Z(z.to_radians())
    }
}

impl Transform for Rotate {
    fn transform(&self, current_transform: Matrix4<f32>) -> Vec<Matrix4<f32>> {
        #[rustfmt::skip]
        vec![match *self {
            Rotate::X(r) => about_origin(current_transform, Matrix4::new(
                1.0, 0.0,      0.0,      0.0, //
                0.0, r.cos(),  -r.sin(), 0.0, //
                0.0, r.sin(),  r.cos(),  0.0, //
                0.0, 0.0,      0.0,      1.0
            )),
            Rotate::Y(r) => about_origin(current_transform, Matrix4::new(
                r.cos(),  0.0, r.sin(), 0.0, //
                0.0,      1.0, 0.0,     0.0, //
                -r.sin(), 0.0, r.cos(), 0.0, //
                0.0,      0.0, 0.0,     1.0
            )),
            Rotate::Z(r) => about_origin(current_transform, Matrix4::new(
                r.cos(), -r.sin(), 0.0, 0.0, //
                r.sin(), r.cos(),  0.0, 0.0, //
                0.0,     0.0,      1.0, 0.0, //
                0.0,     0.0,      0.0, 1.0
            ))
        }]
    }
}
