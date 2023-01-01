use std::fs::File;

fn main() {
    let file = File::open("example.txt");
    if let Ok(ok_file) = file {
        println!("File opened");
    } else {
        panic!("File cannot be opened");
    }

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Failed to open file {:?}", error)
    //     }
    // };
}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username cannot be empty".to_string())
    } else {
        Ok(1)
    }
}
