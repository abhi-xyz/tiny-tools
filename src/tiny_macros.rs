use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Log<F, L> {
    pub file: F,      // Source file where the log is generated
    pub line: L,      // Line number where the log is generated
    pub time: String, // Timestamp for the log entry
    pub msg: String,  // Log message
}

// Macro to create and write log entries
#[macro_export]
macro_rules! json_logger {
    ($msg:expr) => {{
        use chrono::Local;
        use std::fs::OpenOptions;
        use std::io::Write;

        // Get the current timestamp
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Create a log entry
        let log_entry = $crate::tiny_macros::Log {
            file: file!(),
            line: line!(),
            time: timestamp,
            msg: $msg.to_string(),
        };

        // Serialize the log entry to JSON
        let json_log = serde_json::to_string_pretty(&log_entry).unwrap();

        let bin_name = env!("CARGO_PKG_NAME");
        let log_dir = dirs::cache_dir()
            .unwrap_or(dirs::data_dir().unwrap())
            .join(bin_name);
        if !log_dir.exists() {
            std::fs::create_dir_all(&log_dir).unwrap();
        }
        let log_file = &log_dir.join("log.json");
        if !log_file.exists() {
            std::fs::write(log_file, "").unwrap();
        }

        // Define the log file path
        let log_file_path = log_file; // Customize the log file path as needed

        // Open the log file in append mode
        let mut file = OpenOptions::new()
            .create(true) // Create the file if it doesn't exist
            .append(true) // Append to the file if it exists
            .open(log_file_path)
            .unwrap(); // Handle errors appropriately

        // Write the JSON-formatted log entry to the file
        writeln!(file, "{}", json_log).unwrap(); // Handle errors appropriately
    }};
}

#[macro_export]
#[cfg(feature = "debug")]
macro_rules! dprintln {
    ($($arg:tt)*) => (println!($($arg)*));
}

#[macro_export]
#[cfg(not(feature = "debug"))]
macro_rules! dprintln {
    ($($arg:tt)*) => {};
}

#[macro_export]
#[cfg(feature = "debug")]
macro_rules! dprint {
    ($($arg:tt)*) => (print!($($arg)*));
}

#[macro_export]
#[cfg(not(feature = "debug"))]
macro_rules! dprint {
    ($($arg:tt)*) => {};
}

/// # verbose
#[macro_export]
macro_rules! verbose {
    ($verbose_flag:expr, $($arg:tt)*) => {
        if $verbose_flag {
        println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! dev_debug {
($verbose_flag:expr, $($arg:tt)*) => {
    if $verbose_flag {
    println!("[{}:{}] {}", file!(), line!(), format!($($arg)*));
    }
};
}

/// Show an error to stderr in a similar style to GNU coreutils.
///
/// Takes a [`format!`]-like input and prints it to stderr. The output is
/// prepended with the current utility's name.
#[macro_export]
macro_rules! show_error(
    ($($args:tt)+) => ({
        eprint!("{}: ", env!("CARGO_PKG_NAME"));
        eprintln!($($args)+);
    })
);

// Prompt the user with a formatted string and returns `true` if they reply `'y'` or `'Y'`
//
// This macro functions accepts the same syntax as `format!`. The prompt is written to
// `stderr`. A space is also printed at the end for nice spacing between the prompt and
// the user input. Any input starting with `'y'` or `'Y'` is interpreted as `yes`.
//#[macro_export]
//macro_rules! prompt_yes(
//    ($($args:tt)+) => ({
//        use std::io::Write;
//        eprint!("roxide: ");
//        eprint!($($args)+);
//        eprint!(" ");
//    //    uucore::crash_if_err!(1, std::io::stderr().flush());
//        uucore::read_yes()
//    })
//);

/// Macro to prompt the user with a message and collect input.
/// Returns `true` if the input is "yes" or "y" (case-insensitive), otherwise `false`.
///
/// Example usage:
/// ```
/// if prompt_yes!("Do you want to continue? (yes/y):") {
///     println!("Continuing...");
/// } else {
///     println!("Exiting...");
/// }
/// ```
#[macro_export]
macro_rules! prompt_yes {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        // Print the prompt and flush stdout
        print!("{}: ", env!("CARGO_PKG_NAME"));
        print!($($arg)*);
        print!(" "); // Add a space after the prompt
        io::stdout().flush().unwrap();

        // Read input from stdin
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim and check for "yes" or "y" (case-insensitive)
        matches!(input.trim().to_lowercase().as_str(), "yes" | "y")
    }};
}
