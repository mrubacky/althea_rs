#[macro_use]
extern crate rouille;

extern crate debt_keeper;
extern crate rustc_serialize;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use rouille::Response;
use rouille::Request;
use rouille::input;

pub fn listen_for_key_exchange( tx: mpsc::Sender<String>) {
    #[derive(RustcDecodable)]
    struct Json {
        field1: String,
        field2: String
    }
    
    let mutex = Arc::new(Mutex::new(tx));
    thread::spawn(move ||{
        rouille::start_server("[::]:11498", move |request| {
            let json : Json = try_or_400!(input::json_input(request)); 
            mutex.lock().unwrap().send(json.field1).unwrap();
            Response::text("key")
        });
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
