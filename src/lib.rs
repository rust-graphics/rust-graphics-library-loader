#![feature(thread_id_value)]

use std::ffi::{c_void, CString};
use std::mem::transmute_copy;
use std::ptr::null_mut;

extern crate rust_graphics_log;
use rust_graphics_log::log_f;

#[cfg(not(target_os = "windows"))]
extern crate libc;

#[cfg_attr(feature = "debug_derive", derive(Debug))]
pub struct Linker {
    #[cfg(not(target_os = "windows"))]
    link: *mut libc::c_void,
    #[cfg(target_os = "windows")]
    link: winapi::shared::minwindef::HMODULE,
    pub name: String,
}

impl Linker {
    #[cfg(not(target_os = "windows"))]
    pub fn new(library_name: &str) -> Option<Self> {
        let cs = CString::new(library_name).unwrap();
        let link = unsafe { libc::dlopen(cs.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
        if link == null_mut() {
            return None;
        }
        Some(Self {
            link,
            name: library_name.to_string(),
        })
    }

    #[cfg(target_os = "windows")]
    pub fn new(library_name: &str) -> Option<Self> {
        use std::iter::once;
        let cs: Vec<u16> = library_name.encode_utf16().chain(once(0)).collect();
        let link = unsafe { winapi::um::libloaderapi::LoadLibraryW(cs.as_ptr()) };
        if link == null_mut() {
            return None;
        }
        Some(Self {
            link,
            name: library_name.to_string(),
        })
    }

    #[cfg(not(target_os = "windows"))]
    fn get_fun_ptr(&self, name: &str) -> *mut c_void {
        let cs = CString::new(name).unwrap();
        unsafe { libc::dlsym(self.link, cs.as_ptr()) }
    }

    #[cfg(target_os = "windows")]
    fn get_fun_ptr(&self, name: &str) -> *mut c_void {
        let cs = CString::new(name).unwrap();
        unsafe {
            transmute_copy(&winapi::um::libloaderapi::GetProcAddress(
                self.link,
                cs.as_ptr(),
            ))
        }
    }

    pub fn get_function<F>(&self, name: &str) -> Option<F>
    where
        F: Sized,
    {
        let f = self.get_fun_ptr(name);
        if f == null_mut() {
            return None;
        } else {
            unsafe {
                return Some(transmute_copy(&f));
            }
        }
    }
}

impl Drop for Linker {
    #[cfg(target_os = "windows")]
    fn drop(&mut self) {
        if winapi::shared::minwindef::FALSE
            == unsafe { winapi::um::libloaderapi::FreeLibrary(self.link) }
        {
            log_f!("Can not free library '{}'.", self.name);
        }
    }
    #[cfg(not(target_os = "windows"))]
    fn drop(&mut self) {
        if 0 != unsafe { libc::dlclose(self.link) } {
            log_f!("Can not free library '{}'.", self.name);
        }
    }
}
