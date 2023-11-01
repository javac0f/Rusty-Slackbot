#[allow(unused_variables)]
#[allow(unused_imports)]


// EXTERNAL SOURCES
extern crate slack;
extern crate RustySlackbot;


use Rusty_Slackbot::handler;
use slack::RtmClient;
use std::{env, process};


fn main() {
    let key:String = api_key();
    let mut handler = handler::Handler;
    




    println!("Hello, world!");
}


fn api_key() -> String {
    match env::var("SLACK_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            println!("Required the SLACK_API_TOKEN environment variable");
            process::exit(1);
        }
    }
}