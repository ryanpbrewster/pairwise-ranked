#![recursion_limit = "128"]
#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use stdweb::js;
use yew::format::{Json, Nothing};
use yew::services::fetch;
use yew::services::fetch::{FetchTask, Response};
use yew::services::{ConsoleService, FetchService};
use yew::{
    html, Component, ComponentLink, Html, IKeyboardEvent, KeyDownEvent, KeyUpEvent, ShouldRender,
};
use wasm_bindgen::prelude::*;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    let mut name = stdweb::web::window().location().unwrap().pathname().unwrap();
    if name.starts_with('/') {
        name.remove(0);
    }
    if name.is_empty() {
        name = "colors".to_owned();
    }
    yew::App::<Model>::new()
        .mount_to_body()
        .send_message(Msg::FetchList(name));
    yew::run_loop();
    Ok(())
}

pub struct Model {
    link: ComponentLink<Self>,
    console: ConsoleService,
    fetch: FetchService,
    items: Vec<String>,
    ords: Vec<(Pair, Ordering)>,
    sort_state: SortState,
    keyboard_state: KeyboardState,
    fetch_task: Option<FetchTask>,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Pair(pub usize, pub usize);
impl Pair {
    pub fn reverse(&self) -> Pair {
        Pair(self.1, self.0)
    }
}

pub struct SortState {
    num_missing_ords: usize,
    next_missing_ord: Option<Pair>,
    current_order: Permutation,
}

type Permutation = Vec<usize>;

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct KeyboardState {
    left: KeyState,
    right: KeyState,
    down: KeyState,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum KeyState {
    Idle,
    Pressed,
}
impl Default for KeyState {
    fn default() -> Self {
        KeyState::Idle
    }
}

pub enum Msg {
    FetchList(String),
    NewList(Vec<String>),
    Debug(String),
    Rank(Pair, Ordering),
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let items: Vec<String> = vec!["loading..."].into_iter().map(String::from).collect();
        let ords = Vec::new();
        let sort_state = compute_ordering(&items, &ords);

        let keydown = {
            let cb = link.send_back(|evt| Msg::KeyDown(evt));
            move |evt| cb.emit(evt)
        };
        let keyup = {
            let cb = link.send_back(|evt| Msg::KeyUp(evt));
            move |evt| cb.emit(evt)
        };
        js! {
            const keydown = @{keydown};
            const keyup = @{keyup};
            window.addEventListener("keydown", evt => keydown(evt));
            window.addEventListener("keyup", evt => keyup(evt));
        };
        Model {
            link,
            console: ConsoleService::new(),
            fetch: FetchService::new(),
            fetch_task: None,
            items,
            ords,
            sort_state,
            keyboard_state: KeyboardState::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Debug(msg) => self.console.log(&msg),
            Msg::FetchList(name) => {
                self.console.log(&format!("fetching new list: {}", name));
                let req = fetch::Request::get(format!(
                    "https://pairwise-ranked.firebaseio.com/lists/{}.json",
                    name
                ))
                .body(Nothing)
                .unwrap();
                let task = self.fetch.fetch(
                    req,
                    self.link.send_back(
                        |resp: Response<Json<Result<Vec<String>, _>>>| {
                            let (meta, Json(body)) = resp.into_parts();
                            if !meta.status.is_success() {
                                return Msg::Debug(format!("{:?}", meta));
                            }
                            match body {
                                Ok(items) => Msg::NewList(items),
                                Err(err) => Msg::Debug(format!("{:?}", err)),
                            }
                        },
                    ),
                );
                self.fetch_task = Some(task);
            }
            Msg::NewList(items) => {
                self.console.log("starting new list");
                self.items = items;
                self.mutate_ords(|ords| ords.clear());
            }
            Msg::Rank(pair, cmp) => {
                self.mutate_ords(|ords| ords.push((pair, cmp)));
            }
            Msg::KeyDown(evt) => {
                match evt.key().as_ref() {
                    "ArrowLeft" => self.keyboard_state.left = KeyState::Pressed,
                    "ArrowRight" => self.keyboard_state.right = KeyState::Pressed,
                    "ArrowDown" => self.keyboard_state.down = KeyState::Pressed,
                    "z" if evt.ctrl_key() => {
                        self.mutate_ords(|ords| {
                            ords.pop();
                        });
                    }
                    _ => {}
                };
            }
            Msg::KeyUp(evt) => {
                match evt.key().as_ref() {
                    "ArrowLeft" => {
                        self.keyboard_state.left = KeyState::Idle;
                        if let Some(pair) = self.sort_state.next_missing_ord {
                            self.link.send_self(Msg::Rank(pair, Ordering::Greater));
                        }
                    }
                    "ArrowRight" => {
                        self.keyboard_state.right = KeyState::Idle;
                        if let Some(pair) = self.sort_state.next_missing_ord {
                            self.link.send_self(Msg::Rank(pair, Ordering::Less));
                        }
                    }
                    "ArrowDown" => {
                        self.keyboard_state.down = KeyState::Idle;
                        if let Some(pair) = self.sort_state.next_missing_ord {
                            self.link.send_self(Msg::Rank(pair, Ordering::Equal));
                        }
                    }
                    _ => {}
                };
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div id="main">
            { view_info(&self.items, &self.sort_state, self.keyboard_state) }
            { format!("{} in, {} to go", self.ords.len(), self.sort_state.num_missing_ords) }
            { view_items(&self.items, &self.sort_state.current_order) }
            </div>
        }
    }
}

impl Model {
    /// Convenience wrapper that allows mutation of `self.ords` and automatically
    /// recomputes the ordering afterwards.
    fn mutate_ords<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Vec<(Pair, Ordering)>),
    {
        f(&mut self.ords);
        self.sort_state = compute_ordering(&self.items, &self.ords);
    }
}

fn view_info(items: &[String], info: &SortState, keyboard: KeyboardState) -> Html<Model> {
    let p = match info.next_missing_ord {
        None => return html! {},
        Some(p) => p,
    };
    let left = items[p.0].clone();
    let right = items[p.1].clone();
    html! {
    <div id="info">
        <button id="left"
                class=if keyboard.left == KeyState::Pressed { "pressed" } else { "idle "}
                onclick=|_| Msg::Rank(p, Ordering::Greater)>  {left} </button>
        <button id="right"
                class=if keyboard.right == KeyState::Pressed { "pressed" } else { "idle "}
                onclick=|_| Msg::Rank(p, Ordering::Less)>     {right} </button>
    </div>
    }
}

fn view_items(items: &[String], order: &[usize]) -> Html<Model> {
    html! {
      <div id="items">
        <ol id="ordered">
        { for order.iter().rev().map(|&idx| view_item(&items[idx])) }
        </ol>
      </div>
    }
}

fn view_item(item: &str) -> Html<Model> {
    html! {
        <li> { item } </li>
    }
}

fn compute_ordering(items: &[String], ords: &[(Pair, Ordering)]) -> SortState {
    let mut cmps: HashMap<Pair, Ordering> = HashMap::new();
    for &(pair, ord) in ords {
        cmps.insert(pair, ord);
        cmps.insert(pair.reverse(), ord.reverse());
    }
    let mut xs: Vec<usize> = (0..items.len()).collect();
    let mut num_missing_ords = 0;
    let mut next_missing_ord = None;
    isort::merge_insertion_sort(&mut xs, &mut |a: usize, b: usize| {
        let p = Pair(a, b);
        match cmps.get(&p) {
            Some(&ord) => ord,
            None => {
                num_missing_ords += 1;
                if next_missing_ord.is_none() {
                    next_missing_ord = Some(p);
                }
                Ordering::Less
            }
        }
    });
    SortState {
        num_missing_ords,
        next_missing_ord,
        current_order: xs,
    }
}
