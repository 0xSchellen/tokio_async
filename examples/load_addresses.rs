extern crate redis;
use redis::Commands;

fn get_an_address(address: &str) -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    con.get(address)
}

fn set_an_address(address: &str) -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _ : () = con.set("0x00000000219ab540356cbb839cbe05303d7705fa", 13819783)?;
    con.get(address)
}

fn main() {
    let address: &str = "0x00000000219ab540356cbb839cbe05303d7705fa";
    set_an_address(&address);   
    let result: &str = get_an_address(&address);
}