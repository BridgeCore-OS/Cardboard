// Copyright (c) ChefKiss Inc 2021-2022.
// This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

pub fn main() {
    println!("cargo:rustc-link-arg-bins=-Tcardboard/src/linker.ld");
    println!("cargo:rerun-if-changed=cardboard/src/linker.ld");
}