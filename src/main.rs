fn main() {
    let file = std::fs::read_to_string("./src/file.txt").expect("Error: file couldnt be found");
    println!("file: \n{file}");

    println!("Hello, world!");
}
