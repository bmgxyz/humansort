use std::rc::Rc;

use humansort_lib::HumansortState;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq)]
struct AppState {
    current_view: AppView,
    humansort_state: HumansortState,
}

#[derive(PartialEq, Clone)]
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
            Action::AddItem { name } => {
                let mut humansort_state = self.humansort_state.clone();
                humansort_state.push_item(name);
                AppState {
                    current_view: self.current_view.clone(),
                    humansort_state,
                }
                .into()
            }
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
struct InputItemProps {
    state: UseReducerHandle<AppState>,
    value: String,
}

#[function_component]
fn InputItem(props: &InputItemProps) -> Html {
    let InputItemProps { state, value } = props;
    html! {
        <li>{ value }</li>
    }
}

#[derive(Properties, PartialEq)]
struct ViewProps {
    state: UseReducerHandle<AppState>,
}

#[function_component]
fn InputView(props: &ViewProps) -> Html {
    let ViewProps { state } = props;
    let change_view_sorting = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::ChangeView {
                new_view: AppView::Sorting,
            })
        })
    };
    let onkeypress = {
        let state = state.clone();
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let target: HtmlInputElement = e.target_unchecked_into();
                let value = target.value();
                state.dispatch(Action::AddItem { name: value });
                target.set_value("");
            }
        }
    };
    html! {
        <div>
            <div>
                <ul>
                    { for state.humansort_state.get_all_items().iter().map(|item|
                        html! {
                            <InputItem state={props.state.clone()} value={item.to_string()} />
                        }
                    ) }
                </ul>
            </div>
            <input type="text" {onkeypress} />
            <div>
                <button onclick={change_view_sorting}>{ "Start sorting" }</button>
            </div>
        </div>
    }
}

#[function_component]
fn SortingView(props: &ViewProps) -> Html {
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
fn OutputView(props: &ViewProps) -> Html {
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
                    AppView::Input => html! { <InputView {state} /> },
                    AppView::Sorting => html! { <SortingView {state} /> },
                    AppView::Output => html! { <OutputView {state} /> },
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
