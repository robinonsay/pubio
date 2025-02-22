use http::{mp::ThreadPool, server::HttpServer};

fn main() {
    let thread_pool = ThreadPool::<1000, 4>::new();
    let server = HttpServer::new("localhost:8080", &[], &thread_pool).unwrap();
    server.serve().unwrap();
}
