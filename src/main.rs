use inquire::Confirm;

fn main() {
    let repo_answer = Confirm::new("Update repository mention?")
        .with_default(true)
        .with_help_message("Update the remote URL for this repositry.")
        .prompt();

    

    match repo_answer {
        Ok(true) => println!("All done"),
        Ok(false) => println!("Nothing updated. All wrong preserved"),
        Err(_) => println!("Error with questionnaire, try again later"),
    }
}
