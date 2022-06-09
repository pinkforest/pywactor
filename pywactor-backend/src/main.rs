mod http;
use lunatic::{Mailbox};
use lunatic::process::StartProcess;
use lunatic::process::Request;

#[lunatic::main]
fn main(_: Mailbox<()>) {

    let http_server_supervisor = http::HttpServerSupervisor::start((), None);
    let children = http_server_supervisor.children();

    // I tried accept() under both directly start() above and request() below but it blocks either way
    // accept() is supposed to happen on child process thus not blocking everything?
    // The below only accepts connections to 127.0.0.1:9191 despite supposed to be listening in 9191 and 9192
    children.0.request(http::HttpServerCommand { command: http::HttpServerCommandType::Listen });
    children.1.request(http::HttpServerCommand { command: http::HttpServerCommandType::Listen });
    
    dbg!(children);

}
