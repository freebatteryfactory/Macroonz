#![doc = include_str!("README.md")]

mod codec;
mod companions;
mod dispatch;
mod evidence;
mod project;
mod relation_tables;
mod tokens;
mod typestate;

pub(in crate::recipe) use project::project;
