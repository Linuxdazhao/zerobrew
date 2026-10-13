use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use zb_core::Error;

pub fn execute(remove: bool) -> Result<(), Error> {
    let exe = env::current_exe().map_err(Error::file("cannot locate zb"))?;
    let bin_dir = exe.parent().ok_or_else(|| Error::FileError {
        message: "cannot locate zb's directory".into(),
    })?;
    let mut candidates: Vec<PathBuf> = env::split_paths(&env::var_os("PATH").unwrap_or_default())
        .map(|dir| dir.join("brew"))
        .collect();
    candidates.extend([
        PathBuf::from("/opt/homebrew/bin/brew"),
        PathBuf::from("/usr/local/bin/brew"),
        PathBuf::from("/usr/local/Homebrew"),
        PathBuf::from("/home/linuxbrew/.linuxbrew/bin/brew"),
    ]);
    set_alias(bin_dir, remove, &candidates)
}

fn set_alias(bin_dir: &Path, remove: bool, candidates: &[PathBuf]) -> Result<(), Error> {
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
    if remove {
        if owned {
            fs::remove_file(&alias).map_err(Error::file("cannot remove brew symlink"))?;
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
    #[cfg(unix)]
    if !owned {
        std::os::unix::fs::symlink("zb-brew", &alias)
            .map_err(Error::file("cannot create brew symlink"))?;
    }
    #[cfg(not(unix))]
    return Err(Error::ExecutionError {
        message: "brew compatibility symlinks require macOS or Linux".into(),
    });
    println!(
        "Enabled {} -> zb-brew (zerobrew compatibility).",
        alias.display()
    );
    println!(
        "Ensure {} is on PATH; remove with `zb install-alias brew --remove`.",
        bin_dir.display()
    );
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
        companion(dir.path());
        let alias = dir.path().join("brew");
        assert!(!alias.exists());
        set_alias(dir.path(), false, std::slice::from_ref(&alias)).unwrap();
        set_alias(dir.path(), false, std::slice::from_ref(&alias)).unwrap();
        assert_eq!(fs::read_link(&alias).unwrap(), Path::new("zb-brew"));
        fs::remove_file(dir.path().join("zb-brew")).unwrap();
        set_alias(dir.path(), true, &[]).unwrap();
        set_alias(dir.path(), true, &[]).unwrap();
        assert!(fs::symlink_metadata(alias).is_err());
    }

    #[test]
    fn refuses_existing_brew_in_path_or_standard_location() {
        let dir = tempfile::tempdir().unwrap();
        companion(dir.path());
        let other = tempfile::tempdir().unwrap();
        let brew = other.path().join("brew");
        fs::write(&brew, "Homebrew").unwrap();
        assert!(set_alias(dir.path(), false, &[brew]).is_err());
        assert!(!dir.path().join("brew").exists());
    }

    #[test]
    fn never_replaces_or_removes_unknown_files_or_links() {
        let dir = tempfile::tempdir().unwrap();
        companion(dir.path());
        let alias = dir.path().join("brew");
        fs::write(&alias, "Homebrew").unwrap();
        for remove in [false, true] {
            assert!(set_alias(dir.path(), remove, &[]).is_err());
            assert_eq!(fs::read_to_string(&alias).unwrap(), "Homebrew");
        }
        fs::remove_file(&alias).unwrap();
        symlink("missing-homebrew", &alias).unwrap();
        for remove in [false, true] {
            assert!(set_alias(dir.path(), remove, &[]).is_err());
            assert_eq!(
                fs::read_link(&alias).unwrap(),
                Path::new("missing-homebrew")
            );
        }
    }

    #[test]
    fn missing_companion_leaves_no_alias() {
        let dir = tempfile::tempdir().unwrap();
        assert!(set_alias(dir.path(), false, &[]).is_err());
        assert!(fs::symlink_metadata(dir.path().join("brew")).is_err());
    }
}
