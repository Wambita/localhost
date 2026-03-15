#[macro_export]
macro_rules! check {
    ($result:expr) => {{
        if $result == -1 {
            Err($crate::utils::AppErr::last_os_error())
        }
        else {
            Ok($result)
        }
    }};
}

#[macro_export]
#[allow(clippy::macro_metavars_in_unsafe)]
macro_rules! syscall {
    ($name:ident $(, $arg:expr)* $(,)?) => {{
        let func = libc::$name;
        let result = unsafe { func($($arg),*) };
        $crate::check!(result)
    }};
}

#[macro_export]
macro_rules! debug {
    ($val:expr) => {{
        let now = chrono::Local::now();
        eprintln!(
            "{} -  [{}:{}:{}]  -   {:?}",
            now.format("%Y|%m|%d - %H:%M:%S"),
            file!(),
            line!(),
            column!(),
            $val
        );
        $val
    }};
}
