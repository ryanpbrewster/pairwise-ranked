#![recursion_limit = "128"]
#![allow(dead_code)]

use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use std::collections::HashMap;
use std::cmp::Ordering;

pub struct Model {
    console: ConsoleService,
    items: Vec<String>,
    cmps: HashMap<Pair, Ordering>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Pair(String, String);
impl Pair {
    fn new(a: String, b: String) -> Pair {
        if a <= b {
            Pair(a, b)
        } else {
            Pair(b, a)
        }
    }
}

pub enum Msg {
    Step,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let items = vec!["red", "blue", "green", "yellow", "white", "rainbow"]
            .into_iter()
            .map(String::from)
            .collect();
        Model {
            console: ConsoleService::new(),
            items,
            cmps: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Step => {
                self.console.log("step");
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        let mut ordered: Vec<i32> = (0 .. self.items.len() as i32).collect();
        let mut cmp_fn = |a: i32, b: i32| {
            a.cmp(&b)
        };
        isort::merge_insertion_sort(ordered.as_mut_slice(), &mut cmp_fn);
        html! {
            <div id="main">
            {view_items(&self.items)}
            </div>
        }
    }
}

fn view_items(items: &[String]) -> Html<Model> {
    html! {
      <div id="items">
        <ol id="ordered">
        { for items.iter().map(|item| view_item(item)) }
        </ol>
      </div>
    }
}

fn view_item(item: &str) -> Html<Model> {
    html! {
        <li> { item } </li>
    }
}
