use airline::{Request, RequestType, Response, Route, Server};
use assets::Assets;
pub mod assets;

struct ServerEnvironment {
    database: Database,
}
struct Database;

fn setup() -> ServerEnvironment {
    return ServerEnvironment {
        database: Database {},
    };
}

fn index(_req: &Request, res: &mut Response, _data: &ServerEnvironment) {
    res.send_bytes(Assets::index());
}

fn main() {
    let mut server = Server::new(3000, setup);
    server.register(Route::create("", RequestType::Get, index));
    server.start();
}
