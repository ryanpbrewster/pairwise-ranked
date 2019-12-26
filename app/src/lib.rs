#![recursion_limit = "128"]
#![allow(dead_code)]

use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use std::collections::HashMap;
use std::cmp::Ordering;
use isort::Pair;

pub struct Model {
    console: ConsoleService,
    items: Vec<String>,
    ords: Vec<(Pair, Ordering)>,
    need_to_know: Option<Pair>,
    ordered: Permutation,
}

type Permutation = Vec<usize>;

pub enum Msg {
    Rank(Pair, Ordering),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let items: Vec<String> = vec!["red", "blue", "green", "yellow", "white", "rainbow"]
            .into_iter()
            .map(String::from)
            .collect();
        let ords = Vec::new();
        let (ordered, need_to_know) = compute_ordering(&items, &ords);
        Model {
            console: ConsoleService::new(),
            items,
            ords,
            ordered,
            need_to_know,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Rank(pair, cmp) => {
                self.console.log(&format!("{} {:?} {}", self.items[pair.first()], cmp, self.items[pair.second()]));
                self.ords.push((pair, cmp));
                let (ordered, need_to_know) = compute_ordering(&self.items, &self.ords);
                self.ordered = ordered;
                self.need_to_know = need_to_know;
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div id="main">
            { view_info(&self.items, self.need_to_know) }
            { view_items(&self.items, &self.ordered) }
            </div>
        }
    }
}

fn view_info(items: &[String], info: Option<Pair>) -> Html<Model> {
    match info {
        None => html! { "done" },
        Some(p) => {
            let a = items[p.first()].clone();
            let b = items[p.second()].clone();
            html! {
            <div id="info">
                <button onclick=|_| Msg::Rank(p, Ordering::Greater)>  {a} </button>
                <button onclick=|_| Msg::Rank(p, Ordering::Less)>     {b} </button>
            </div>
            }
        },
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
    let mut xs: Vec<usize> = (0 .. items.len()).collect();
    let need_to_know = isort::merge_insertion_sort(&mut xs, &mut |a: usize, b: usize| {
        let p = Pair(a, b);
        cmps.get(&p).cloned().ok_or(p)
    }).err();
    (xs, need_to_know)
}
