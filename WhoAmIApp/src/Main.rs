use whoami;

fn main() {
    println!("Welcome to my Rust console app!");

    // Get the current username
    let username = whoami::username();

    // Display the username
    println!("Hello, {}!", username);

    // Wait for user input before exiting
    println!("Press Enter to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}