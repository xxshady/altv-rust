#[cfg(windows)]
use io_extras::os::windows::{AsRawHandleOrSocket, RawHandleOrSocket};

use std::os::windows::prelude::{AsHandle, BorrowedHandle};
use wasmtime_wasi::Error;
use wasi_common::{
    file::{FdFlags, FileType},
    ErrorExt,
};

fn log_unimplemented_error() {
    unsafe {
        altv_sdk::helpers::log_error(
            "Default stdout and stderr are not implemented, use altv::dbg or altv::log instead",
        );
    }
}

macro_rules! impl_wasi_file {
    ($struct:ty) => {
        #[async_trait::async_trait]
        impl wasmtime_wasi::WasiFile for $struct {
            fn as_any(&self) -> &dyn std::any::Any {
                log_unimplemented_error();
                self
            }
            #[cfg(unix)]
            fn pollable(&self) -> Option<rustix::fd::BorrowedFd> {
                log_unimplemented_error();
                None
            }
            #[cfg(windows)]
            fn pollable(&self) -> Option<io_extras::os::windows::RawHandleOrSocket> {
                log_unimplemented_error();
                None
            }
            async fn get_filetype(&self) -> Result<FileType, Error> {
                log_unimplemented_error();
                Err(Error::not_supported())
            }
            async fn get_fdflags(&self) -> Result<FdFlags, Error> {
                log_unimplemented_error();
                Err(Error::not_supported())
            }
            async fn write_vectored<'a>(
                &self,
                _bufs: &[std::io::IoSlice<'a>],
            ) -> Result<u64, Error> {
                log_unimplemented_error();
                Err(Error::not_supported())
            }
            async fn write_vectored_at<'a>(
                &self,
                _bufs: &[std::io::IoSlice<'a>],
                _offset: u64,
            ) -> Result<u64, Error> {
                log_unimplemented_error();
                Err(Error::not_supported())
            }
            async fn seek(&self, _pos: std::io::SeekFrom) -> Result<u64, Error> {
                log_unimplemented_error();
                Err(Error::not_supported())
            }
            async fn set_times(
                &self,
                _atime: Option<wasi_common::SystemTimeSpec>,
                _mtime: Option<wasi_common::SystemTimeSpec>,
            ) -> Result<(), Error> {
                log_unimplemented_error();
                Err(Error::not_supported())
            }
            fn isatty(&self) -> bool {
                log_unimplemented_error();
                false
            }
        }

        #[cfg(windows)]
        impl AsHandle for $struct {
            fn as_handle(&self) -> BorrowedHandle<'_> {
                log_unimplemented_error();
                self.0.as_handle()
            }
        }

        #[cfg(windows)]
        impl AsRawHandleOrSocket for $struct {
            #[inline]
            fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
                log_unimplemented_error();
                self.0.as_raw_handle_or_socket()
            }
        }
    };
}

pub struct UnimplementedStdout(pub std::io::Stdout);
impl_wasi_file!(UnimplementedStdout);

pub struct UnimplementedStderr(pub std::io::Stderr);
impl_wasi_file!(UnimplementedStderr);
