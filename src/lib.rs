// [[file:../bevy.note::2fdba27f][2fdba27f]]
// #![deny(warnings)]
// #![deny(clippy::all)]
// 2fdba27f ends here

// [[file:../bevy.note::3edfd207][3edfd207]]
mod animation;
mod arcball;
mod base;
mod net;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
pub mod molecule;
// 3edfd207 ends here

// [[file:../bevy.note::043e6795][043e6795]]
use bevy::app::App;
use bevy::prelude::*;
// 043e6795 ends here
