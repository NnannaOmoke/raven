use std::{
    env,
    fs::OpenOptions,
    io::{self, BufReader, BufWriter, ErrorKind, Read, Write}, 
    net::{ IpAddr, SocketAddr, TcpListener, TcpStream}, 
    process::{Command, Stdio}, 
    thread, 
    time::Duration
};

const SEND_ATTEMPTS: usize = 3;

pub fn try_send(raw_ip: &IpAddr, file_path: &String) -> io::Result<()> {
    let fhandle = OpenOptions::new().read(true).open(file_path).expect("File could not be accessed");
    let mut reader = BufReader::new(fhandle);
    let stream = TcpStream::connect(SocketAddr::new(*raw_ip, 21_000))?;
    let mut bufwr = BufWriter::new(stream);
    let _ = io::copy(&mut reader, &mut bufwr);
    bufwr.write(b"\n")?;
    return Ok(());
}

pub fn naked_try_send() -> io::Result<()>{
    let stream = TcpStream::connect("172.26.166.42:21000")?;
    let mut writer = BufWriter::new(stream);
    writer.write_all(b"Hello Eric!")?;
    return Ok(())
}

pub fn listen() -> io::Result<()> {
    let listener = TcpListener::bind(
        "0.0.0.0:21000")?;
    let mut container = String::new();
    match listener.accept() {
        Ok((mut stream, _)) => {
            stream.read_to_string(&mut container)?;
            println!("{}", container);
            thread::sleep(Duration::from_secs(5));
            return Ok(());
        }
        Err(_) => panic!("Error accepting stream"),
    }
}




pub fn f_write(fpath: &String, ip: &IpAddr) -> io::Result<()>{
    let stream = TcpStream::connect(SocketAddr::new(*ip, 21_000))?;
    let mut bufw = BufWriter::new(stream);
    let fhandle = OpenOptions::new().read(true).open(fpath)?;
    let mut bufr = BufReader::new(fhandle);
    //io::copy to bufw and bufr

    let val = io::copy(&mut bufr, &mut bufw)?;
    println!("{} have been copied to the stream", val);
    return Ok(());
}

pub fn send_details(fpath: &String, ip:& IpAddr) -> io::Result<()>{
    let stream = TcpStream::connect(SocketAddr::new(*ip, 21_000))?;
    let mut bufw = BufWriter::new(stream);
    bufw.write_all(fpath.as_bytes())?;
    return Ok(());
}

pub fn send_with_retries(addr: &IpAddr, fname: &String) -> io::Result<()>{
    let mut tries = SEND_ATTEMPTS.clone();
    while tries > 0{
        match try_send(addr, fname){
            Ok(_) => return Ok(()),
            Err(err) => match err.kind(){
                ErrorKind::ConnectionAborted => {
                    eprintln!("The connection was aborted by the remote client. {} tries remain", tries);
                    tries -= 1;
                    
                }
                ErrorKind::ConnectionReset => {
                    eprintln!("The connection was reset by the remote client. {} tries remain", tries);
                    tries -= 1;
                }
                ErrorKind::TimedOut => {
                    eprintln!("The operation could not be completed in time. {} tries remain", tries);
                    tries -= 1;
                }
                err => {
                    eprintln!("Unexpected Error: {}. {} retries left", err, tries);
                    tries -= 1;
                }
            }
        }
    }



    Ok(())

}