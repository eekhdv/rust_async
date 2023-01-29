// mod web_server;
// use web_server::handlers;

// use tokio::net::TcpListener;

use tokio;
use wayland_client::{protocol::wl_registry, Connection, Dispatch, QueueHandle};

struct AppData;

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        // When receiving events from the wl_registry, we are only interested in the
        // `global` event, which signals a new available global.
        // When receiving this event, we just print its characteristics in this example.
        if let wl_registry::Event::Global { name, interface, version } = event {
            println!("[{}] {} (v{})", name, interface, version);
        }
    }
}

#[tokio::main]
async fn main() {
    let conn = Connection::connect_to_env().expect("[ERROR] Cannot connect to the Wayland server!");
    let display = conn.display();

    let mut event_queue = conn.new_event_queue();
    
    let qh = event_queue.handle();

    let _registry = display.get_registry(&qh, ());



    event_queue.roundtrip(&mut AppData).unwrap();
    // let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    // loop {
    //     let (socket, _) = listener.accept().await.unwrap();
    //     tokio::task::spawn(async move {
    //         handlers::con_handler(socket).await;
    //     });
    // }
}

