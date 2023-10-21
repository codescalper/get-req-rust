use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("https://dummyjson.com/products/1")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {:?}", res.status());
    println!("Body: \n{:#?}", body);
    println!("Status: {:?}", res.headers());
    Ok(())
}
