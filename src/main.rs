extern crate json;
extern crate reqwest;

use std::io::Read;

fn main() {
    let mut json_body = String::new();
    let mut res = reqwest::get("https://api.coinmarketcap.com/v1/ticker/?limit=10").unwrap();

    assert!(res.status().is_success());

    res.read_to_string(&mut json_body);

    let parsed = json::parse(&json_body).unwrap();

    // members() is used to iterate over an array
    for item in parsed.members() {
        // need to first extract the string from the json value
        // then parse the string to float
        // finally unwrap the result
        let price:f32 = item["price_usd"].to_string().parse().unwrap();

        // a bit about string formatting, very useful
        // from https://doc.rust-lang.org/std/fmt/#syntax
        println!("{: >2} {: <17} {:10.4}", item["rank"], item["name"], price);
    }
}
