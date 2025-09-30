use dotenv::dotenv; // for token


fn main() 
{
    println!("Hello, world!");
    let c = "https://api.github.com/search/repositories?q=language:C/C++&sort=stars&order=desc&per_page=10";
    let java = "https://api.github.com/search/repositories?q=language:Java&sort=stars&order=desc&per_page=10";
    let rust = "https://api.github.com/search/repositories?q=language:Rust&sort=stars&order=desc&per_page=10";

    println!("C: {}", c);
    println!("Java: {}", java);
    println!("Rust: {}", rust);

}
