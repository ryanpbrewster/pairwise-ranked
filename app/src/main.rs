use app::{Model, Msg};

fn main() {
    yew::initialize();
    let mut name = stdweb::web::window().location().unwrap().hash().unwrap();
    if name.starts_with('#') {
        name.remove(0);
    }
    if name.is_empty() {
        name = "colors".to_owned();
    }
    yew::App::<Model>::new()
        .mount_to_body()
        .send_message(Msg::FetchList(name));
    yew::run_loop();
}
