use std::{fs, io::{Error, Write}, net::{TcpListener, TcpStream}};

use super::{mp::Executable, HttpRequest, HttpResponse};

pub type HttpHandler = fn(server: HttpRequest) -> Result<HttpResponse, Error>;

#[allow(dead_code)]
pub struct HttpRouteHandler
{
    route: &'static str,
    handler: HttpHandler
}

pub enum HttpMethodHandler
{
    Get(HttpRouteHandler),
    Head(HttpRouteHandler),
    Post(HttpRouteHandler),
    Put(HttpRouteHandler),
    Delete(HttpRouteHandler),
    Connect(HttpRouteHandler),
    Options(HttpRouteHandler),
    Trace(HttpRouteHandler),
    Patch(HttpRouteHandler),
}

pub struct HttpServer<'a>
{
    listener: TcpListener,
    handlers: &'a [HttpMethodHandler],
    thread_pool: &'a dyn Executable
}

struct HttpProcessor
{
    // handlers: &'a [HttpMethodHandler],
}

impl HttpProcessor
{
    pub fn new() -> Self
    {
        // Self
        // {
        //     handlers: handlers
        // }
        Self{}
    }

    fn conn_handler(&self, mut stream: TcpStream) -> Result<(), Error>
    {
       
        let http_request = HttpRequest::new(&stream)?;
        match http_request {
            HttpRequest::Get(content) =>
            {
                println!("Http GET Request: {}", content.route)
            },
            _ => {}
        }
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
        Ok(())
    }
}

impl<'a> HttpServer<'a>
{
    pub fn new(addr: &str, handlers: &'a [HttpMethodHandler], thread_pool: &'a dyn Executable) -> Result<Self, Error>
    {
        let listener = TcpListener::bind(addr)?;
        Ok(Self{
            listener: listener,
            handlers: handlers,
            thread_pool: thread_pool
        })
    }

    pub fn serve(&self) -> Result<(), Error>
    {
        println!("Serving...");
        for stream in self.listener.incoming()
        {
            let stream = stream?;
            let job = move ||
            {
                let processor = HttpProcessor::new();
                processor.conn_handler(stream).unwrap();
            };
            let job = Box::new(job);
            self.thread_pool.try_submit(job)?;
        }
        Ok(())
    }
}
