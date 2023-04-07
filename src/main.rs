use inquire::Confirm;
use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {:?}", path.display());
    let help_message = format!(
        "Update the remote URL for this repository {:?}.",
        path.display()
    );
    // let help_message = format!("Update the remote URL for this repository {}.", path);
    let repo_answer = Confirm::new("Update repository mention?")
        .with_default(true)
        .with_help_message(&help_message)
        .prompt();

    match repo_answer {
        Ok(true) => println!("All done"),
        Ok(false) => println!("Nothing updated. All wrong preserved"),
        Err(_) => println!("Error with questionnaire, try again later"),
    }
}
