#[macro_export]
macro_rules! match_target {
    ($macos_arm:expr, $macos_x:expr, $linux_x:expr, $windows_x:expr, $default:expr) => {
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        return $macos_arm;

        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        return $macos_x;

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        return $linux_x;

        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        return $windows_x;

        #[cfg(not(any(
            all(target_os = "macos", target_arch = "aarch64"),
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "linux", target_arch = "x86_64"),
            all(target_os = "windows", target_arch = "x86_64")
        )))]
        $default
    };
}
