#![allow(dead_code)]
#![allow(unused_variables)]
const USAGE: &str = r#"
.\nkrypt.exe
.\nkrypt.exe stop
.\nkrypt.exe transfer FilePATH IpAddress 
"#;

const HELP: &str = r#"
transfer sends a file from your pc to an external system.
Note that the external system has to be running nkrypt
The naked call will bind to a port on your PC until you manually terminate the process by running nkrypt.exe stop
"#;

static mut LISTEN_FLAG_RUN_OPTION: Option<bool> = None;

fn main(){


}





// pub fn match_args() -> bool{
//    //put the nested matches here and
// }

