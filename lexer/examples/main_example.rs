use dozen_lexer::{ run };

fn main() {
    let tokens = run("./examples/main_example.dzn");
    println!("{:?}", tokens);
}