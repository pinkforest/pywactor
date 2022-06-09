use lunatic::process::{AbstractProcess, ProcessRef, ProcessRequest};

use lunatic::net::TcpStream;
use lunatic::supervisor::Supervisor;
use lunatic::supervisor::SupervisorConfig;
use lunatic::supervisor::SupervisorStrategy;

pub struct HttpServerSupervisor;

impl Supervisor for HttpServerSupervisor {
    type Arg = ();
    type Children = (HttpServer, HttpServer);

    fn init(config: &mut SupervisorConfig<Self>, _: ()) {
        config.set_strategy(SupervisorStrategy::OneForOne);
        config.children_args(
            (
                ("127.0.0.1:9191".to_owned(),Some( "main1".to_owned())),
                ("127.0.0.1:9192".to_owned(),Some( "main2".to_owned())),
            ));
    }
}

use lunatic::net::TcpListener;
use lunatic::process::StartProcess;

pub struct HttpServer {
    tcp_listener: TcpListener,
    server_addr: String,
}

impl AbstractProcess for HttpServer {
    type Arg = String;
    type State = Self;

    fn init(_: ProcessRef<Self>, server_addr: String) -> Self {
        let tcp_listener = TcpListener::bind(server_addr.clone()).unwrap();
        let server_addr_bound = tcp_listener.local_addr().unwrap();
        println!("Listening on addr: {}", &server_addr);
        HttpServer { tcp_listener, server_addr }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum HttpServerCommandType {
    Listen,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HttpServerCommand {
    pub command: HttpServerCommandType,
}

impl ProcessRequest<HttpServerCommand> for HttpServer {
    type Response = u32;

    fn handle(state: &mut Self::State, _: HttpServerCommand) -> u32 {
        println!("handle() Listen @ {} ", &state.server_addr);
        loop {
            println!("handle() Entered loop @ {}", &state.server_addr);
            if let Ok((tcp_stream, _peer)) = state.tcp_listener.accept() {
                dbg!("TCP Stream accepted @ {}", &state.server_addr);
                let http = Http::start(tcp_stream, Some("hmmm"));
            }
        }
    }
}

pub struct Http {
    tcp_stream: TcpStream,
}

impl AbstractProcess for Http {
    type Arg = TcpStream;
    type State = Self;

    fn init(_: ProcessRef<Self>, tcp_stream: TcpStream) -> Self {
        dbg!("Initialised TCP Stream :: post");
        Http { tcp_stream }
    }
}
