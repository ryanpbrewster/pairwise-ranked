use app::{Model, Msg};

fn main() {
    yew::initialize();
    yew::App::<Model>::new()
        .mount_to_body()
        .send_message(Msg::FetchList("taylorswift".to_owned()));
    yew::run_loop();
}
