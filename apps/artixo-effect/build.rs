//! Windows only: embed the app icon and version info (VERSIONINFO) into `artixo-effect.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `ARTIXO_EFFECT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/artixo-effect.ico");
    println!("cargo:rerun-if-env-changed=ARTIXO_EFFECT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/artixo-effect.ico")
        .set("ProductName", "ARTIXO Effect")
        .set("FileDescription", "ARTIXO Effect motion graphics and visual effects")
        .set("LegalCopyright", "Copyright (c) 2026 ARTIXO ONE. Based on EffectCraft, Copyright (c) 2026 ArtCraft Team and the EffectCraft contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "artixo-effect.exe")
        .set("InternalName", "artixo-effect");
    if let Err(e) = res.compile() {
        if std::env::var_os("ARTIXO_EFFECT_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=artixo-effect.exe built without icon/version resources: {e}");
    }
}
