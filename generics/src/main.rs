struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    println!("Hello, world!");
    let cmd1 = BrowserCommand {
        name: String::from("Navigate"),
        payload: String::from("https://usmans.info"),
    };

    let cmd2 = BrowserCommand {
        name: "zoom".to_owned(),
        payload: 200,
    };

    let cmd3 = BrowserCommand::new("Navigate 2".to_owned(), 500);

    cmd1.print_payload();

    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();
}

fn serialize_payload<T>(payload: T) -> String {
    "placeholder".to_string()
}
