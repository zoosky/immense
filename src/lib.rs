//! immense describes 3D structures with L-Systems and outputs them as Wavefront Object files you
//! can plug into your renderer of choice.
//!
//! # Basics
//!
//! There are some builtin rules for meshes such as `cube()`. We can build production rules from
//! them to describe structures. For example:
//!
//! ````
//! cube()
//! ````
//! ![](https://i.imgur.com/s68Kk0U.png)
//!
//! ````
//! cube().tf(Translate::x(3))
//! ````
//!
//! ![](https://i.imgur.com/1nALK9q.png)
//!
//! ````
//! cube().tf(Replicate::n(3, Translate::y(1.1)))
//! ````
//!
//! ![](https://i.imgur.com/xqufPmN.png)
//!
//! # Recursion
//!
//! Recursive structures are particularly convenient to represent.
//!
//! ```
//! fn recursive_tile(depth_budget: usize) -> Rule {
//!    let rule = Rule::new()
//!        .push(cube().tf(Translate::by(0.25, 0.25, 0.0)).tf(Scale::by(0.4)))
//!        .push(
//!            cube()
//!                .tf(Translate::by(-0.25, -0.25, 0.0))
//!                .tf(Scale::by(0.4)),
//!        )
//!        .push(
//!            cube()
//!                .tf(Translate::by(-0.25, 0.25, 0.0))
//!                .tf(Scale::by(0.4)),
//!        );
//!    if depth_budget > 0 {
//!        rule.push(
//!            recursive_tile(depth_budget - 1)
//!                .tf(Translate::by(0.25, -0.25, 0.0))
//!                .tf(Scale::by(0.5)),
//!        )
//!   } else {
//!        rule
//!    }
//!}
//!
//! ...
//!
//! recursive_tile(3)
//! ```
//!
//! ![](https://i.imgur.com/huqVLHE.png)
//!
//! # Randomness
//!
//! Rules can be constructed with randomness. To do this construction of the rule must be delayed
//! to mesh generation so the random values are different for each invocation, since the number of
//! invocations is not known until then. For needs like this we can make Rules and Subrules from
//! any type implementing ```ToRule```.
//!
//! ````
//! #[derive(Debug)] struct RandCube;
//!
//! impl ToRule for RandCube {fn to_rule(&self) -> Rule {cube().tf(*thread_rng()
//!    .choose(&[Translate::x(0.1), Translate::x(-0.1), Translate::x(0.2), Translate::x(-0.2),
//!            ])
//!            .unwrap())
//!    }
//!}
//!
//!...
//!
//! Rule::from(RandCube {}).tf(Replicate::n(4, Translate::y(1.0)))
//!````
//!
//! ![](https://i.imgur.com/bSNc6jw.png)

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
