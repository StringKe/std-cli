use crate::release::macos::{create_mac_app_bundle, default_app_bundles, verify_app_bundle};
use crate::CliError;
use std::{
    fs,
    path::{Path, PathBuf},
};
use std_core::StdCore;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub(crate) fn install_plan(core: &StdCore, prefix: Option<&Path>) -> Result<String, CliError> {
    let prefix = install_prefix(prefix);
    let bin_dir = prefix.join("bin");
    let apps_dir = prefix.join("Applications");
    let lines = [
        format!("install_prefix={}", prefix.display()),
        format!("bin_dir={}", bin_dir.display()),
        format!("apps_dir={}", apps_dir.display()),
        format!("data_dir={}", core.config.data_dir.display()),
        format!("workflows_dir={}", core.config.workflows_dir().display()),
        format!("plugins_dir={}", core.config.plugins_dir().display()),
        "binaries=std,std-launcher,std-studio".to_string(),
        "app_bundles=std Launcher.app,std Studio.app".to_string(),
        format!(
            "next=cargo build --release && std install run --prefix {} --from target/release",
            prefix.display()
        ),
        format!("verify=std install verify --prefix {}", prefix.display()),
    ];
    Ok(lines.join("\n"))
}

pub(crate) fn install_run(
    core: &StdCore,
    prefix: Option<&Path>,
    from: Option<&Path>,
) -> Result<String, CliError> {
    core.ensure_storage()?;
    let prefix = install_prefix(prefix);
    let bin_dir = prefix.join("bin");
    let apps_dir = prefix.join("Applications");
    let source_dir = match from {
        Some(path) => path.to_path_buf(),
        None => std::env::current_exe()?
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| CliError::Install("current executable has no parent".to_string()))?,
    };
    fs::create_dir_all(&bin_dir)?;
    fs::create_dir_all(&apps_dir)?;

    let mut installed = Vec::new();
    for binary in ["std", "std-launcher", "std-studio"] {
        let source = source_dir.join(binary);
        if !source.is_file() {
            return Err(CliError::Install(format!(
                "missing binary: {}",
                source.display()
            )));
        }
        let target = bin_dir.join(binary);
        fs::copy(&source, &target)?;
        make_executable(&target)?;
        installed.push(target.display().to_string());
    }
    let app_bundles = install_app_bundles(&source_dir, &apps_dir)?;

    Ok(format!(
        "installed\nprefix={}\nbin_dir={}\napps_dir={}\n{}\n{}",
        prefix.display(),
        bin_dir.display(),
        apps_dir.display(),
        installed
            .into_iter()
            .map(|path| format!("binary={path}"))
            .collect::<Vec<_>>()
            .join("\n"),
        app_bundles
            .into_iter()
            .map(|path| format!("app_bundle={}", path.display()))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}

pub(crate) fn install_verify(core: &StdCore, prefix: Option<&Path>) -> Result<String, CliError> {
    let prefix = install_prefix(prefix);
    let bin_dir = prefix.join("bin");
    let apps_dir = prefix.join("Applications");
    let mut verified = Vec::new();
    for binary in ["std", "std-launcher", "std-studio"] {
        let path = bin_dir.join(binary);
        if !path.is_file() {
            return Err(CliError::Install(format!(
                "installed binary missing: {}",
                path.display()
            )));
        }
        verify_executable_file(&path)?;
        verified.push(path.display().to_string());
    }
    let app_bundles = verify_installed_app_bundles(&apps_dir)?;
    for dir in [
        core.config.data_dir.clone(),
        core.config.workflows_dir(),
        core.config.index_dir(),
        core.config.memory_dir(),
        core.config.history_dir(),
        core.config.plugins_dir(),
    ] {
        if !dir.is_dir() {
            return Err(CliError::Install(format!(
                "storage directory missing: {}",
                dir.display()
            )));
        }
    }
    Ok(format!(
        "install verify PASS\nprefix={}\nbin_dir={}\napps_dir={}\nbinaries={}\napp_bundles={}\nstorage=PASS",
        prefix.display(),
        bin_dir.display(),
        apps_dir.display(),
        verified.len(),
        app_bundles
    ))
}

fn install_prefix(prefix: Option<&Path>) -> PathBuf {
    prefix.map(Path::to_path_buf).unwrap_or_else(|| {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".local")
    })
}

fn make_executable(path: &Path) -> Result<(), CliError> {
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(permissions.mode() | 0o755);
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

fn install_app_bundles(source_dir: &Path, apps_dir: &Path) -> Result<Vec<PathBuf>, CliError> {
    let mut installed = Vec::new();
    for spec in default_app_bundles() {
        let source = source_dir.join(spec.executable);
        let bundle = create_mac_app_bundle(apps_dir, &source, spec, env!("CARGO_PKG_VERSION"))?;
        make_executable(&bundle.join("Contents").join("MacOS").join(spec.executable))?;
        installed.push(bundle);
    }
    Ok(installed)
}

fn verify_installed_app_bundles(apps_dir: &Path) -> Result<usize, CliError> {
    let mut count = 0;
    for spec in default_app_bundles() {
        let bundle = apps_dir.join(format!("{}.app", spec.name));
        verify_app_bundle(&bundle)?;
        verify_executable_file(&bundle.join("Contents").join("MacOS").join(spec.executable))?;
        count += 1;
    }
    Ok(count)
}

fn verify_executable_file(path: &Path) -> Result<(), CliError> {
    #[cfg(unix)]
    {
        let mode = fs::metadata(path)?.permissions().mode();
        if mode & 0o111 == 0 {
            return Err(CliError::Install(format!(
                "installed binary is not executable: {}",
                path.display()
            )));
        }
    }
    Ok(())
}
