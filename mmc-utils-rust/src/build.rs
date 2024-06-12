// build.rs
fn main() {
    // Tell cargo to tell rustc to link the mmc_cmds
    // library located in the "../../mmc-utils" directory.
    println!("cargo:rustc-link-search=../../mmc-utils");
    println!("cargo:rustc-link-lib=mmc_cmds");
}