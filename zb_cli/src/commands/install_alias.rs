use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::init::is_writable;
use zb_core::Error;
use zb_io::validate_privileged_path;

pub fn execute(prefix: &Path, remove: bool) -> Result<(), Error> {
    let exe = env::current_exe().map_err(Error::file("cannot locate zb"))?;
    let bin_dir = exe.parent().ok_or_else(|| Error::FileError {
        message: "cannot locate zb's directory".into(),
    })?;
    let homebrew_prefix = get_homebrew_prefix();
    let mut candidates: Vec<PathBuf> = env::split_paths(&env::var_os("PATH").unwrap_or_default())
        .map(|dir| dir.join("brew"))
        .collect();
    if env::var_os("ZEROBREW_HOMEBREW_PREFIX").is_some() {
        candidates.push(homebrew_prefix.join("bin/brew"));
    } else {
        candidates.extend([
            PathBuf::from("/opt/homebrew/bin/brew"),
            PathBuf::from("/usr/local/bin/brew"),
            PathBuf::from("/usr/local/Homebrew"),
            PathBuf::from("/home/linuxbrew/.linuxbrew/bin/brew"),
        ]);
    }
    set_alias(bin_dir, prefix, &homebrew_prefix, remove, &candidates)
}

fn get_homebrew_prefix() -> PathBuf {
    if let Some(prefix) = env::var_os("ZEROBREW_HOMEBREW_PREFIX") {
        return PathBuf::from(prefix);
    }
    if cfg!(target_os = "macos") {
        PathBuf::from("/opt/homebrew")
    } else {
        PathBuf::from("/home/linuxbrew/.linuxbrew")
    }
}

