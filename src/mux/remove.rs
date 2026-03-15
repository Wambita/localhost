#[cfg(target_os = "linux")]
use libc::{
    epoll_event,
    EPOLLET,
    EPOLLIN,
    EPOLL_CTL_ADD,
    EPOLL_CTL_DEL,
};
#[cfg(target_os = "windows")]
use windows::Win32::System::IO::{
    CreateIoCompletionPort,
    GetQueuedCompletionStatus,
    INVALID_HANDLE_VALUE,
    OVERLAPPED,
};
use {
    crate::{
        mux::Multiplexer,
        syscall,
        utils::AppResult,
    },
    std::{
        ffi::c_void,
        net::Shutdown,
        os::fd::RawFd,
        ptr::null_mut,
    },
};
#[cfg(target_os = "macos")]
use {
    libc::{
        kevent,
        EVFILT_READ,
    },
    std::ptr::null,
};

impl Multiplexer {
    pub(in crate::mux) fn remove(&mut self, fd: RawFd) -> AppResult<i32> {
        if let Some(client) = self.streams.remove(&fd) {
            client
                .stream
                .shutdown(Shutdown::Both)?;
        };

        #[cfg(target_os = "linux")]
        {
            (syscall!(
                epoll_ctl,
                self.file_descriptor,
                EPOLL_CTL_DEL,
                fd,
                core::ptr::null_mut()
            ))
        }
        #[cfg(target_os = "macos")]
        {
            let mut event = kevent {
                ident:  fd as usize,
                filter: EVFILT_READ,
                flags:  libc::EV_DELETE,
                fflags: 0,
                data:   0,
                udata:  null_mut::<c_void>(),
            };
            syscall!(
                kevent,
                self.file_descriptor,
                &mut event,
                1,
                null_mut(),
                0,
                null()
            )
        }
        #[cfg(target_os = "windows")]
        {
            syscall!(CloseHandle, fd as HANDLE)
        }
    }
}
