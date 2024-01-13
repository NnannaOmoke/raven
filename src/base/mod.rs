use local_ip_address;
use rand::{self, distributions::Uniform, prelude::*};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    error::Error,
    fs::OpenOptions,
    io::{self, prelude::*, BufReader, BufWriter},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
};

const REGISTRY_FHANDLE_NAME: &str = "RegistrySerialized.txt";
const REGISTRY_LISTEN_PORT: usize = 61_000;


#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct Node {
    master: Option<IpAddr>,
    ip_addr: IpAddr,
    port_num: usize,
}

impl Node {
    const PREF_REGISTRY_LISTEN_PORT: usize = 55_555;
    const PREF_PING_LISTEN_PORT: usize = 55_777;
    const PREF_DATA_LISTEN_PORT: usize = 55_888;

    //duplex method
    pub fn build(master: Option<IpAddr>) -> Node {
        return Node {
            master,
            ip_addr: local_ip_address::local_ip()
                .expect("Could not obtain IP address as Node device"),
            port_num: 55_000usize,
        };
    }

    //reciever method
    //put this in a thread, that will be bound to a port. It will keep listening until a connection comes in
    pub fn listen(&self, req_accepted: bool, file_descriptor: &str) -> () {
        let listener = TcpListener::bind(SocketAddr::new(
            self.ip_addr,
            Self::PREF_DATA_LISTEN_PORT as u16,
        ))
        .expect("Could not connect to the port");
        if req_accepted {
            match listener.accept() {
                Ok((stream, _)) => self.write_to_file(stream, file_descriptor),
                Err(_) => eprintln!("Connection failed"), //add further documentation later
            }
        }
    }

    //sender method
    pub fn request_id(&self, id: usize) -> Node {
        //send a request to the master for the details of the device you want to transfer to
        let stream = TcpStream::connect(SocketAddr::new(
            self.master.unwrap(),
            REGISTRY_LISTEN_PORT as u16,
        ))
        .expect("Connection to master device failed. Could not reach registry");
        //write request to stream
        let mut bufwriter = BufWriter::new(stream);
        let data = &id.to_string();
        bufwriter
            .write_all(data.as_bytes())
            .expect("Could not write bytes to stream!");

        //listen for registry response
        let listener = TcpListener::bind(SocketAddr::new(
            self.ip_addr,
            Self::PREF_REGISTRY_LISTEN_PORT as u16,
        ))
        .expect("Could not get registry response");
        let node = match listener.accept() {
            Ok((stream, _)) => {
                //destructure registry response and return it
                //it comes in as bytes, no?
                //read bytes and write them to something
                let mut reader = BufReader::new(stream);
                let mut bytes = vec![];
                reader
                    .read_to_end(&mut bytes)
                    .expect("Could not read bytes from input stream!");
                let val: Node = serde_json::from_slice(&bytes).expect("Deserialization Failed!");
                val
            }
            Err(_) => panic!("Error accepting input stream!"),
        };
        return node;
    }

    //sender method
    pub fn ping(&self, external_node: &Node) -> bool {
        //request to connect. This should return a boolean value
        let string = &format!(
            "{} wants to connect to your device\nAccept connection\n[Y/N]",
            { self.ip_addr }
        );
        let stream = TcpStream::connect(SocketAddr::new(
            external_node.ip_addr,
            external_node.port_num as u16,
        ))
        .expect("Connection to external node failed!");
        let mut bufwriter = BufWriter::new(stream);
        bufwriter
            .write_all(&mut string.as_bytes())
            .expect("Write to stream failed!");

        //then listen in for incoming responses from the node
        let listener = TcpListener::bind(SocketAddr::new(
            self.ip_addr,
            Self::PREF_PING_LISTEN_PORT as u16,
        ))
        .expect("Connection failed. Port is occupied");
        //accept connections and send response off to some handler
        match listener.accept() {
            //supposed to read the stream and return the boolean value!
            Ok((stream, _)) => {
                let mut reader = BufReader::new(stream);
                let mut string_bool = String::new();
                reader
                    .read_to_string(&mut string_bool)
                    .expect("Nigga what the fuck is wrong with your internet?");
                let final_bool = string_bool
                    .trim()
                    .parse::<bool>()
                    .expect("Conversion failed!");
                return final_bool;
            }
            Err(_) => panic!("Error retrieving data from node"),
        }
    }

