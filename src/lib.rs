extern crate kagura;
extern crate wasm_bindgen;

use kagura::Attributes;
use kagura::Events;
use kagura::Html;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(kagura::Component::new(init(), update, render), "app");
}

struct State {
    name: String,
    dialog: Dialog,
}

fn init() -> State {
    State {
        name: String::from(""),
        dialog: Dialog::None,
    }
}

enum Dialog {
    None,
    InputName,
}

enum Msg {
    SetName(String),
    ShowInputNameDialog,
    CloseDialog,
}

struct Sub();

fn update(state: &mut State, msg: &Msg) -> Option<Sub> {
    match msg {
        Msg::SetName(name) => {
            state.name = name.clone();
            if let Dialog::InputName = state.dialog {
                state.dialog = Dialog::None
            }
        }
        Msg::ShowInputNameDialog => state.dialog = Dialog::InputName,
        Msg::CloseDialog => state.dialog = Dialog::None,
    }
    None
}

fn render(state: &State) -> Html<Msg> {
    match state.dialog {
        Dialog::None => render_no_dialog(state),
        Dialog::InputName => redner_dialog(state),
    }
}

fn render_no_dialog(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().id("app"),
        Events::new(),
        vec![render_content(state)],
    )
}

fn redner_dialog(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().id("app"),
        Events::new(),
        vec![
            render_content(state),
            Html::div(
                Attributes::new()
                    .class("modal-backdrop")
                    .class("fade")
                    .class("show"),
                Events::new(),
                vec![],
            ),
            Html::div(
                Attributes::new()
                    .class("modal")
                    .class("fade")
                    .class("show")
                    .style("display", "block"),
                Events::new(),
                vec![Html::div(
                    Attributes::new()
                        .class("modal-dialog")
                        .class("modal-dialog-centered"),
                    Events::new(),
                    vec![Html::component(input_name_dialog::new().subscribe(
                        |sub| match sub {
                            input_name_dialog::Sub::Ok(name) => Box::new(Msg::SetName(name)),
                            input_name_dialog::Sub::Cancel => Box::new(Msg::CloseDialog),
                        },
                    ))],
                )],
            ),
        ],
    )
}

fn render_content(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().class("jumbotron"),
        Events::new(),
        vec![
            Html::h1(
                Attributes::new().class("display-4"),
                Events::new(),
                vec![Html::unsafe_text(
                    String::from("Hello ") + &state.name + " !",
                )],
            ),
            Html::button(
                Attributes::new()
                    .class("btn")
                    .class("btn-primary")
                    .class("btn-lg")
                    .type_("button"),
                Events::new().on_click(|_| Msg::ShowInputNameDialog),
                vec![Html::unsafe_text("Input name")],
            ),
        ],
    )
}

mod input_name_dialog {
    use kagura::Attributes;
    use kagura::Events;
    use kagura::Html;

    pub fn new() -> kagura::Component<Msg, State, Sub> {
        kagura::Component::new(String::from(""), update, render)
    }

    pub type State = String;

    pub enum Msg {
        InputName(String),
        Ok,
        Cancel,
    }

    pub enum Sub {
        Ok(String),
        Cancel,
    }

    fn update(state: &mut State, msg: &Msg) -> Option<Sub> {
        match msg {
            Msg::InputName(name) => {
                *state = name.clone();
                None
            }
            Msg::Ok => Some(Sub::Ok(state.clone())),
            Msg::Cancel => Some(Sub::Cancel),
        }
    }

    fn render(state: &State) -> Html<Msg> {
        Html::div(
            Attributes::new().class("modal-content"),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class("modal-header"),
                    Events::new(),
                    vec![Html::h5(
                        Attributes::new().class("modal-title"),
                        Events::new(),
                        vec![Html::unsafe_text("名前を入力してください")],
                    )],
                ),
                Html::div(
                    Attributes::new().class("modal-body"),
                    Events::new(),
                    vec![Html::input(
                        Attributes::new().class("form-control").value(state),
                        Events::new().on_input(|v| Msg::InputName(v)),
                        vec![],
                    )],
                ),
                Html::div(
                    Attributes::new().class("modal-footer"),
                    Events::new(),
                    vec![
                        Html::button(
                            Attributes::new()
                                .class("btn")
                                .class("btn-secondary")
                                .type_("button"),
                            Events::new().on_click(|_| Msg::Cancel),
                            vec![Html::unsafe_text("Cancel")],
                        ),
                        Html::button(
                            Attributes::new()
                                .class("btn")
                                .class("btn-primary")
                                .type_("button"),
                            Events::new().on_click(|_| Msg::Ok),
                            vec![Html::unsafe_text("Ok")],
                        ),
                    ],
                ),
            ],
        )
    }
}
