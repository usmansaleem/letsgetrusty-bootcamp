struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    // rule 3: lifetime of return ref is same as lifetime of self
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

fn main() {
    println!("Hello, world!");
    let mut tweet = Tweet { content: "example" };
    let old_content = tweet.replace_content("place_example");
    println!("{old_content}");
    println!("{}", tweet.content);
}

// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime parameter
// is assigned to all output parameters.
// 3. If there are multiple input lifetime parameters, but one of them is
// &self or &mut self, the lifetime of self is assigned to all
// output lifetime parameters.

fn take_return_content(content: &str) -> &str {
    content
}
