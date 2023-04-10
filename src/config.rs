pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(windows)]
pub const CLI_NAME: &str = "whop.exe";
#[cfg(not(windows))]
pub const CLI_NAME: &str = "whop";