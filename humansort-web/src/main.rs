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
                humansort_state.add_item(&name);
                AppState {
                    current_view: self.current_view.clone(),
                    humansort_state,
                }
                .into()
            }
            Action::RenameItem { old_name, new_name } => {
                let mut humansort_state = self.humansort_state.clone();
                humansort_state.rename_item(&old_name, &new_name).unwrap();
                AppState {
                    current_view: self.current_view.clone(),
                    humansort_state,
                }
                .into()
            }
            Action::RemoveItem { name } => {
                let mut humansort_state = self.humansort_state.clone();
                humansort_state.remove_item(&name).unwrap();
                AppState {
                    current_view: self.current_view.clone(),
                    humansort_state,
                }
                .into()
            }
            Action::SelectPreference { winner, others } => {
                let mut preferences = others;
                preferences.insert(0, winner);
                let mut humansort_state = self.humansort_state.clone();
                humansort_state.update(&preferences).unwrap();
                AppState {
                    current_view: self.current_view.clone(),
                    humansort_state,
                }
                .into()
            }
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
    let editing = use_state(|| false);
    let editing_value = use_state(|| value.to_string());
    let onremove = {
        let state = state.clone();
        let value = value.clone();
        Callback::from(move |_| {
            state.dispatch(Action::RemoveItem {
                name: value.to_string(),
            })
        })
    };
    let onedit = {
        let editing = editing.clone();
        Callback::from(move |_| {
            editing.set(true);
        })
    };
    let onkeypress = {
        let state = state.clone();
        let old_value = value.clone();
        let editing = editing.clone();
        let editing_value = editing_value.clone();
        move |e: KeyboardEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            let new_value = target.value();
            editing_value.set(new_value.clone());
            if e.key() == "Enter" {
                state.dispatch(Action::RenameItem {
                    old_name: old_value.to_string(),
                    new_name: new_value,
                });
                editing.set(false);
            }
        }
    };
    // TODO: replace text in buttons with icons
    html! {
        <li>
            {
                if *editing {
                    html! {
                        <input type="text" {onkeypress} value={editing_value.to_string()} />
                    }
                }
                else {
                    html! {
                        <>
                            { value }
                            <button onclick={onedit}>{ "edit" }</button>
                        </>
                    }
                }
            }
            <button onclick={onremove}>{ "remove" }</button>
        </li>
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
        // TODO: check that there are enough items before changing the view
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

#[derive(Properties, PartialEq)]
struct SortingItemProps {
    winner: String,
    others: Vec<String>,
    items_to_sort_setter: UseStateSetter<Vec<String>>,
    state: UseReducerHandle<AppState>,
}

#[function_component]
fn SortingItem(props: &SortingItemProps) -> Html {
    let SortingItemProps {
        winner,
        others,
        items_to_sort_setter,
        state,
    } = props;
    let onclick = {
        let state = state.clone();
        let winner = winner.clone();
        let others = others.clone();
        let items_to_sort_setter = items_to_sort_setter.clone();
        Callback::from(move |_| {
            state.dispatch(Action::SelectPreference {
                winner: winner.to_string(),
                others: others.to_vec(),
            });
            items_to_sort_setter.set(state.humansort_state.next().unwrap());
        })
    };
    html! {
        <li>
            <button onclick={onclick}>{ winner }</button>
        </li>
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
    let items_to_sort = use_state(|| state.humansort_state.next().unwrap());
    html! {
        <div>
            <div>
                <ol>
                    { for items_to_sort.iter().enumerate().map(|(idx, item)| {
                        let mut others = (*items_to_sort).clone();
                        others.remove(idx);
                        html! {
                            <SortingItem
                                winner={item.to_string()}
                                {others}
                                items_to_sort_setter={items_to_sort.setter()}
                                state={state.clone()}
                            />
                        }
                    }
                    ) }
                </ol>
            </div>
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
            <div>
                <ul>
                    { for state.humansort_state.get_all_items().iter().map(|item|
                        html! {
                            <li>{ item.to_string() }</li>
                        }
                    ) }
                </ul>
            </div>
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
