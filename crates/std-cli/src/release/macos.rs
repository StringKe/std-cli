use crate::CliError;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy)]
pub(crate) struct MacAppBundleSpec {
    pub(crate) name: &'static str,
    pub(crate) executable: &'static str,
    pub(crate) identifier: &'static str,
    pub(crate) agent: bool,
}

pub(crate) fn default_app_bundles() -> [MacAppBundleSpec; 2] {
    [
        MacAppBundleSpec {
            name: "std Launcher",
            executable: "std-launcher",
            identifier: "com.stringke.std-cli.launcher",
            agent: true,
        },
        MacAppBundleSpec {
            name: "std Studio",
            executable: "std-studio",
            identifier: "com.stringke.std-cli.studio",
            agent: false,
        },
    ]
}

pub(crate) fn create_mac_app_bundle(
    apps_dir: &Path,
    source_binary: &Path,
    spec: MacAppBundleSpec,
    version: &str,
) -> Result<PathBuf, CliError> {
    if !source_binary.is_file() {
        return Err(CliError::Install(format!(
            "missing binary: {}",
            source_binary.display()
        )));
    }

    let bundle_dir = apps_dir.join(format!("{}.app", spec.name));
    let contents_dir = bundle_dir.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    fs::create_dir_all(&macos_dir)?;
    fs::copy(source_binary, macos_dir.join(spec.executable))?;
    fs::write(
        contents_dir.join("Info.plist"),
        mac_app_info_plist(spec, version),
    )?;
    Ok(bundle_dir)
}

pub(crate) fn verify_app_bundle(bundle: &Path) -> Result<(), CliError> {
    let _ = app_bundle_executable(bundle)?;
    Ok(())
}

pub(crate) fn app_bundle_executable(bundle: &Path) -> Result<PathBuf, CliError> {
    let contents_dir = bundle.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    let plist = contents_dir.join("Info.plist");
    if !plist.is_file() {
        return Err(CliError::Install(format!(
            "app bundle missing Info.plist: {}",
            plist.display()
        )));
    }
    let plist_body = fs::read_to_string(&plist)?;
    let executable = plist_body
        .split("<key>CFBundleExecutable</key>")
        .nth(1)
        .and_then(|tail| tail.split("<string>").nth(1))
        .and_then(|tail| tail.split("</string>").next())
        .ok_or_else(|| {
            CliError::Install(format!(
                "app bundle Info.plist missing CFBundleExecutable: {}",
                plist.display()
            ))
        })?;
    let binary = macos_dir.join(executable);
    if !binary.is_file() {
        return Err(CliError::Install(format!(
            "app bundle missing executable: {}",
            binary.display()
        )));
    }
    Ok(binary)
}

fn mac_app_info_plist(spec: MacAppBundleSpec, version: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleDisplayName</key>
  <string>{name}</string>
  <key>CFBundleExecutable</key>
  <string>{executable}</string>
  <key>CFBundleIdentifier</key>
  <string>{identifier}</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>{name}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>{version}</string>
  <key>CFBundleVersion</key>
  <string>{version}</string>
      <key>LSMinimumSystemVersion</key>
      <string>13.0</string>
      {agent_plist}
    </dict>
    </plist>
    "#,
        name = spec.name,
        executable = spec.executable,
        identifier = spec.identifier,
        version = version,
        agent_plist = if spec.agent {
            "<key>LSUIElement</key>\n  <true/>"
        } else {
            ""
        }
    )
}
