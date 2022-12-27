use std::rc::Rc;

use humansort_lib::HumansortState;
use yew::prelude::*;

#[derive(PartialEq)]
struct AppState {
    current_view: AppView,
    humansort_state: HumansortState,
}

#[derive(PartialEq)]
enum AppView {
    Input,
    Sorting,
    Output,
}

enum Action {
    AddItem { name: String },
    RenameItem { old_name: String, new_name: String },
    RemoveItem { name: String },
    SelectPreference { winner: String, others: Vec<String> },
    ChangeView { new_view: AppView },
}

impl Reducible for AppState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::AddItem { name } => todo!(),
            Action::RenameItem { old_name, new_name } => todo!(),
            Action::RemoveItem { name } => todo!(),
            Action::SelectPreference { winner, others } => todo!(),
            Action::ChangeView { new_view } => AppState {
                current_view: new_view,
                humansort_state: self.humansort_state.clone(),
            }
            .into(),
        }
    }
}

#[derive(Properties, PartialEq)]
struct ViewProps {
    state: UseReducerHandle<AppState>,
}

#[function_component]
fn Input(props: &ViewProps) -> Html {
    let ViewProps { state } = props;
    let change_view_sorting = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::ChangeView {
                new_view: AppView::Sorting,
            })
        })
    };
    html! {
        <div>
            <p>{ "I am the Input component." }</p>
            <button onclick={change_view_sorting}>{ "Start sorting" }</button>
        </div>
    }
}

#[function_component]
fn Sorting(props: &ViewProps) -> Html {
    let ViewProps { state } = props;
    let change_view_input = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::ChangeView {
                new_view: AppView::Input,
            })
        })
    };
    let change_view_output = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::ChangeView {
                new_view: AppView::Output,
            })
        })
    };
    html! {
        <div>
            <p>{ "I am the Sorting component." }</p>
            <button onclick={change_view_input}>{ "Edit items" }</button>
            <button onclick={change_view_output}>{ "View sorted list" }</button>
        </div>
    }
}

#[function_component]
fn Output(props: &ViewProps) -> Html {
    let ViewProps { state } = props;
    let change_view_sorting = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::ChangeView {
                new_view: AppView::Sorting,
            })
        })
    };
    let change_view_input = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::ChangeView {
                new_view: AppView::Input,
            })
        })
    };
    html! {
        <div>
            <p>{ "I am the Output component." }</p>
            <button onclick={change_view_input}>{ "Edit items" }</button>
            <button onclick={change_view_sorting}>{ "Continue sorting" }</button>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let state = use_reducer(|| AppState {
        current_view: AppView::Input,
        humansort_state: HumansortState::new(),
    });

    html! {
        <div class="container">
            {
                match state.current_view {
                    AppView::Input => html! { <Input {state} /> },
                    AppView::Sorting => html! { <Sorting {state} /> },
                    AppView::Output => html! { <Output {state} /> },
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
