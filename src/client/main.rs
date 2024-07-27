fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.as_str() {
        "fetch" => print_fetch(),
        "bebra" => (),
        _ => (),
    }
}

fn print_fetch() {
    
}
