//! immense describes 3D structures with L-Systems and outputs them as Wavefront Object files you
//! can plug into your renderer of choice.
//!
//! # Intro
//!
//! We start with some builtin rules such as [cube][api::builtin::cube], and create structures by
//! transforming and replicating those rules. Here's an example of how expressive this can be:
//!
//!```
//! # use immense::*;
//! Rule::new().push(vec![
//!     Replicate::n(36, vec![Tf::rz(10.0), Tf::ty(0.1)]),
//!     Replicate::n(36, vec![Tf::ry(10.0), Tf::tz(1.2)]),
//!    ],
//!    cube(),
//!)
//! # ;
//!```
//!
//! ![](https://i.imgur.com/5ccKkpQ.png)
//!
//! # Basics
//!
//! Let's start with a cube. You probably want to write your meshes to a file and watch them in a
//! viewer with autoreload. [Meshlab](http://www.meshlab.net/) is a great viewer (and much more)
//! that can reload your meshes when changed.
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
//! write_meshes(meshes, &mut output_file)?;
//! # Ok(())
//! # };
//! ````
//!
//! ![](https://i.imgur.com/s68Kk0U.png)
//!
//! We can translate the cube with the `Tf::t*` family of functions which generate translate
//! transforms. We'll apply [Tf::tx][api::transforms::Transform::tx] by creating our own rule and
//! invoking the cube rule with a transform.
//!
//! ````
//! # use immense::*;
//! Rule::new().push(Tf::tx(3.0), cube())
//! # ;
//! ````
//!
//! ![](https://i.imgur.com/1nALK9q.png)
//!
//! We can replicate transforms with [Replicate][api::transforms::Replicate] which generates
//! multiple invocations of a subrule, each with more applications of the same transform applied to
//! it.
//!
//! ````
//! # use immense::*;
//! Rule::new().push(Replicate::n(3, Tf::ty(1.1)), cube())
//! # ;
//! ````
//!
//! Notice that our translation is 1.1 and that that is 0.1 more than the length of our cube. That's
//! no coincidence! All the built in meshes are 1 in length so that you can use convenient
//! measurements like this, even when deep in a transform stack.
//!
//! ![](https://i.imgur.com/xqufPmN.png)
//!
//! # Recursion
//!
//! You can generate rules recursively with the api we've covered so far, but doing so would put
//! your entire rule tree in memory at one time, which can become a problem. immense provides a
//! trait, [ToRule][api::ToRule], so you can give it types that can instantiate rules when needed.
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
//! RecursiveTile {
//!     depth_budget: 3
//! }.to_rule()
//! # ;
//! ````
//!
//! ![](https://i.imgur.com/huqVLHE.png)
//!
//! # Randomness
//!
//! Using [ToRule][api::ToRule] to delay rule construction, we can sample some random values
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
//!                 .choose(&[Tf::tx(0.1), Tf::tx(-0.1), Tf::tx(0.2), Tf::tx(-0.2)])
//!                 .unwrap(),
//!             cube(),
//!         )
//!     }
//! }
//!
//! Rule::new().push(Replicate::n(4, Tf::ty(1.0)), RandCube {})
//! # ;
//! ````
//!
//! ![](https://i.imgur.com/bSNc6jw.png)
//!
//!

#![feature(custom_attribute)]
#![feature(bind_by_move_pattern_guards)]
#![feature(stmt_expr_attributes)]
#![feature(const_fn)]

mod api;
mod error;
mod export;
mod mesh;

pub use crate::api::*;
pub use crate::error::Error;

use crate::error::Result;
use std::io;

pub fn write_meshes(meshes: Vec<mesh::Mesh>, mut sink: impl io::Write) -> Result<()> {
    let mut vertex_offset = 0;
    for mesh in meshes {
        let vertex_count = mesh.vertices.len();
        export::render_obj(mesh, vertex_offset, &mut sink)?;
        vertex_offset += vertex_count;
    }
    Ok(())
}
