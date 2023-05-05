use crate::error::UnleashError::Win32Error;
use anyhow::Context as _;
use std::path::Path;
use windows::core::PCWSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE_DELAY_UNTIL_REBOOT};

fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub fn unleash(target: &Path) -> anyhow::Result<()> {
    let target = to_wstring(target.to_str().context("to_wstring")?);
    let result = unsafe { MoveFileExW(PCWSTR(target.as_ptr()), None, MOVEFILE_DELAY_UNTIL_REBOOT) };

    if result.as_bool() {
        Ok(())
    } else {
        let error = unsafe { GetLastError() };
        Err(Win32Error(error))?
    }
}

pub fn unleash_recursive(target: &Path) -> anyhow::Result<()> {
    let walk_dir = walkdir::WalkDir::new(target).contents_first(true);

    for it in walk_dir.into_iter() {
        let it = it?;
        unleash(it.path())?
    }

    Ok(())
}
