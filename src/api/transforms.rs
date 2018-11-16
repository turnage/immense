use crate::mesh::Vertex;
use nalgebra::Matrix4;
use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, RgbHue};
use std::iter;

fn identity() -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    )
}

/// An ergonomic alias for [Transform][self::Transform].
pub type Tf = Transform;

/// A Transform, when applied, modifies a mesh. When applied to a rule, it transforms all the meshes
/// that rule eventually expands to. Transforms may be translations, scales, rotations, etc.
///
/// It may be helpful to think of transforms to rules as transforming the space in which the rule or
/// mesh is evaluated. For example this rule will translate a cube 4.0 on the x axis in our root
/// frame of reference:
///
/// ````
/// # use immense::*;
/// let our_translated_cube = Rule::new().push(Tf::tx(4.0), cube());
/// # ;
/// ````
///
/// This rule will translate a cube -4.0 on the x axis in our root frame of reference:
///
/// ````
/// # use immense::*;
/// # let our_translated_cube = Rule::new().push(Tf::tx(4.0), cube());
///let containing_rule = Rule::new().push(Tf::ry(180.0), our_translated_cube);
/// # ;
/// ````
///
/// This rule will translate a half-sized cube 2.0 on the x axis in our root frame of reference.
///
/// ````
/// # use immense::*;
/// # let our_translated_cube = Rule::new().push(Tf::tx(4.0), cube());
/// let containing_rule = Rule::new().push(Tf::s(0.5), our_translated_cube)
/// # ;
/// ````
#[derive(Copy, Clone, Debug)]
pub struct Transform {
    spatial: Matrix4<f32>,
    color: ColorTransform,
}

#[derive(Copy, Clone, Debug)]
enum ColorTransform {
    Override(Hsv),
    Delta(Hsv),
}

impl Default for ColorTransform {
    fn default() -> ColorTransform {
        ColorTransform::Delta(Hsv::new(0.0, 1.0, 1.0))
    }
}

impl ColorTransform {
    fn cons(self, other: ColorTransform) -> Self {
        match (self, other) {
            (_, ColorTransform::Override(color)) => ColorTransform::Override(color),
            (ColorTransform::Override(color), ColorTransform::Delta(delta)) => {
                ColorTransform::Override(Hsv::new(
                    color.hue + delta.hue,
                    color.saturation * delta.saturation,
                    color.value * delta.value,
                ))
            }
            (ColorTransform::Delta(delta_a), ColorTransform::Delta(delta_b)) => {
                ColorTransform::Delta(Hsv::new(
                    delta_a.hue + delta_b.hue,
                    delta_a.saturation * delta_b.saturation,
                    delta_a.value * delta_b.value,
                ))
            }
        }
    }

    fn color(self) -> Hsv {
        match self {
            ColorTransform::Override(color) => color,
            ColorTransform::Delta(delta) => {
                let color = Hsv::new(0.0, 1.0, 1.0);
                Hsv::new(
                    color.hue + delta.hue,
                    color.saturation * delta.saturation,
                    color.value * delta.value,
                )
            }
        }
    }
}

impl Transform {
    pub(crate) fn cons(&self, other: Transform) -> Transform {
        // TODO: determine when translation to origin is necessary if ever.
        Transform {
            spatial: self.spatial * other.spatial,
            color: self.color.cons(other.color),
        }
    }

    pub(crate) fn apply_to(&self, vertex: Vertex) -> Vertex {
        self.spatial * vertex
    }

    pub(crate) fn get_color(&self) -> Rgb<Srgb, f32> {
        Rgb::from(
            ColorTransform::Override(Hsv::new(0.0, 1.0, 1.0))
                .cons(self.color)
                .color(),
        )
    }

    /// A translation on all axes.
    pub fn t(x: f32, y: f32, z: f32) -> Self {
        Self {
            spatial: Translate::by(x, y, z),
            ..Self::default()
        }
    }

    /// A translation on the x axis.
    pub fn tx(x: f32) -> Self {
        Self {
            spatial: Translate::x(x),
            ..Self::default()
        }
    }

    /// A translation on the y axis.
    pub fn ty(y: f32) -> Self {
        Self {
            spatial: Translate::y(y),
            ..Self::default()
        }
    }

    /// A translation on the z axis.
    pub fn tz(z: f32) -> Self {
        Self {
            spatial: Translate::z(z),
            ..Self::default()
        }
    }

    /// A uniform scale in all dimensions.
    pub fn s(factor: f32) -> Self {
        Self {
            spatial: Scale::all(factor),
            ..Self::default()
        }
    }

    /// A scale in all dimensions.
    pub fn sby(x: f32, y: f32, z: f32) -> Self {
        Self {
            spatial: Scale::by(x, y, z),
            ..Self::default()
        }
    }

    /// A rotation about the x axis.
    pub fn rx(x: f32) -> Self {
        Self {
            spatial: Rotate::x(x),
            ..Self::default()
        }
    }

    /// A rotation about the y axis.
    pub fn ry(y: f32) -> Self {
        Self {
            spatial: Rotate::y(y),
            ..Self::default()
        }
    }

    /// A rotation about the z axis.
    pub fn rz(z: f32) -> Self {
        Self {
            spatial: Rotate::z(z),
            ..Self::default()
        }
    }

    /// A color override that takes precedence over colors set higher in the rule tree.
    pub fn color(color: Hsv) -> Self {
        Self {
            color: ColorTransform::Override(color),
            ..Self::default()
        }
    }

    /// Adds `delta` to the current color hue.
    pub fn hue(delta: impl Into<RgbHue<f32>>) -> Self {
        Self {
            color: ColorTransform::Delta(Hsv::new(delta, 1.0, 1.0)),
            ..Self::default()
        }
    }

