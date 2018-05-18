use regex::Regex;

// Aside: I'd really, really prefer to use the function name (not line
// number), but this path name, but this issue has been open for two
// years: https://github.com/rust-lang/rfcs/issues/1743
//
// Also, can't use `::` on Windows, so using `.` instead:
// https://msdn.microsoft.com/en-us/library/aa365247
//

// Convert a Rust module path into an acceptable file name (as a string)
pub fn filename_of_module_path_(module_path:&str) -> String {
    // Module paths (via module_path!()) in Rust contain the token
    // `::`, but we cannot use `::` on Windows for the names of files,
    // so we will use period (`.`) instead:
    //
    // https://msdn.microsoft.com/en-us/library/aa365247
    //
    let re = Regex::new(r"::").unwrap();
    format!("{}", re.replace_all(module_path, "."))
}

// A file name (as a string) from the current module path
#[macro_export]
macro_rules! filename_of_module_path {
    () => {{
        use util::filename_of_module_path_;
        filename_of_module_path_(module_path!())
    }}
}