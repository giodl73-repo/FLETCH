use std::io::{Read, Write};
fn main() {
    let mut input = String::new();
    std::io::stdin()
        .take(3 * 1024 * 1024 + 1)
        .read_to_string(&mut input)
        .expect("UTF-8 request");
    let output = fletch_registry_inspector::inspect_registry(&input);
    std::io::stdout()
        .write_all(output.as_bytes())
        .expect("report");
}