    /// Multiplies the current color saturation by `factor`
    pub fn saturation(factor: f32) -> Self {
        Self {
            color: ColorTransform::Delta(Hsv::new(0.0, factor, 1.0)),
            ..Self::default()
        }
    }

    /// Multiplies the current color value by `factor`.
    pub fn value(factor: f32) -> Self {
        Self {
            color: ColorTransform::Delta(Hsv::new(0.0, 1.0, factor)),
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
            color: ColorTransform::default(),
        }
    }
}

/// A TransformArgument is a transform that should be applied to the invocation of a
/// [Rule][crate::api::Rule].
///
/// See the [From][std::convert::From] and [Into][std::convert::Into] implementations
/// which produce this type to find out what kind of argument each type becomes.
#[derive(Debug)]
pub enum TransformArgument {
    /// A single transform that corresponds to one invocation with the given transform.
    Single(Transform),
    /// An arbitrary number of transforms (e.g. from [Replicate][self::Replicate]) that correspond
    /// to one invocation each.
    Many(Vec<Transform>),
}

/// An ergonomics macro for listing transforms that will apply in order and branch on replications.
#[macro_export]
macro_rules! tf {
    ($($transform:expr),+ $(,)*) => ({
        let mut args: Vec<TransformArgument> = vec![];
        $(args.push($transform.into());)*
        args
    });
}

impl Into<Vec<Transform>> for TransformArgument {
    fn into(self) -> Vec<Transform> {
        match self {
            TransformArgument::Single(transform) => vec![transform],
            TransformArgument::Many(transforms) => transforms,
        }
    }
}

/// A single transform will correspond to one invocation.
impl From<Transform> for TransformArgument {
    fn from(transform: Transform) -> Self {
        TransformArgument::Single(transform)
    }
}

/// A vector of transforms will be sequentially composed into a single transform and correspond to
/// one invocation.
impl From<Vec<Transform>> for TransformArgument {
    fn from(transforms: Vec<Transform>) -> Self {
        TransformArgument::Single(Transform::coalesce(None, transforms.into_iter()))
    }
}

/// A slice of transforms will be sequentially composed into a single transform and correspond to
/// one invocation.
impl From<&[Transform]> for TransformArgument {
    fn from(transforms: &[Transform]) -> Self {
        TransformArgument::Single(Transform::coalesce(None, transforms.iter().map(|t| *t)))
    }
}

impl From<Vec<TransformArgument>> for TransformArgument {
    fn from(args: Vec<TransformArgument>) -> Self {
        let mut emitted = vec![Transform::default()];
        for arg in args {
            emitted = Transform::cross(emitted, arg.into());
        }
        TransformArgument::Many(emitted)
    }
}

/// An optional transform will of course correspond to one invocation. This implementation
/// also allows you to pass [None][std::option::Option::None] to invoke rules unmodified.
impl From<Option<Transform>> for TransformArgument {
    fn from(maybe_input: Option<Transform>) -> Self {
        match maybe_input {
            Some(input) => input.into(),
            None => TransformArgument::Many(vec![]),
        }
    }
}

/// A vector of replications will be composed sequentially, which means the number of corresponding
/// rule invocations is the product of each replication. A vector with a replication of transform A
/// 36 times then replication of B 10 times will yield transforms for every sequence of A then B
/// (e.g. (A_1, B_1), (A_1, B_2), ..., (A_36, B_36)), so 360 total.
impl From<Vec<Replicate>> for TransformArgument {
    fn from(replications: Vec<Replicate>) -> TransformArgument {
        let mut emitted = vec![];
        for replication in replications.into_iter().map(|r| -> Vec<Transform> {
            let input: TransformArgument = r.into();
            input.into()
        }) {
            emitted = if emitted.is_empty() {
                replication
            } else {
                Transform::cross(emitted, replication)
            };
        }
        TransformArgument::Many(emitted)
    }
}

/// Replicates a transform n times.
///
/// The transforms will stack, so ```Replicate::n(2, Tf::x(1.0))``` on some rule will result in two
/// invocations of the rule with ```Tf::x(1.0)``` and ```Tf::x(2.0)```.
pub struct Replicate {
    n: usize,
    source: TransformArgument,
}

impl Replicate {
    pub fn n(n: usize, source: impl Into<TransformArgument>) -> Self {
        Self {
            n,
            source: source.into(),
        }
    }
}

/// The replication will become ```n``` transforms, corresponding to one invocation each.
impl Into<TransformArgument> for Replicate {
    fn into(self) -> TransformArgument {
        match self.source {
            TransformArgument::Single(transform) => {
                TransformArgument::Many((0..self.n).map(|i| transform.stack(i)).collect())
            }
            TransformArgument::Many(transforms) => TransformArgument::Many({
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

#[derive(Default, Clone, Copy, Debug)]
struct Scale;

impl Scale {
    pub fn all(factor: f32) -> Matrix4<f32> {
        Scale::by(factor, factor, factor)
    }

    pub fn by(x: f32, y: f32, z: f32) -> Matrix4<f32> {
        Matrix4::new(
            x, 0.0, 0.0, 0.0, //
            0.0, y, 0.0, 0.0, //
            0.0, 0.0, z, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        )
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
        Translate::by(0.5, 0.0, 0.5) * Matrix4::new(
                r.cos(),  0.0, r.sin(), 0.0, //
                0.0,      1.0, 0.0,     0.0, //
                -r.sin(), 0.0, r.cos(), 0.0, //
                0.0,      0.0, 0.0,     1.0
            )* Translate::by(-0.5, 0.0, -0.5)
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
