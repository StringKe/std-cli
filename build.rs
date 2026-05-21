fn main() {
    println!("cargo:rustc-env=STD_TEST_MODE=1");
    println!("cargo:rustc-env=STD_ALLOW_DESKTOP_AUTOMATION=0");
    println!("cargo:rustc-env=STD_ALLOW_UI_PREVIEW=0");
    println!("cargo:rustc-env=STD_ALLOW_BACKGROUND_UI_AUTOMATION=0");
}
