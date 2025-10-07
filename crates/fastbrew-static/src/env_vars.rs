pub struct EnvVars;

impl EnvVars {
    /// The path to the binary that was used to invoke fastbrew.
    ///
    /// If the executable was invoked through a symbolic link, some platforms will return the path
    /// of the symbolic link and other platforms will return the path of the symbolic link’s target.
    ///
    /// See <https://doc.rust-lang.org/std/env/fn.current_exe.html#security> for security
    /// considerations.
    pub const FASTBREW: &'static str = "FASTBREW";
}
