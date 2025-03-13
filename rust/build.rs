fn main() {
    println!("cargo:rerun-if-changed=binding/ts.go");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    std::process::Command::new("go")
        .args(&[
            "build",
            "-v",
            "-buildmode=c-archive",
            r#"-ldflags=-extldflags='-static'"#,
            "-o",
            &format!("{}/libts.a", out_dir),
            "binding/ts.go",
        ])
        .status()
        .expect("Failed to build ts.go");

    println!("cargo:rustc-link-search=native={}", out_dir);

    if std::env::var("TARGET") == Ok("x86_64-pc-windows-msvc".to_string()) {
        println!("cargo:rustc-link-lib=legacy_stdio_definitions");
    }

    napi_build::setup();
}
