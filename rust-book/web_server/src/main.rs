// REF: https://doc.rust-lang.org/book/ch20-01-single-threaded.html?search=template
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // // -- Thread for each request approach:
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });

        // -- Thread pool approach:
        pool.execute(|| handle_connection(stream));
    }

    // NOTE: U: After impl Drop for ThreadPool, when our server
    // shuts down after serving our requests (i.e. take(2)),
    // the ThreadPool will go out of scope here at the end of main,
    // and the drop() implementation that closes the channel by
    // moving the channel sender out of ThreadPool, which results in
    // our Worker threads to return errors. This is all part of
    // the graceful shutdown.
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Request: {:#?}", http_request);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // REFACTORED:
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    // TODO: Insert template variables here!
    insert_template_variable(&contents);
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // OLD Manual Templating!:
    // if request_line == "GET / HTTP/1.1" {
    //     // NOTE: Responses have the following format:
    //     // HTTP-Version Status-Code Reason-Phrase CRLF
    //     // headers CRLF
    //     // message-body
    //     let status_line = "HTTP/1.1 200 OK";
    //     let mut contents = fs::read_to_string("hello.html").unwrap();
    //     // Q: Does Templating take place here? Parse contents for "{{ foo }}" and
    //     // insert the value before using format!() to build the response String?
    //     // A: YES! See below.
    //     let template_var_1 = "var1";
    //     let template_var_2 = "var2";
    //
    //     contents = contents.replace("{{ template_var_1 }}", template_var_1);
    //     contents = contents.replace("{{ template_var_2 }}", template_var_2);
    //
    //     // for line in contents.lines() {
    //     //     if line.contains("{{ template_var_1 }}") {
    //     //         line.replace("template_var_1", template_var_1)
    //     //     } else if line.contains("{{ template_var_2 }}") {
    //     //         line.replace("{{ template_var_2 }}", template_var_2)
    //     //     } else {
    //     //         continue;
    //     //     };
    //     // }
    //
    //     let length = contents.len();
    //
    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     // Some other request
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();
    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }
}

fn insert_template_variable(var: &str) {
    let formatted_str = format!("{{ {} }}", var);
    // println!("formatted_str: {:?}", formatted_str);
}
