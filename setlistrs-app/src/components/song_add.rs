use std::ops::Deref;

use gloo_net::http::Request;
use setlistrs_types::{Song, YTLink};
use web_sys::{Element, FormData};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::app::Route;

#[derive(Clone, PartialEq, Properties)]
struct LinkWithDisplayTitleProps {
    pub input_name_prefix: String,
    pub entries_list: LinkWithDisplayTitleIdList,
    pub on_add: Callback<()>,
    pub on_remove: Callback<i32>,
    pub min_entries_count: Option<i32>,
}

#[function_component(LinkWithDisplayTitle)]
fn link_with_display_title(
    LinkWithDisplayTitleProps {
        input_name_prefix,
        entries_list,
        on_add,
        on_remove,
        min_entries_count,
    }: &LinkWithDisplayTitleProps,
) -> Html {
    let on_plus_click = {
        let on_add = on_add.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_add.emit(())
        })
    };

    let on_minus_click = {
        let on_remove = on_remove.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let button: Element = e.target_unchecked_into();

            on_remove.emit(
                button
                    .get_attribute("data-entity_id")
                    .expect("There is always a number here")
                    .parse::<i32>()
                    .expect("There is always a number here"),
            );
        })
    };

    let display_minus_button = min_entries_count.is_none()
        || (min_entries_count.is_some()
            && min_entries_count.expect("Value is hardcoded")
                < entries_list.list.len().try_into().unwrap());

    html! {
        <>

        {
            for entries_list.list.iter().map(|entry_id| html! {
                <div class={classes!("grid")}>
                    <input name={ format!("{}_display_title_{}", input_name_prefix, entry_id) } placeholder={ "Title" } />
                    <input name={ format!("{}_url_{}", input_name_prefix, entry_id) } placeholder={ "Url" } />
                    {
                        if display_minus_button {
                            html! {
                            <button data-entity_id={entry_id.to_string()} title={"Remove link"} class={classes!("red-bg-bd")} onclick={on_minus_click.clone()}>{"-"}</button>
                        }
                        } else { html!{""} }
                    }
                </div>
            })
        }

        <button title={"Add one more link"} onclick={on_plus_click}>{"+"}</button>
        </>
    }
}

enum LinkWithDisplayTitleAction {
    Add(),
    Remove(i32),
}

#[derive(Clone, PartialEq)]
struct LinkWithDisplayTitleIdList {
    pub list: Vec<i32>,
    pub last_added: i32,
}

impl Reducible for LinkWithDisplayTitleIdList {
    type Action = LinkWithDisplayTitleAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            LinkWithDisplayTitleAction::Add() => {
                let mut list = self.list.clone();
                let new_last_added_id = self.last_added + 1;

                list.push(new_last_added_id);

                LinkWithDisplayTitleIdList {
                    last_added: new_last_added_id,
                    list,
                }
            }
            LinkWithDisplayTitleAction::Remove(list_entry_id) => {
                let mut list = self.list.clone();
                let last_added = self.last_added;

                list.retain(|list_entry| *list_entry != list_entry_id);

                LinkWithDisplayTitleIdList { last_added, list }
            }
        }
        .into()
    }
}

#[function_component(SongAdd)]
pub fn add_song_form() -> Html {
    let yt_links_state = use_reducer(|| LinkWithDisplayTitleIdList {
        last_added: 1,
        list: Vec::from([1]),
    });
    let covers_links_state = use_reducer(|| LinkWithDisplayTitleIdList {
        last_added: 1,
        list: Vec::from([1]),
    });
    let navigator = use_navigator().expect("There is no reason it will not work.");

    let on_yt_link_add = {
        let yt_links_state = yt_links_state.clone();
        Callback::from(move |_: ()| yt_links_state.dispatch(LinkWithDisplayTitleAction::Add()))
    };
    let on_yt_link_rm = {
        let yt_links_state = yt_links_state.clone();
        Callback::from(move |entity_id: i32| {
            yt_links_state.dispatch(LinkWithDisplayTitleAction::Remove(entity_id))
        })
    };

    let on_cover_add = {
        let covers_links_state = covers_links_state.clone();
        Callback::from(move |_: ()| covers_links_state.dispatch(LinkWithDisplayTitleAction::Add()))
    };
    let on_cover_rm = {
        let covers_links_state = covers_links_state.clone();
        Callback::from(move |entity_id: i32| {
            covers_links_state.dispatch(LinkWithDisplayTitleAction::Remove(entity_id))
        })
    };

    let onsubmit = {
        let yt_links_state = yt_links_state.clone();
        let covers_links_state = covers_links_state.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            let navigator = navigator.clone();
            e.prevent_default();

            let form_data: FormData = FormData::new_with_form(&e.target_unchecked_into())
                .expect("This will work since we have only one form.");

            let yt_links: Vec<YTLink> = yt_links_state
                .deref()
                .list
                .iter()
                .map(|link_id| YTLink {
                    url: match form_data.get(&format!("yt_url_{}", link_id)).as_string() {
                        Some(url) => url.into(),
                        None => todo!(),
                    },
                    display_title: form_data
                        .get(&format!("yt_display_title_{}", link_id))
                        .as_string(),
                })
                .collect();

            let cover_links: Vec<YTLink> = covers_links_state
                .deref()
                .list
                .iter()
                .map(|link_id| YTLink {
                    url: match form_data.get(&format!("cover_url_{}", link_id)).as_string() {
                        Some(url) => url.into(),
                        None => todo!(),
                    },
                    display_title: form_data
                        .get(&format!("cover_display_title_{}", link_id))
                        .as_string(),
                })
                .collect();

            wasm_bindgen_futures::spawn_local(async move {
                let _response = Request::post("http://127.0.0.1:8081/songs")
                    .json(&Song {
                        name: form_data.get("song_title").as_string().unwrap(),
                        source: yt_links,
                        cover: Some(cover_links),
                        chords: form_data.get("chords").as_string().unwrap(),
                    })
                    .expect("This will work")
                    .send()
                    .await;

                navigator.push(&Route::SongList);
            });
        })
    };

    html! {
    <article>
        <form onsubmit={onsubmit}>
            <input name="song_title" placeholder={"song title"} />
            <fieldset>
                <legend>{"Sources links"}</legend>
                <LinkWithDisplayTitle
                    input_name_prefix={"yt"}
                    entries_list={(*yt_links_state).clone()}
                    on_add={on_yt_link_add.clone()}
                    on_remove={on_yt_link_rm.clone()}
                    min_entries_count={1}
                />
            </fieldset>
            <fieldset>
                <legend>{"Covers"}</legend>
                <LinkWithDisplayTitle
                    input_name_prefix={"cover"}
                    entries_list={(*covers_links_state).clone()}
                    on_add={on_cover_add.clone()}
                    on_remove={on_cover_rm.clone()}
                />
            </fieldset>
            <input name="chords" placeholder={"chords, ex: b G D A"} />

            <button type={"submit"} >{ "Add new song" }</button>
        </form>
    </article>
        }
}
