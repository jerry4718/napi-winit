use cfg_aliases::cfg_aliases;

#[allow(
    clippy::disallowed_macros,
    reason = "The build script defines cfg aliases for platform-specific bindings."
)]
fn main() {
    napi_build::setup();

    println!("cargo:rerun-if-changed=build.rs");

    cfg_aliases! {
        windows_platform: { target_os = "windows" },
        macos_platform: { target_os = "macos" },
        free_unix: { all(unix, not(target_vendor = "apple"), not(target_os = "android"), not(target_os = "emscripten")) },
        redox: { target_os = "redox" },
        x11_platform: { all(feature = "x11", free_unix, not(redox)) },
        wayland_platform: { all(feature = "wayland", free_unix, not(redox)) },
    }
}
