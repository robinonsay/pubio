use std::{io::{BufRead, BufReader, Error, Write}, net::{TcpListener, TcpStream}};

use super::{HttpRequest, HttpResponse};

pub type HttpHandler = fn(server: HttpRequest) -> Result<HttpResponse, Error>;

pub struct HttpRoute
{
    route: &'static str,
    handler: HttpHandler
}


pub struct HttpServer<'a>
{
    listener: TcpListener,
    handlers: &'a [HttpRoute]
}


impl<'a> HttpServer<'a>
{
    pub fn new(addr: &str, handlers: &'a [HttpRoute]) -> Result<Self, Error>
    {
        let listener = TcpListener::bind(addr)?;
        Ok(Self{
            listener: listener,
            handlers: handlers
        })
    }

    fn conn_handler(&self, mut stream: TcpStream)
    {
        let buf_reader = BufReader::new(&stream);
        let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
        println!("Request: {http_request:#?}");
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn serve(&self) -> Result<(), Error>
    {
        println!("Serving...");
        for stream in self.listener.incoming()
        {
            let stream = stream?;
            self.conn_handler(stream);
        }
        Ok(())
    }
}
