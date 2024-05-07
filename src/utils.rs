use std::{fs::DirEntry, os::unix::fs::PermissionsExt};

use miette::*;

pub fn permissions_to_string(permissions: u32) -> String {
    let mut result = String::with_capacity(6);
    let mask = [0b100, 0b010, 0b001]; // Mask for checking read, write, and execute permissions

    for &m in &mask {
        if permissions & m != 0 {
            result.push('r');
        } else {
            result.push('-');
        }

        if permissions & (m << 3) != 0 {
            result.push('w');
        } else {
            result.push('-');
        }

        if permissions & (m << 6) != 0 {
            result.push('x');
        } else {
            result.push('-');
        }
    }

    result
}

/// # Permissions to Machine String
/// ```text
/// perm-fact    = "Perm" "=" *pvals
/// pvals        = "a" / "c" / "d" / "e" / "f" /
///                "l" / "m" / "p" / "r" / "w"
/// ```
/// Checks for the permissions of a file and returns a string representation of the permissions
/// If the file is a directory returns the appropiate permissions
///
/// Check: https://datatracker.ietf.org/doc/html/rfc3659#section-7.5.5
pub fn permissions_to_machine_string(entry: &DirEntry) -> Result<String> {
    let metadata = entry.metadata().into_diagnostic()?;
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    let mask = [0b100, 0b010, 0b001]; // Mask for checking read, write, and execute permissions
    let mut result = String::with_capacity(9);

    if metadata.is_dir() {
        if mode & 0o100 != 0 {
            result.push('e');
        }
        if mode & 0o200 != 0 {
            result.push('l');
        }
        if mode & 0o400 != 0 {
            result.push('a');
        }
        if mode & 0o1000 != 0 {
            result.push('c');
        }
        return Ok(result);
    }

    for &m in &mask {
        if mode & m != 0 {
            result.push('r');
        } else if mode & (m << 3) != 0 {
            result.push('w');
        }
    }
    Ok(result)
}
