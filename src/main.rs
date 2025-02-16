use http::server::HttpServer;

mod http;

fn main() {
    let server = HttpServer::new("localhost:8080", &[]).unwrap();
    server.serve().unwrap();
}
