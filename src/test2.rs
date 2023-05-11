use std::io;

fn main() {
    println!("Start Recording.");

    let mut result: Vec<String> = vec![];
    loop {
        let prompt = get_prompt();

        match prompt {
            Some(msg) => {
                if msg == "n" || msg == "no" {
                    preview(&result);
                    break;
                }
                println!("Done. Continue? (n / no to stop, any other key to add)");
                result.push(msg);
            }
            None => println!("Invalid Input"),
        }
    }
}

fn get_prompt() -> Option<String> {
    let mut prompt = String::new();

    match io::stdin().read_line(&mut prompt) {
        Ok(_) => Some(prompt.trim().to_string()),
        Err(_) => None,
    }
}

fn preview(result: &Vec<String>) {
    println!("Preview: {:#?}", result);
}
