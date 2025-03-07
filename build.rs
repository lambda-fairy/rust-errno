fn main() {
    // Cygwin target, added in 1.86
    println!("cargo:rustc-check-cfg=cfg(target_os, values(\"cygwin\"))");
}
