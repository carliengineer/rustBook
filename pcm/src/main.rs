//multiple paths from same origin
use std::{cmp::Ordering, io};
// glob operator,  to bring all public items defined in a path into scope
use std::collections::*;

/*
    Packages: A Cargo feature that lets you build, test, and share crates. 
              Several rules determine what a package can contain. 
              A package must contain zero or one library crates, and no more.
              It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).
    Crates: A tree of modules that produces a library or executable
               If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package.
               A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
    Modules and use: Let you control the organization, scope, and privacy of paths
               Modules let us organize code within a crate into groups for readability and easy reuse. 
               Modules also control the privacy of items, which is whether an item can be used by outside code (public)
               or is an internal implementation detail and not available for outside use (private).
    Paths: A way of naming an item, such as a struct, function, or module
*/

fn main() {
    println!("yo yo yo, world!");
}
