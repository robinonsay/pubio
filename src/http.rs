use std::{io::{BufRead, BufReader, Error, ErrorKind, Read}, net::TcpStream, result, str::FromStr};

pub mod server;

#[derive(Debug)]
pub enum HttpHeader
{

}

#[derive(Debug)]
pub struct HttpContent
{
    http_version: String,
    route: String,
    headers: Vec<String>,
    body: String,
}

#[derive(Debug)]
pub struct HttpResponse
{
    content: HttpContent
}

#[derive(Debug)]
pub enum HttpRequest
{
    Get(HttpContent),
    Head(HttpContent),
    Post(HttpContent),
    Put(HttpContent),
    Delete(HttpContent),
    Connect(HttpContent),
    Options(HttpContent),
    Trace(HttpContent),
    Patch(HttpContent),
}

impl HttpRequest
{
    pub fn new(stream: &TcpStream) -> Result<Self, Error>
    {
        let mut buf_rdr = BufReader::new(stream);
        let mut lines = buf_rdr.lines();
        // Create start line buffer
        let start_line = lines.next()
        .ok_or(Error::new(ErrorKind::InvalidData, "Invalid Buffer"))??;
        // Parse start line by white spaces
        let mut parts = start_line.split_whitespace();
        // Get method
        let method = parts.next()
        .ok_or(Error::new(ErrorKind::InvalidData, "Invalid start line in method"))?
        .to_string();
        // Get route
        let route = parts.next()
        .ok_or(Error::new(ErrorKind::InvalidData, "Invalid start line in route"))?
        .to_string();
        // Get version
        let version = parts.next()
        .ok_or(Error::new(ErrorKind::InvalidData, "Invalid start line in version"))?
        .to_string();
        // Get headers
        let mut headers = Vec::<String>::new();
        loop
        {
            let line = lines.next()
            .ok_or(Error::new(ErrorKind::InvalidData, "Invalid Buffer"))??;
            if line.is_empty()
            {
                break;
            }
            headers.push(line);            
        }
        // Create the http content
        let http_content = HttpContent
        {
            http_version: version,
            route: route,
            headers: headers,
            body: String::new(),
        };
        // Match the method with the http content
        match method.as_str() {
            "GET" => return Ok(HttpRequest::Get(http_content)),
            "HEAD" => return Ok(HttpRequest::Head(http_content)),
            "POST" => return Ok(HttpRequest::Post(http_content)),
            "PUT" => return Ok(HttpRequest::Put(http_content)),
            "DELETE" => return Ok(HttpRequest::Delete(http_content)),
            "CONNECT" => return Ok(HttpRequest::Connect(http_content)),
            "OPTIONS" => return Ok(HttpRequest::Options(http_content)),
            "TRACE" => return Ok(HttpRequest::Trace(http_content)),
            "PATCH" => return Ok(HttpRequest::Patch(http_content)),
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid http method"))
        }
    }
}
