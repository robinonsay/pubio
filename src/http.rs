pub mod server;

pub enum HttpHeader
{

}

pub struct HttpContent
{
    headers: Vec<HttpHeader>,
    body: String,
}

pub struct HttpResponse
{
    content: HttpContent
}

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
