use crate::*;

const SEND_ATTEMPTS: usize = 3;
const FNAME_LIMIT_SIZE: usize = 255;

fn raw_send(raw_ip: &IpAddr, file_path: &String) -> io::Result<()> {
    let mut stopped = 0usize;
    let fhandle = OpenOptions::new().read(true).open(file_path)?;
    let mut bufr = BufReader::new(fhandle);
    //we have to find a means of writing the filename to the stream
    let mut empbuffer = [0; FNAME_LIMIT_SIZE];
    let fpatharr = file_path.as_bytes();
    //if it exceeds our allowed filename, throw an error
    if fpatharr.len() >= FNAME_LIMIT_SIZE{
        eprintln!("The file path exceeds allowed limits!");
        return Err(
            io::Error::from(ErrorKind::InvalidInput));
    }
    //copy the contents of fpatharr to empbuffer
    for (index, elem) in fpatharr.bytes().enumerate(){
        empbuffer[index] = elem.unwrap();
        stopped = index;
    }
    //Fill the remainder of the buffer with whitespaces
    empbuffer[stopped..FNAME_LIMIT_SIZE - 1].fill(b" "[0]);
    //connect to remote address
    let stream = TcpStream::connect(SocketAddr::new(*raw_ip, 21_000))?;
    let mut bufwr = BufWriter::new(stream);
    bufwr.write_all(&empbuffer[..])?;
    io::copy(&mut bufr, &mut bufwr)?;
    Ok(())
}

fn test() -> io::Result<()>{
    let stream = TcpStream::connect("172.26.166.42:21000")?;
    let mut bufw = BufWriter::new(stream);
    bufw.write_all(b"Hello Eric!")?;
    return Ok(())
}


fn write_to_file(stream: TcpStream) -> io::Result<()>{
    //the first 255 bytes of the stream is the filename, which we will pa
    let mut fname = [0; FNAME_LIMIT_SIZE];
    let mut bufr = BufReader::new(stream);
    bufr.read_exact(&mut fname)?;
    //we've read 255 bytes to the fname string, the rest should just be filled with the whitespace bytes, so we'll strip it
    let fname = String::from_utf8_lossy(&fname).trim_end().to_string();
    let fhandle = OpenOptions::new().write(true).create_new(true).open(fname)?;
    let mut bufw = BufWriter::new(fhandle);
     //we want to read anything that occurs from the first 255 bytes of a reader, to EOF
    io::copy(&mut bufr, &mut bufw)?;
    Ok(())
}


pub fn send_with_retries(addr: &IpAddr, fname: &String) -> io::Result<()>{
    let mut tries = SEND_ATTEMPTS.clone();
    while tries > 0{
        match raw_send(addr, fname){
            Ok(_) => return Ok(()),
            Err(err) => match err.kind(){
                ErrorKind::ConnectionAborted => {
                    if tries == 0{
                        return Err(err);
                    }
                    eprintln!("The connection was aborted by the remote client. {} tries remain", tries);
                    tries -= 1;
                    
                }
                ErrorKind::ConnectionReset => {
                    if tries == 0{
                        return Err(err);
                    }
                    eprintln!("The connection was reset by the remote client. {} tries remain", tries);
                    tries -= 1;
                }
                ErrorKind::TimedOut => {
                    if tries == 0{
                        return Err(err);
                    }
                    eprintln!("The operation could not be completed in time. {} tries remain", tries);
                    tries -= 1;
                }
                ErrorKind::InvalidInput => {
                    return Err(err)
                }
                error => {
                    if tries == 0{
                        return Err(err);
                    }
                    eprintln!("Unexpected Error: {}. {} retries left", error, tries);
                    tries -= 1;
                }
            }
        }
    }
    //would never actually get here, but OK
    Ok(())
}

pub fn listen() -> io::Result<()>{
    let listener = TcpListener::bind("0.0.0.0:21000")?;
    for container in listener.incoming(){
        //maybe put the name of the file in the first, maybe 255 bytes of the stream?
        let stream = container?;
        write_to_file(stream)?;
    }
    Ok(())
}