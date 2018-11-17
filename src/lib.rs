// Copyright 2018 The immense Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! immense describes 3D structures with simple composable rules and outputs them as Wavefront
//! object files you can plug into your renderer of choice.
//!
//! # Demo
//!
//!```
//! # use immense::*;
//! Rule::new().push(vec![
//!     Replicate::n(1, vec![Tf::saturation(0.8), Tf::hue(160.0)]),
//!     Replicate::n(36, vec![Tf::rz(10.0), Tf::ty(0.1)]),
//!     Replicate::n(36, vec![Tf::ry(10.0), Tf::tz(1.2), Tf::hue(3.4)]),
//!    ],
//!    cube(),
//!)
//! # ;
//!```
//!
//! ![](https://i.imgur.com/1Emik4Z.png)
//!
//! # Table of Contents
//!
//! 1. [Intro](#intro)
//! 2. [Composing Rules](#composing_rules)
//!     1. [Recursion](#recursion)
//!     2. [Randomness](#randomness)
//! 3. [Color](#color)
//! 4. [Ergonomics Macros](#ergonomics-macros)
//! 5. [Custom Meshes](#custom-meshes)
//!
//! # Intro
//!
//! In immense, you create a [Rule][self::rule::Rule] that describes your structure, which is
//! ultimately composed of [meshes](https://en.wikipedia.org/wiki/Polygon_mesh). immense provides
//! some builtin meshes, such as [cube][self::rule::builtin::cube], and you can create your own rules
//! by using these builtins which you'll see in the next section.
//!
//! After you've built your [Rule][self::rule::Rule], you can export the meshes it expands to as a
//! Wavefront object file for the next part of your workflow, whether that is rendering it in Blender,
//! printing it in your 3D printer, or importing it into your game!
//!
//! # Composing Rules
//!
//! Let's start with a cube. You probably want to write your meshes to a file and watch them in a
//! viewer with autoreload. [Meshlab](http://www.meshlab.net/) is a great viewer (and much more)
//! that can reload your meshes when changed. Check out [ExportConfig][self::export::ExportConfig]
//! to see what options you can set that will work best for your rendering or printing workflow.
//!
//! ````
//! # use failure::{Error};
//! # let _ = || -> Result<(), Error> {
//! use immense::*;
//! use std::fs::File;
//!
//! let rule = cube();
//! let meshes = rule.generate();
//! let mut output_file = File::create("my_mesh.obj")?;
//! write_meshes(ExportConfig::default(), meshes, &mut output_file)?;
//! # Ok(())
//! # };
//! ````
//!
//!
//! ![](https://i.imgur.com/s68Kk0U.png)
//!
//! We can translate the cube with the `Tf::t*` family of functions which generate translate
//! transforms. We'll apply [Tf::tx][rule::transforms::Transform::tx] by creating our own rule and
//! invoking the cube rule with a transform.
//!
//! ````
//! # use immense::*;
//! let rule = Rule::new().push(Tf::tx(3.0), cube());
//! ````
//!
//! ![](https://i.imgur.com/1nALK9q.png)
//!
//! We can replicate transforms with [Replicate][rule::transforms::Replicate] which generates
//! multiple invocations of a subrule, each with more applications of the same transform applied to
//! it.
//!
//! ````
//! # use immense::*;
//! let rule = Rule::new().push(Replicate::n(3, Tf::ty(1.1)), cube());
//! ````
//!
//! Notice that our translation is 1.1 and that that is 0.1 more than the length of our cube. That's
//! no coincidence! All the built in meshes are 1 in length so that you can use convenient
//! measurements like this, even when deep in a transform stack.
//!
//! ![](https://i.imgur.com/xqufPmN.png)
//!
//! ## Recursion
//!
//! You can generate rules recursively with the api we've covered so far, but doing so would put
//! your entire rule tree in memory at one time, which can become a problem. immense provides a
//! trait, [ToRule][rule::ToRule], so you can give it types that can instantiate rules when needed.
//!
//! ````
//! # use immense::*;
//! struct RecursiveTile {
//!     depth_budget: usize,
//! }
//!
//! impl ToRule for RecursiveTile {
//!     fn to_rule(&self) -> Rule {
//!         let rule = Rule::new()
//!             .push(vec![Tf::t(0.25, 0.25, 0.0), Tf::s(0.4)], cube())
//!             .push(vec![Tf::t(-0.25, -0.25, 0.0), Tf::s(0.4)], cube())
//!             .push(vec![Tf::t(-0.25, 0.25, 0.0), Tf::s(0.4)], cube());
//!         if self.depth_budget > 0 {
//!             rule.push(
//!                 vec![Tf::t(0.25, -0.25, 0.0), Tf::s(0.4)],
//!                 RecursiveTile {
//!                     depth_budget: self.depth_budget - 1,
//!                 },
//!             )
//!         } else {
//!             rule
//!         }
//!     }
//! }
//!
//! let rule = RecursiveTile {
//!     depth_budget: 3
//! }.to_rule();
//! ````
//!
//! ![](https://i.imgur.com/huqVLHE.png)
//!
//! ## Randomness
//!
//! Using [ToRule][rule::ToRule] to delay rule construction, we can sample some random values
//! each time our type builds a rule.
//!
//! ````
//! # use immense::*;
//! # use rand::*;
//! struct RandCube;
//!
//! impl ToRule for RandCube {
//!     fn to_rule(&self) -> Rule {
//!         Rule::new().push(
//!             *thread_rng()
//!                 .choose(&[Tf::tx(0.1),
//!                           Tf::tx(-0.1),
//!                           Tf::tx(0.2),
//!                           Tf::tx(-0.2)])
//!                 .unwrap(),
//!             cube(),
//!         )
//!     }
//! }
//!
//! let rule = Rule::new().push(Replicate::n(4, Tf::ty(1.0)),
//!                             RandCube {});
//! ````
//!
//! ![](https://i.imgur.com/bSNc6jw.png)
//!
//! # Color
//!
//! immense can export some colors alongside your mesh, by linking the object file output to an
//! mtl file (material library). Set the output mtl file in
//! [export_colors][crate::export::ExportConfig::export_colors] and immense will write out colors.
//!
//! You can specify colors overrides and transforms in HSV color space using Ogeon's [palette][palette].
//! See [Tf::color][crate::rule::transforms::Transform::color], [Tf::hue][crate::rule::transforms::Transform::hue],
//! [Tf::saturation][crate::rule::transforms::Transform::saturation], [Tf::value][crate::rule::transforms::Transform::value].
//!
//! # Ergonomics Macros
//!
//! immense provides two ergonomics macros that make defining rules and transform sequences a little
//! easier once you have an intuition for their semantics. They are [`rule!`] and [`tf!`], which
//! help compose rules and transform sequences respectively.
//!
//! They transform the demo code above into:
//!
//! ````
//! # use immense::*;
//! rule![
//!     tf![
//!         Tf::saturation(0.8),
//!         Tf::hue(160.0),
//!         Replicate::n(36, vec![Tf::rz(10.0), Tf::ty(0.1)]),
//!         Replicate::n(36, vec![Tf::ry(10.0), Tf::tz(1.2), Tf::hue(3.4)]),
//!     ] => cube(),
//! ]
//! # ;
//! ````
//!
//! # Custom Meshes
//!
//! You can create meshes on your own and use them as rules by calling
//! [Mesh::from][self::mesh::Mesh::from] if you format your meshes according to object file format.
//!
//! Meshes can be expensive to allocate. immense handles the primitives on your behalf, but if you
//! introduce your own meshes you must be careful not to allocate them more than once. One million
//! references to a sphere are fine, one million spheres will probably kill the process.
//!
//! An example is the [sphere][self::rule::builtin::sphere] builtin which allows you to create a
//! potentially expensive sphere estimation:
//!
//! ````
//! # use immense::*;
//! # use std::rc::Rc;
//! let sphere: Rc<Mesh> = sphere(/*resolution=*/4);
//! let rule = Rule::new().push(Tf::s(2.0), sphere);
//! ````

mod error;
mod export;
mod mesh;
mod rule;

pub use crate::error::Error;
pub use crate::export::{ExportConfig, MeshGrouping};
pub use crate::mesh::{vertex, Mesh, Vertex};
pub use crate::rule::*;
pub use palette::{Hsv, RgbHue};

use crate::error::Result;
use std::io;

/// Writes out meshes as a Wavefront object file to the given [Write][io::Write] sink.
pub fn write_meshes(
    config: ExportConfig,
    meshes: impl Iterator<Item = OutputMesh>,
    sink: impl io::Write,
) -> Result<()> {
    export::write_meshes(config, meshes, sink)?;
    Ok(())
}
