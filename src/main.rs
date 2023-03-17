use error_chain::error_chain;
use std::io::Read;

error_chain!{
  foreign_links{
    Io(std::io::Error);
    HttpRequest(reqwest::Error);
  }
}




   fn main() -> Result<()> { 
    let mut res = reqwest::blocking::get("https://www.futa.edu.ng")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("status {}",res.status());
    println!("headers \n{:#?}", res.headers());
    println!("body: \n{}", body);
    Ok(())
}
