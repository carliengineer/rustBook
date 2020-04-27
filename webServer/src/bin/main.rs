use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
//use threadpool::ThreadPool;
use webServer::ThreadPool;
//use std::sync::mpsc::channel;

//type alias, for conveince
type Job = Box<dyn FnOnce() + Send + 'static>;

fn main() {
    //the stream consists of calling unwrap to terminate our program if the stream has any errors; if there aren’t any errors, the program prints a message.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let multi_threading = true;
    let pool = ThreadPool::new(4);


    //The reason we might receive errors from the incoming method when a client connects to the server is that we’re not actually iterating over connections. Instead, we’re iterating over connection attempts.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("Conection established");
        pool.execute(|| {
            handle_connection(stream);
        });
    }

}

fn handle_connection(mut stream : TcpStream) {
    //buffer 512 bytes in size
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    // if buffer.starts_with(get) {
    //
    //     //return contents of the web server
    //     let contents = fs::read_to_string("first.html").unwrap();
    //
    //     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    //     //Because the write operation could fail, we use unwrap on any error result as before. Again, in a real application you would add error handling here.
    //     stream.write(response.as_bytes()).unwrap();
    //     //flush will wait and prevent the program from continuing until all the bytes are written to the connection
    //     stream.flush().unwrap();
    //
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //
    //     let response = format!("{}{}", status_line, contents);
    //
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "first.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();



}

// //multi-threading
// //A thread pool is a group of spawned threads that are waiting and ready to handle a task.

// fn mthread_handle_connection(mut stream : TcpStream ) {

// }