//! Webserver part of the project
//! responsible for hosting the web-interface
//! responsible for handling websocket connections
//! will open a new webserver on the predefined ip
//! websocket peers can also send the following commands (as strings) to the server:
//! /list to receive a list of all available rooms
//! /join <anything> will join desired room, every message in that room will be relayed to the peer
//! if a peer tries to join a nonexisting room, the room will be created
//! /name <anything> will set the session name to the desired input
//! Example usage of Webserver
//! Keep in mind that you need to Spawn anything else that needs to run before the Webserver as it is blocking the whole thread. use tokio::thread::spawn(async move{<yourcode>}) to do so
//! ```rust
//! use bt_webserver::start_webserver;
//! #[actix::main]
//! async fn main() {
//!     start_webserver().await.unwrap();
//!
//! }
//! ```
use actix::*;
use actix_files::Files;
use actix_web::get;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};
mod websocket_server;
mod websocket_session;
/// main method of webserver
/// will start a webserver that hosts your html files
/// will start a websocketserver that handles websocket traffic
/// will mount everything in the src/webserver/static/js folder to /js
/// will mount everything in the src/webserver/static/css folder to /css
pub async fn start_webserver() -> std::io::Result<()> {
    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let server = websocket_server::ChatServer::new(app_state.clone()).start();
    println!("Starting webserver on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .service(Files::new("/js", "src/webserver/static/js"))
            .service(Files::new("/css", "src/webserver/static/css"))
            .service(index)
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .route("/count", web::get().to(get_count))
            .route("/ws", web::get().to(chat_route))
    })
    .disable_signals()
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .unwrap();
    Ok(())
}
/// all html requests to root will proc this method, should lead to some kind of index.html or similar
#[get("/")]
async fn index() -> impl Responder {
    let html = include_str!("webserver/static/index.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}
/// service that hosts a websocket server
/// when connected to the server you can send a string with /join <anything>
/// to join your desired room, after joining every string you send on that socket will be sent to every client connected to the same room
/// if the room does not yet exist, it will be created
/// you can also send /list as a string to the socket, the socket will then return all valid rooms
/// you can also send /name <anything> as a string to the socket to set a session name
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<websocket_server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        websocket_session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
/// keeps track of how many users are connected
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}
