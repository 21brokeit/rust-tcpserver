// IMPORT ALL NECESSARY MODELS 
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_tcp(mut stream:TcpStream){

    // a buffer to read data from the client
    let mut buffer= [0; 1024];

    stream.read(&mut buffer).expect("Failed to read from client");

    let request= String::from_utf8_lossy(&buffer[..]);
    println!("Received request {} ",request );
    let response = "Hello there welcome".as_bytes();
    stream.write(response).expect("Failed to send response");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to listen");
    println!("server listening on 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream {
            Ok(stream)=>{
                std::thread::spawn(|| handle_tcp(stream));
            }
            Err(e)=>{
                eprintln!("Failed to establish connection {} ",e);
            }
        }
    }

}