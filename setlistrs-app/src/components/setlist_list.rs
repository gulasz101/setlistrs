use gloo_net::http::Request;

use setlistrs_types::SetlistList as SetlistListModel;
use web_sys::{HtmlElement, MouseEvent};
use yew::{
    classes, function_component, html, use_effect_with_deps, use_reducer, use_state, AttrValue,
    Callback, Html, Properties, Reducible, TargetCast,
};
use yew_router::prelude::use_navigator;

use crate::app::Route;

#[function_component(SetlistList)]
pub fn setlist_list() -> Html {
    let setlists = use_state(|| None);

    {
        let setlists = setlists.clone();
        use_effect_with_deps(
            move |_| {
                let setlists = setlists.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("http://127.0.0.1:8081/setlists").send().await;
                    match response {
                        Ok(response) => {
                            let possible_setlists: Result<SetlistListModel, _> =
                                response.json().await;
                            match possible_setlists {
                                Ok(s) => setlists.set(Some(s)),
                                Err(_) => panic!(), // TODO -> handle it gracefuly
                            }
                        }
                        Err(_) => panic!(), // TODO -> handle it gracefuly
                    };
                });

                || ()
            },
            (),
        );
    }
    html! {
        <article>
        {
            match &*setlists {
                Some(setlists) => {
                    let actual_setlist: Vec<SingleSetlist> = setlists
                        .data
                        .iter()
                        .map(|( id, display_title )| SingleSetlist {
                            id: id.clone(),
                            display_title: AttrValue::from(display_title.clone())
                        })
                    .collect();

                    html! { <SetlistListContent setlists={actual_setlist} /> }
                },
                None => html! { <p> { "Fetching data..." } </p> }
            }
        }
        </article>
    }
}

#[derive(PartialEq)]
struct SingleSetlist {
    pub id: i64,
    pub display_title: AttrValue,
}

#[derive(Properties, PartialEq)]
struct SetlistListContentProps {
    pub setlists: Vec<SingleSetlist>,
}

enum SetlistDeleteAction {
    Delete(i64),
}
struct Setlists {
    pub setlists: Vec<SingleSetlist>,
}
impl Reducible for Setlists {
    type Action = SetlistDeleteAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            SetlistDeleteAction::Delete(setlist_id) => {
                let list = self
                    .setlists
                    .iter()
                    .filter(|single_setlist| single_setlist.id != setlist_id)
                    .map(|single_setlist| SingleSetlist {
                        id: single_setlist.id,
                        display_title: single_setlist.display_title.clone(),
                    })
                    .collect();

                Setlists { setlists: list }.into()
            }
        }
    }
}

#[function_component(SetlistListContent)]
fn setlist_list_content(SetlistListContentProps { setlists }: &SetlistListContentProps) -> Html {
    let lorem: Vec<SingleSetlist> = setlists
        .into_iter()
        .map(|single_setlist| SingleSetlist {
            id: single_setlist.id,
            display_title: single_setlist.display_title.clone(),
        })
        .collect();

    let owned_setlists = use_reducer(|| Setlists { setlists: lorem });
    let on_details_click = {
        let navigator = use_navigator().expect("There is no reason this would not work");
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let navigator = navigator.clone();
            let button: HtmlElement = e.target_unchecked_into();

            match button.get_attribute("data-setlist-id") {
                Some(setlist_id) => navigator.push(&Route::SetlistDetails {
                    id: setlist_id
                        .parse::<i64>()
                        .expect("id always will be there as numeric"),
                }),
                None => panic!(),
            };
        })
    };

    // let deleted: UseStateHandle<Option<String>> = use_state(|| None);

    let on_delete_click = {
        let owned_setlists = owned_setlists.clone();
        // let deleted = deleted.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let button: HtmlElement = e.target_unchecked_into();

            match button.get_attribute("data-setlist-id") {
                Some(setlist_id) => {
                    let owned_setlists = owned_setlists.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let response = Request::delete(
                            format!("http://127.0.0.1:8081/setlists/{}", setlist_id).as_str(),
                        )
                        .send()
                        .await;
                        match response {
                            Ok(response) => {
                                if response.status() == 204 {
                                    owned_setlists.dispatch(SetlistDeleteAction::Delete(
                                        setlist_id.parse::<i64>().expect(""),
                                    ));
                                }
                            }
                            Err(_) => panic!(), // TODO -> handle it gracefuly
                        };
                    });
                }
                None => panic!(),
            };
        })
    };

    html! {
        <table>
            <caption>{"Setlists"}</caption>
            <thead class={classes!("grid")}>
                <th>{"Display title"}</th>
                <th>{"Action"}</th>
            </thead>
            <tbody>
            {
            for owned_setlists.setlists.iter().map(|single_setlist| html! {
                <tr class={classes!("grid")}>
                    <td class={classes!("grid")}>{&single_setlist.display_title}</td>
                    <td class={classes!("grid")}>
                    <button onclick={on_details_click.clone()} data-setlist-id={single_setlist.id.to_string()}>{"Details / Edit"}</button>
                    <button onclick={on_delete_click.clone()} class={classes!("red-bg-bd")} data-setlist-id={single_setlist.id.to_string()}>{"Delete"}</button>
                    </td>
                </tr>
            })
            }
            </tbody>
        </table>
    }
}
