//! Core library for the `forge-sync` command line application.
//!
//! The supported embedding surface is deliberately small: parse a [`Config`]
//! and execute one synchronization pass with [`run_once`].

pub mod archive;
pub mod config;
pub mod engine;
pub mod forge;
pub mod github;
pub mod gitmirror;
pub mod model;
pub mod state;

pub use config::Config;
pub use engine::{run_once, RunReport};
