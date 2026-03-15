#[cfg(target_os = "linux")]
use libc::{
    epoll_event,
    EPOLLET,
    EPOLLIN,
    EPOLL_CTL_ADD,
};
#[cfg(target_os = "windows")]
use windows::Win32::System::IO::{
    CreateIoCompletionPort,
    INVALID_HANDLE_VALUE,
};
use {
    crate::{
        debug,
        mux::Multiplexer,
        syscall,
        utils::AppResult,
    },
    std::os::fd::RawFd,
};
#[cfg(target_os = "macos")]
use {
    libc::{
        kevent,
        EVFILT_READ,
        EV_ADD,
    },
    std::{
        ffi::c_void,
        ptr::{
            null,
            null_mut,
        },
    },
};

impl Multiplexer {
    pub(in crate::mux) fn add(&self, fd: RawFd) -> AppResult<i32> {
        #[cfg(target_os = "linux")]
        {
            let mut event = epoll_event {
                events: EPOLLIN as u32 | EPOLLET as u32,
                u64:    fd as u64,
            };

            syscall!(
                epoll_ctl,
                self.file_descriptor,
                EPOLL_CTL_ADD,
                fd,
                &mut event
            )
            .inspect_err(|e| {
                debug!(e);
            })
        }
        #[cfg(target_os = "macos")]
        {
            let event = kevent {
                ident:  fd as usize,
                filter: EVFILT_READ,
                flags:  EV_ADD,
                fflags: 0,
                data:   0,
                udata:  null_mut::<c_void>(),
            };

            syscall!(
                kevent,
                self.file_descriptor,
                &event,
                1,
                null_mut(),
                0,
                null(),
            )
            .inspect_err(|e| {
                debug!(e);
            })
        }
        #[cfg(target_os = "windows")]
        {
            syscall!(
                CreateIoCompletionPort,
                fd as HANDLE,
                self.file_descriptor,
                0,
                0
            )
            .inspect_err(|e| {
                debug!(e);
            })
        }
    }
}
