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
    GetQueuedCompletionStatus,
    INVALID_HANDLE_VALUE,
    OVERLAPPED,
};
#[cfg(target_os = "macos")]
use {
    crate::utils::timeout,
    libc::kevent,
    std::ptr::null,
};
use {
    crate::{
        mux::{
            Multiplexer,
            OsEvent,
        },
        syscall,
        utils::{
            AppResult,
            TIMEOUT,
        },
    },
    std::mem::MaybeUninit,
};

impl Multiplexer {
    pub(in crate::mux) fn poll(&self, events: &mut Vec<MaybeUninit<OsEvent>>) -> AppResult<i32> {
        #[cfg(target_os = "linux")]
        {
            syscall!(
                epoll_wait,
                self.file_descriptor,
                events.as_mut_ptr() as *mut epoll_event,
                events.len() as i32,
                TIMEOUT as i32,
            )
        }
        #[cfg(target_os = "macos")]
        {
            syscall!(
                kevent,
                self.file_descriptor,
                null(),
                0,
                events.as_mut_ptr() as *mut kevent,
                events.len() as i32,
                timeout(TIMEOUT)
            )
        }
        #[cfg(target_os = "windows")]
        {
            let mut bytes_transferred = 0;
            let mut completion_key = 0;
            let mut overlapped = null_mut();

            syscall!(
                GetQueuedCompletionStatus,
                self.file_descriptor,
                &mut bytes_transferred,
                &mut completion_key,
                &mut overlapped,
                TIMEOUT
            )
        }
    }
}
