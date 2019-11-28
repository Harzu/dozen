use simple_lexer::{ from_file };

fn main() {
    let tokens = from_file("./examples/main_example.dzn");
    println!("{:?}", tokens);
}