pub struct Statement {
    
}


/// Ask for input from stdin
/// and return the input as String
pub fn accept_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
