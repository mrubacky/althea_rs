
extern crate rouille;

use rouille;
use std::thread;
use std::sync::{Arc};
use std::sync::mpsc;

pub fn listen_for_key_exchange<T>(&tx: mpsc::Sender<T>){
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
