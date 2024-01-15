//ahhhh the protocols to handle the BS I've built in `base`
//God help me

//there will be no structs declared here. It's just a series of procedures that deal will how to senders and recievers 
//will behave on startup and all


use crate::base::{Node, Master};

use serde_bytes;
use sanitize_filename::{self, sanitize_with_options, Options};
use std::{
    fs,
    io::{ErrorKind, Write}, prelude::*,
    time, net::IpAddr,
};

#[cfg(target_os = "windows")]
const USAGE: &str = r#"
raven.exe SEND ID FPATH
raven.exe UPGRADE 
raven.exe ADD IP_ADDR
raven.exe RM ID
"#;

const UNSAFE_WARNING: &str = r#"
This operation is UNSAFE! Perform this away from people and cameras, if possible
"#;

const WELCOME_MSG: &str = r#"
This is Raven, a secure means of sending data from one device to another without any third party interference
Except the one you and your friends/group/associates/company officially designate
Enjoy!
"#;


pub fn santize_fpath(file_path: &str) -> String{
    return sanitize_with_options(file_path, Options{windows: true, truncate: true, replacement: ""});
}

fn check_if_prev_present() -> bool {
    let tfhandle = match fs::File::open(base::APP_REGISTRY){
        Ok(_) => true,
        Err(err) => match err.kind(){
            ErrorKind::NotFound => install(),
            _ => {eprintln!("Error encountered: {err:?}"); false}
        }
    };
    return tfhandle;
}


fn install() -> bool{
    let thandle = fs::File::create(base::APP_REGISTRY).unwrap();
    //add other things here?
    //it's just to create the registry file 
    //maybe we can write an instant (i.e. moment of creation, and use that as a key for encypting the registry and other files)
    let epoch = time::Instant::now();
    return true;
}

fn connect_if_master(device_node: &Node, addr: &usize, ) -> (){
    let master = device_node.master;
    if let Some(var) = master{
        //if there is a master to be specified, call that master and handle transactions with it
        
    };
}

fn build_into_master(device_node: &Node) -> Master{
    todo!()
}

fn connect_if_ip(device_node: &Node, addr: &IpAddr) -> (){
    todo!()
}






