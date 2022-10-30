// main调用server server调用router router调用handler
mod server;
mod router;
mod handler;

use server::Server;

fn main() {
    let server = Server::new("localhost:3001");
    server.run();
}
