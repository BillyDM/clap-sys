use crate::version::*;

use std::ffi::c_void;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host {
    pub clap_version: clap_version,
    pub host_data: *mut c_void,
    pub name: *const c_char,
    pub vendor: *const c_char,
    pub url: *const c_char,
    pub version: *const c_char,
    pub get_extension:
        unsafe extern "C" fn(host: *const clap_host, extension_id: *const c_char) -> *const c_void,
    pub request_restart: unsafe extern "C" fn(host: *const clap_host),
    pub request_process: unsafe extern "C" fn(host: *const clap_host),
    pub request_callback: unsafe extern "C" fn(host: *const clap_host),
}

unsafe impl Send for clap_host {}
unsafe impl Sync for clap_host {}
