use crate::utils::{DEFAULT_EXECUTABLES, is_installed};
pub fn check_python() -> Option<String> {
    is_installed(DEFAULT_EXECUTABLES.python)
}
