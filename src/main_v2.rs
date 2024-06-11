// https://docs.rs/wayland-client/latest/wayland_client/#getting-started-example
use wayland_client::{protocol::wl_registry, Connection, Dispatch};
use wayland_protocols_wlr::foreign_toplevel::v1::client::{
    zwlr_foreign_toplevel_handle_v1::{self, ZwlrForeignToplevelHandleV1},
    zwlr_foreign_toplevel_manager_v1::{self, ZwlrForeignToplevelManagerV1},
};
struct AppData;

impl Dispatch<ZwlrForeignToplevelManagerV1, ()> for AppData {
    fn event(
        _: &mut Self,
        _proxy: &ZwlrForeignToplevelManagerV1,
        event: <ZwlrForeignToplevelManagerV1 as wayland_client::Proxy>::Event,
        _: &(),
        _conn: &Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            zwlr_foreign_toplevel_manager_v1::Event::Toplevel { toplevel } => {
                println!("new toplevel")
            }
            zwlr_foreign_toplevel_manager_v1::Event::Finished => (),
            _ => todo!(),
        }
    }
}

impl Dispatch<ZwlrForeignToplevelHandleV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &ZwlrForeignToplevelHandleV1,
        event: <ZwlrForeignToplevelHandleV1 as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            zwlr_foreign_toplevel_handle_v1::Event::State { state } => println!("{:#?}", state),
            zwlr_foreign_toplevel_handle_v1::Event::Title { title } => print!("{}", title),
            zwlr_foreign_toplevel_handle_v1::Event::AppId { app_id } => print!("{app_id}"),
            _ => todo!(),
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            println!("[{}] {} (v{})", name, interface, version);
        }
    }
}

fn main() {
    let conn = match Connection::connect_to_env() {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error connecting to Wayland: {}", e);
            return;
        }
    };

    // Retrieve the WlDisplay Wayland object from the connection. This object is
    // the starting point of any Wayland program, from which all other objects will
    // be created.
    let display = conn.display();

    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let _registry = display.get_registry(&qh, ());

    println!("Advertised globals:");

    event_queue.roundtrip(&mut AppData).unwrap();
    // event_queue.
}
