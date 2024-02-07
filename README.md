# Scanny
Scanny is an asynchronous port scanner. All you need to do is specify an ip and it will return a list of all available ports. 

## Example Usage 
```
use scanny::Scanner;
#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".to_string();
    let ports = Scanner::scan(ip).await;
    println!("Current open ports {:?}"), ports;
}
```