fn set_alias(
    bin_dir: &Path,
    prefix: &Path,
    homebrew_prefix: &Path,
    remove: bool,
    candidates: &[PathBuf],
) -> Result<(), Error> {
    validate_privileged_path(prefix)?;
    validate_privileged_path(homebrew_prefix)?;
    let alias = bin_dir.join("brew");
    let companion = bin_dir.join("zb-brew");
    // A relative link remains removable even if zb-brew was deleted or moved.
    let owned = fs::read_link(&alias).is_ok_and(|target| target == Path::new("zb-brew"));
    let exists = match fs::symlink_metadata(&alias) {
        Ok(_) => true,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
        Err(e) => return Err(Error::file("cannot inspect brew entrypoint")(e)),
    };
    if exists && !owned {
        return Err(Error::ExecutionError {
            message: format!(
                "refusing to modify {}; it is not zerobrew's brew symlink",
                alias.display()
            ),
        });
    }

    let target_bin = prefix.join("bin");
    let prefix_alias = homebrew_prefix.join("bin");
    let link_prefix_bin = prefix_alias != target_bin;
    let mut owned_prefix = false;
    if link_prefix_bin {
        match fs::symlink_metadata(homebrew_prefix) {
            Ok(meta) if !meta.is_dir() => {
                return Err(Error::ExecutionError {
                    message: format!(
                        "refusing to modify {}; {} is not a directory",
                        prefix_alias.display(),
                        homebrew_prefix.display()
                    ),
                });
            }
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(Error::file("cannot inspect Homebrew prefix")(e)),
        }
        // A link pointing to <prefix>/bin remains removable even if <prefix>/bin was deleted.
        owned_prefix = fs::read_link(&prefix_alias).is_ok_and(|target| target == target_bin);
        let prefix_exists = match fs::symlink_metadata(&prefix_alias) {
            Ok(_) => true,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
            Err(e) => return Err(Error::file("cannot inspect Homebrew bin path")(e)),
        };
        if prefix_exists && !owned_prefix {
            return Err(Error::ExecutionError {
                message: format!(
                    "refusing to modify {}; it is not zerobrew's brew bin symlink",
                    prefix_alias.display()
                ),
            });
        }
    }

    if remove {
        if owned {
            fs::remove_file(&alias).map_err(Error::file("cannot remove brew symlink"))?;
        }
        if owned_prefix {
            fs::remove_file(&prefix_alias)
                .map_err(Error::file("cannot remove Homebrew bin symlink"))?;
        }
        println!("zerobrew's brew entrypoint is disabled.");
        return Ok(());
    }

    let companion_path = fs::canonicalize(&companion).map_err(|_| Error::ExecutionError {
        message: format!(
            "{} is missing; reinstall zerobrew to enable brew compatibility",
            companion.display()
        ),
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&companion).map_err(Error::file("cannot inspect zb-brew"))?;
        if !metadata.is_file() || metadata.permissions().mode() & 0o111 == 0 {
            return Err(Error::ExecutionError {
                message: format!("{} is not an executable file", companion.display()),
            });
        }
    }
    for candidate in candidates {
        if candidate == &alias && owned {
            continue;
        }
        match fs::symlink_metadata(candidate) {
            Ok(_) if fs::canonicalize(candidate).ok().as_ref() == Some(&companion_path) => {}
            Ok(_) => {
                return Err(Error::ExecutionError {
                    message: format!(
                        "refusing to enable brew compatibility: Homebrew or another brew entrypoint exists at {}",
                        candidate.display()
                    ),
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(Error::file("cannot check existing brew entrypoints")(e)),
        }
    }
    if link_prefix_bin && !owned_prefix {
        ensure_writable_homebrew_prefix(homebrew_prefix)?;
    }
    #[cfg(unix)]
    {
        if !owned {
            std::os::unix::fs::symlink("zb-brew", &alias)
                .map_err(Error::file("cannot create brew symlink"))?;
        }
        if link_prefix_bin && !owned_prefix {
            std::os::unix::fs::symlink(&target_bin, &prefix_alias)
                .map_err(Error::file("cannot create Homebrew bin symlink"))?;
        }
    }
    #[cfg(not(unix))]
    return Err(Error::ExecutionError {
        message: "brew compatibility symlinks require macOS or Linux".into(),
    });
    println!(
        "Enabled {} -> zb-brew (zerobrew compatibility).",
        alias.display()
    );
    if link_prefix_bin {
        println!(
            "Enabled {} -> {} (Homebrew bin path compatibility).",
            prefix_alias.display(),
            target_bin.display()
        );
    }
    println!(
        "Ensure {} is on PATH; remove with `zb install-alias brew --remove`.",
        bin_dir.display()
    );
    Ok(())
}

fn ensure_writable_homebrew_prefix(homebrew_prefix: &Path) -> Result<(), Error> {
    let need_sudo = if homebrew_prefix.exists() {
        !is_writable(homebrew_prefix)
    } else {
        homebrew_prefix
            .parent()
            .map(|p| p.exists() && !is_writable(p))
            .unwrap_or(true)
    };
    if !need_sudo {
        fs::create_dir_all(homebrew_prefix)
            .map_err(Error::file("cannot create Homebrew prefix directory"))?;
        return Ok(());
    }
    let status = Command::new("sudo")
        .args(["mkdir", "-p", &homebrew_prefix.to_string_lossy()])
        .status()
        .map_err(Error::file("failed to run sudo mkdir"))?;
    if !status.success() {
        return Err(Error::ExecutionError {
            message: format!("failed to create directory: {}", homebrew_prefix.display()),
        });
    }
    let user = Command::new("whoami")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .or_else(|| env::var("USER").ok())
        .ok_or_else(|| Error::ExecutionError {
            message: "could not determine current user (whoami failed and $USER not set)".into(),
        })?;
    let status = Command::new("sudo")
        .args(["chown", &user, &homebrew_prefix.to_string_lossy()])
        .status()
        .map_err(Error::file("failed to run sudo chown"))?;
    if !status.success() {
        return Err(Error::ExecutionError {
            message: format!("failed to set ownership on {}", homebrew_prefix.display()),
        });
    }
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::{PermissionsExt, symlink};

    fn companion(dir: &Path) {
        let path = dir.join("zb-brew");
        fs::write(&path, "test executable").unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    }

    #[test]
    fn alias_is_opt_in_idempotent_and_removable_when_stale() {
        let dir = tempfile::tempdir().unwrap();
        let bin_dir = dir.path().join("zb-bin");
        let prefix = dir.path().join("zerobrew");
        let homebrew = dir.path().join("homebrew");
        fs::create_dir_all(&bin_dir).unwrap();
        fs::create_dir_all(prefix.join("bin")).unwrap();
        fs::write(prefix.join("bin/jq"), "binary").unwrap();
        companion(&bin_dir);
        let alias = bin_dir.join("brew");
        assert!(!alias.exists());
        set_alias(
            &bin_dir,
            &prefix,
            &homebrew,
            false,
            std::slice::from_ref(&alias),
        )
        .unwrap();
        set_alias(
            &bin_dir,
            &prefix,
            &homebrew,
            false,
            std::slice::from_ref(&alias),
        )
        .unwrap();
        assert_eq!(fs::read_link(&alias).unwrap(), Path::new("zb-brew"));
        assert_eq!(
            fs::read_link(homebrew.join("bin")).unwrap(),
            prefix.join("bin")
        );
        assert!(homebrew.join("bin/jq").exists());
        fs::remove_file(bin_dir.join("zb-brew")).unwrap();
        fs::remove_dir_all(&prefix).unwrap();
        set_alias(&bin_dir, &prefix, &homebrew, true, &[]).unwrap();
        set_alias(&bin_dir, &prefix, &homebrew, true, &[]).unwrap();
        assert!(fs::symlink_metadata(alias).is_err());
        assert!(fs::symlink_metadata(homebrew.join("bin")).is_err());
    }

    #[test]
    fn refuses_existing_brew_in_path_or_standard_location() {
        let dir = tempfile::tempdir().unwrap();
        let bin_dir = dir.path().join("zb-bin");
        let prefix = dir.path().join("zerobrew");
        let homebrew = dir.path().join("homebrew");
        fs::create_dir_all(&bin_dir).unwrap();
        companion(&bin_dir);
        let other = tempfile::tempdir().unwrap();
        let brew = other.path().join("brew");
        fs::write(&brew, "Homebrew").unwrap();
        assert!(set_alias(&bin_dir, &prefix, &homebrew, false, &[brew]).is_err());
        assert!(!bin_dir.join("brew").exists());
        assert!(!homebrew.join("bin").exists());
    }

    #[test]
    fn never_replaces_or_removes_unknown_files_or_links() {
        let dir = tempfile::tempdir().unwrap();
        let bin_dir = dir.path().join("zb-bin");
        let prefix = dir.path().join("zerobrew");
        let homebrew = dir.path().join("homebrew");
        fs::create_dir_all(&bin_dir).unwrap();
        companion(&bin_dir);
        let alias = bin_dir.join("brew");
        fs::write(&alias, "Homebrew").unwrap();
        for remove in [false, true] {
            assert!(set_alias(&bin_dir, &prefix, &homebrew, remove, &[]).is_err());
            assert_eq!(fs::read_to_string(&alias).unwrap(), "Homebrew");
        }
        fs::remove_file(&alias).unwrap();
        symlink("missing-homebrew", &alias).unwrap();
        for remove in [false, true] {
            assert!(set_alias(&bin_dir, &prefix, &homebrew, remove, &[]).is_err());
            assert_eq!(
                fs::read_link(&alias).unwrap(),
                Path::new("missing-homebrew")
            );
        }
        fs::remove_file(&alias).unwrap();

        fs::create_dir_all(homebrew.join("bin")).unwrap();
        fs::write(homebrew.join("bin/existing"), "keep").unwrap();
        for remove in [false, true] {
            assert!(set_alias(&bin_dir, &prefix, &homebrew, remove, &[]).is_err());
            assert_eq!(
                fs::read_to_string(homebrew.join("bin/existing")).unwrap(),
                "keep"
            );
            assert!(fs::symlink_metadata(&alias).is_err());
        }
        fs::remove_dir_all(homebrew.join("bin")).unwrap();
        symlink(dir.path().join("other-bin"), homebrew.join("bin")).unwrap();
        for remove in [false, true] {
            assert!(set_alias(&bin_dir, &prefix, &homebrew, remove, &[]).is_err());
            assert_eq!(
                fs::read_link(homebrew.join("bin")).unwrap(),
                dir.path().join("other-bin")
            );
            assert!(fs::symlink_metadata(&alias).is_err());
        }
    }

    #[test]
    fn missing_companion_leaves_no_alias() {
        let dir = tempfile::tempdir().unwrap();
        let bin_dir = dir.path().join("zb-bin");
        let prefix = dir.path().join("zerobrew");
        let homebrew = dir.path().join("homebrew");
        fs::create_dir_all(&bin_dir).unwrap();
        assert!(set_alias(&bin_dir, &prefix, &homebrew, false, &[]).is_err());
        assert!(fs::symlink_metadata(bin_dir.join("brew")).is_err());
        assert!(fs::symlink_metadata(homebrew.join("bin")).is_err());
    }
}
