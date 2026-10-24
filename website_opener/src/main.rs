
use std::io::{self, Write};
use std::process::Command;

fn main() {
    // Ask the user to enter a website URL.
    // print! does not automatically add a newline.
    print!("Please enter your website URL: ");

    // Flush the output so the prompt appears before we read input.
    io::stdout()
        .flush()
        .expect("Failed to flush output");

    // Create a mutable, empty String to store the user's input.
    // 'mut' means the variable's value can be changed.
    let mut url = String::new();

    // Read one line of input from the keyboard.
    // read_line() appends the input to the String.
    // expect() displays an error message if reading fails.
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read input");

    // Remove leading and trailing whitespace, such as the newline.
    // to_string() creates an owned String we can modify.
    let mut url = url.trim().to_string();

    // Check whether the URL already starts with HTTP or HTTPS.
    // If neither prefix exists, add HTTPS to the beginning.
    if !url.starts_with("https://") && !url.starts_with("http://") {
        url = format!("https://{}", url);
    }

    // Remove the HTTPS or HTTP prefix to inspect the hostname.
    // or_else() tries the second option if the first returns None.
    let valid = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .map(|host| {
            // Ignore the path after the hostname.
            let host = host.split('/').next().unwrap_or("");

            // Perform a basic check for a dotted hostname.
            // This is not a complete URL validation method.
            host.contains('.')
                && !host.starts_with('.')
                && !host.ends_with('.')
        })
        // If no HTTP prefix was found, treat the URL as invalid.
        .unwrap_or(false);

    // Choose what to do based on the validation result.
    if valid {
        println!("Opening your website...");

        // macOS uses the 'open' command to launch the default browser.
        #[cfg(target_os = "macos")]
        let result = Command::new("open").arg(&url).status();

        // Linux commonly uses 'xdg-open' to launch the default browser.
        #[cfg(target_os = "linux")]
        let result = Command::new("xdg-open").arg(&url).status();

        // Windows uses 'start' through the Command Prompt.
        // The empty string provides the window title argument.
        #[cfg(target_os = "windows")]
        let result = Command::new("cmd")
            .args(["/C", "start", "", &url])
            .status();

        // Check whether the browser-launch command could be executed.
        match result {
            // The command started. This does not guarantee the page loaded.
            Ok(_) => println!("Browser launch requested."),

            // Display the operating-system error if launching failed.
            Err(error) => println!("Could not open browser: {}", error),
        }
    } else {
        // This branch runs if the basic hostname check fails.
        println!("Invalid URL. Please try again.");
    }
}
