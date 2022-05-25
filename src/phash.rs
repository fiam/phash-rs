use std::ffi::{c_void, CString};
use std::io::{Error, ErrorKind, Result, Write};
use std::os::raw::{c_char, c_float, c_int, c_uchar, c_ulonglong};
use std::path::Path;
use std::slice;

use tempfile::NamedTempFile;

extern "C" {
    fn free(ptr: *const c_void);
    fn ph_dct_imagehash_c(filename: *const c_char, hash: *mut c_ulonglong) -> c_int;
    fn ph_mh_imagehash_c(
        filename: *const c_char,
        N: *mut c_int,
        alpha: c_float,
        lvl: c_float,
    ) -> *mut c_uchar;

}

#[cfg(unix)]
fn path_to_cstring<P: AsRef<Path>>(path: P) -> CString {
    use std::os::unix::ffi::OsStrExt;
    CString::new(path.as_ref().as_os_str().as_bytes()).unwrap()
}

#[cfg(not(unix))]
fn path_to_cstring<P: AsRef<Path>>(path: P) -> Vec<u8> {
    CString::new(path.as_ref().to_string_lossy().to_string().into_bytes()).unwrap()
}

pub fn dct_imagehash<P: AsRef<Path>>(path: P) -> Result<u64> {
    let mut hash: c_ulonglong = 0;
    let c_str = path_to_cstring(path);
    let c_ptr: *const c_char = c_str.as_ptr() as *const c_char;
    unsafe {
        if ph_dct_imagehash_c(c_ptr, &mut hash) < 0 {
            Err(Error::new(ErrorKind::NotFound, "could not load image"))
        } else {
            Ok(hash as u64)
        }
    }
}

pub fn dct_imagehash_from_memory(data: &[u8]) -> Result<u64> {
    let mut file = NamedTempFile::new()?;
    file.write_all(data)?;
    file.as_file().sync_all()?;
    let path = file.into_temp_path();
    let result = dct_imagehash(&path);
    path.close()?;
    result
}

pub struct MHImageHashArgs {
    pub alpha: f32,
    pub level: f32
}

impl Default for MHImageHashArgs {
    fn default() -> Self {
        Self {
            alpha: 2.0,
            level: 1.0,
        }
    }
}

pub fn mh_imagehash<P: AsRef<Path>>(path: P, args: &MHImageHashArgs) -> Result<Vec<u8>> {
    let mut n: c_int = 0;
    let c_str = path_to_cstring(path);
    let c_ptr: *const c_char = c_str.as_ptr() as *const c_char;
    unsafe {
        let result = ph_mh_imagehash_c(c_ptr, &mut n, args.alpha as c_float, args.level as c_float);
        if result.is_null() {
            Err(Error::new(ErrorKind::NotFound, "could not load image"))
        } else {
            let slice = slice::from_raw_parts(result, n as usize);
            let mut v = vec![];
            v.extend_from_slice(slice);
            free(result as *const c_void);
            Ok(v)
        }
    }
}

pub fn mh_imagehash_from_memory(data: &[u8], args: &MHImageHashArgs) -> Result<Vec<u8>> {
    let mut file = NamedTempFile::new()?;
    file.write_all(data)?;
    file.as_file().sync_all()?;
    let path = file.into_temp_path();
    let result = mh_imagehash(&path, args);
    path.close()?;
    result
}