    //private reciever method
    //we will have to modify the listen method so as to obtain the metadata of the file
    //we will also have to sanitize the file descriptor too. i.e. convert /../../somethign.img to something.img
    pub fn write_to_file(&self, external_stream: TcpStream, file_descriptor: &str) -> () {
        //write contents of this stream to a file
        let mut bufr = BufReader::new(external_stream);
        let fhandle = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_descriptor)
            .expect("File already exists in location!");
        //shebi if i begin use this format methodology my code go choke
        let mut bufw = BufWriter::new(fhandle);
        //read from the stream to a vec, and then write from the vec to the file
        //this could be expensive. Could the reader not read to a bufwriter directly?
        io::copy(&mut bufr, &mut bufw).expect("Failed to read from stream to file");
        return ;
    }

    //sender method
    pub fn send_to_stream(&self, file_descriptor: &str, external_device: &Node) -> (){
        let fhandle = OpenOptions::new().read(true).open(file_descriptor).expect("Could not open file!");
        let mut bufr = BufReader::new(fhandle);
        let stream = TcpStream::connect(SocketAddr::new(external_device.ip_addr, Self::PREF_DATA_LISTEN_PORT as u16)).expect("Port busy!");
        let mut bufw = BufWriter::new(stream);
        io::copy(&mut bufr, &mut bufw).expect("Reading from file to stream failed");
        return;
    }

    //private reciever method
    fn handle_connect_requests(&self, external_stream: TcpStream) -> bool {
        let mut buf = vec![];
        let mut reader = BufReader::new(external_stream);
        reader
            .read_to_end(&mut buf)
            .expect("Could not read from input node");

        let reply = String::from_utf8(buf).expect("Non valid UTF-8 String detected");
        println!("{reply}:");
        let mut ans = String::new();
        io::stdin().read_to_string(&mut ans).unwrap();
        match &ans.trim()[..] {
            "Y" | "y" | "yes" | "YES" => return true,
            _ => return false,
        }
    }

    //reciever methods
    //bind a listener to a port and wait for ping reqs
    pub fn ping_listener(&self) -> bool {
        //maybe we return the value of the `handle_connect_requests` method
        let listener = TcpListener::bind(SocketAddr::new(
            self.ip_addr,
            Self::PREF_PING_LISTEN_PORT as u16,
        ))
        .expect("Port Unavailable");
        match listener.accept() {
            Ok((stream, _)) => self.handle_connect_requests(stream),
            Err(_) => panic!(),
        }
    }
    //sender
    pub fn file_parser() -> String{
        //shouldn't we parse this as args or something?
        //it should be higher level
        todo!()
    }
}

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct Master {
    registry: HashMap<usize, Node>,
}

impl Master {
    const PREF_PORT: usize = 61_000;

    //there is no new function. Master nodes are built entirely from device nodes.
    pub fn add_node(&mut self, device: &mut Node) -> () {
        let mut rng = rand::thread_rng();
        let mut id: usize = rng.gen_range(1_000_000_000..9_999_999_999);
        while self.registry.contains_key(&id) {
            id += rng.sample(Uniform::new(1, 99));
        }
        //add device to registry after validating id
        self.registry.insert(id, device.clone());
        //then, make the master node point to the ipaddress of the master machine
        device.master =
            Some(local_ip_address::local_ip().expect("IP address for master machine unobtainable"));
    }

    //removes a node from the registry
    //isn't it better to be able to refer to devices by their id?
    //i.e. id should be assigned by master?
    pub fn rm_node(&mut self, device_id: usize) -> () {
        //remove key from registry
        self.registry.remove_entry(&device_id);
    }

    //remember to refactor to get the id of the device requesting for id and validate that
    //to be fair, that was a one line function
    pub fn serialize_device_state(&self, id: usize) -> Vec<u8> {
        //i.e. after setting the device state, you can then serialize the new device struct as a vector,
        //then send that over as a stream of bytes to a device, which then reads it and modifies the fields as neccesary
        let device = &self.registry[&id];
        let bytes = serde_json::to_vec(device).expect("Serialization of device failed!");
        return bytes;
    }

    pub fn save_registry_state(&self) -> () {
        let fhandle = OpenOptions::new()
            .write(true)
            .open(REGISTRY_FHANDLE_NAME)
            .expect("Error Opening the registry file");
        let mut vec = serde_json::to_vec(self).expect("Error Serializing Registry");
        let mut bufwriter = BufWriter::new(fhandle);
        bufwriter
            .write_all(&mut vec)
            .expect("Error writing to Registry file");
    }

    pub fn load_registry_state() -> Master {
        let fhandle = OpenOptions::new()
            .read(true)
            .open(REGISTRY_FHANDLE_NAME)
            .expect("Error reading from registry");
        let mut bufreader = BufReader::new(fhandle);
        let mut data = vec![];
        let _ = bufreader.read_to_end(&mut data);
        let master = serde_json::from_slice(&data).expect("Error deserializing registry!");
        return master;
    }

    pub fn validate_id(&self, id: usize) -> bool {
        return self.registry.contains_key(&id);
    }
}

impl From<Node> for Master {
    fn from(mut device: Node) -> Self {
        let hmap: HashMap<usize, Node> = HashMap::new();
        let mut master = Master { registry: hmap };
        master.add_node(&mut device);
        return master;
    }
}
