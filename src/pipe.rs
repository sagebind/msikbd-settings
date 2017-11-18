use libc;
use std::ffi::CString;
use std::fs::Permissions;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;


/// Create a named pipe in the file system.
pub fn create_named_pipe<P: Into<PathBuf>>(path: P, permissions: Permissions) -> io::Result<()> {
    unsafe {
        libc::umask(0);
    }

    let path = path.into();
    let path_c_str = CString::new(path.to_string_lossy().into_owned()).unwrap();
    let status = unsafe {
        libc::mkfifo(path_c_str.as_ptr(), permissions.mode())
    };

    match status {
        0 => Ok(()),
        _ => Err(io::Error::from_raw_os_error(status)),
    }
}
