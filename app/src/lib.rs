#![recursion_limit = "128"]
#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use stdweb::js;
use yew::services::ConsoleService;
use yew::{
    html, Component, ComponentLink, Html, IKeyboardEvent, KeyDownEvent, KeyUpEvent, ShouldRender,
};

pub struct Model {
    link: ComponentLink<Self>,
    console: ConsoleService,
    items: Vec<String>,
    ords: Vec<(Pair, Ordering)>,
    need_to_know: Option<Pair>,
    ordered: Permutation,
    keyboard_state: KeyboardState,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Pair(pub usize, pub usize);
impl Pair {
    pub fn reverse(&self) -> Pair {
        Pair(self.1, self.0)
    }
}

type Permutation = Vec<usize>;

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct KeyboardState {
    left: KeyState,
    right: KeyState,
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
    Rank(Pair, Ordering),
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let items: Vec<String> = vec!["red", "blue", "green", "yellow", "white", "rainbow"]
            .into_iter()
            .map(String::from)
            .collect();
        let ords = Vec::new();
        let (ordered, need_to_know) = compute_ordering(&items, &ords);

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
            console: ConsoleService::new(),
            link,
            items,
            ords,
            ordered,
            need_to_know,
            keyboard_state: KeyboardState::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Rank(pair, cmp) => {
                self.ords.push((pair, cmp));
                let (ordered, need_to_know) = compute_ordering(&self.items, &self.ords);
                self.ordered = ordered;
                self.need_to_know = need_to_know;
            }
            Msg::KeyDown(evt) => {
                match evt.key().as_ref() {
                    "ArrowLeft" => self.keyboard_state.left = KeyState::Pressed,
                    "ArrowRight" => self.keyboard_state.right = KeyState::Pressed,
                    _ => {}
                };
            }
            Msg::KeyUp(evt) => {
                match evt.key().as_ref() {
                    "ArrowLeft" => {
                        self.keyboard_state.left = KeyState::Idle;
                        if let Some(pair) = self.need_to_know {
                            self.link.send_self(Msg::Rank(pair, Ordering::Greater));
                        }
                    }
                    "ArrowRight" => {
                        self.keyboard_state.right = KeyState::Idle;
                        if let Some(pair) = self.need_to_know {
                            self.link.send_self(Msg::Rank(pair, Ordering::Less));
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
            { view_info(&self.items, self.need_to_know, self.keyboard_state) }
            { view_items(&self.items, &self.ordered) }
            { format!("{} choices in", self.ords.len()) }
            </div>
        }
    }
}

fn view_info(items: &[String], info: Option<Pair>, keyboard: KeyboardState) -> Html<Model> {
    let p = match info {
        None => return html! { "done" },
        Some(p) => p,
    };
    let left = items[p.0].clone();
    let right = items[p.1].clone();
    html! {
    <div id="info">
        <button class=if keyboard.left == KeyState::Pressed { "pressed" } else { "idle "}
                onclick=|_| Msg::Rank(p, Ordering::Greater)>  {left} </button>
        <button class=if keyboard.right == KeyState::Pressed { "pressed" } else { "idle "}
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

fn compute_ordering(items: &[String], ords: &[(Pair, Ordering)]) -> (Permutation, Option<Pair>) {
    let mut cmps: HashMap<Pair, Ordering> = HashMap::new();
    for &(pair, ord) in ords {
        cmps.insert(pair, ord);
        cmps.insert(pair.reverse(), ord.reverse());
    }
    let mut xs: Vec<usize> = (0..items.len()).collect();
    let mut need_to_know = None;
    isort::merge_insertion_sort(&mut xs, &mut |a: usize, b: usize| {
        let p = Pair(a, b);
        match cmps.get(&p) {
            Some(&ord) => ord,
            None => {
                if need_to_know.is_none() {
                    need_to_know = Some(p);
                }
                Ordering::Equal
            }
        }
    });
    (xs, need_to_know)
}
