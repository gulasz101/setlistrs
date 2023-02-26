use std::ops::Deref;

use gloo_net::http::Request;
use setlistrs_types::{Song, YTLink};
use web_sys::FormData;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::app::Route;

#[derive(Clone, PartialEq, Properties)]
struct YtLinkProps {
    pub last_id: i32,
    pub onadd: Callback<i32>,
}

#[function_component(AddYTLink)]
fn add_yt_link(props: &YtLinkProps) -> Html {
    let on_another_link_click = {
        let onadd = props.onadd.clone();
        let last_id = props.last_id.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            onadd.emit((last_id + 1) as i32);
        })
    };
    html! {
        <div class={classes!("grid")}>
            <input name={ format!("url_title_{}", props.last_id.clone().to_string()) } placeholder={"url title"}/>
            <input name={ format!("url_{}", props.last_id.clone().to_string()) } placeholder={"yt-url"}/>
            <button title={"Another link"} onclick={on_another_link_click}>{"+"}</button>
        </div>
    }
}

enum YtLinksAddedAction {
    Add(i32),
}

struct YtLinksAdded {
    pub list: Vec<i32>,
}

impl Reducible for YtLinksAdded {
    type Action = YtLinksAddedAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            YtLinksAddedAction::Add(value) => {
                let mut list = self.list.clone();

                list.push(value);

                YtLinksAdded { list: list }.into()
            }
        }
    }
}

#[function_component(SongAdd)]
pub fn add_song_form() -> Html {
    let yt_links_state = use_reducer(|| YtLinksAdded {
        list: Vec::from([1]),
    });
    let navigator = use_navigator().expect("There is no reason it will not work.");

    let on_yt_link_add = {
        let yt_links_state = yt_links_state.clone();
        Callback::from(move |value: i32| {
            yt_links_state.dispatch(YtLinksAddedAction::Add(value));
        })
    };

    let onsubmit = {
        let yt_links_state = yt_links_state.clone();
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
                    url: match form_data.get(&format!("url_{}", link_id)).as_string() {
                        Some(url) => url.into(),
                        None => todo!(),
                    },
                    display_title: form_data.get(&format!("url_title_{}", link_id)).as_string(),
                })
                .collect();

            let cover_yt_link = YTLink {
                url: match form_data.get("cover_url").as_string() {
                    Some(value) => value,
                    None => todo!(),
                },
                display_title: form_data.get("cover_title").as_string(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                let _response = Request::post("http://127.0.0.1:8081/songs")
                    .json(&Song {
                        name: form_data.get("song_title").as_string().unwrap(),
                        source: yt_links,
                        cover: Some(vec![cover_yt_link]),
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
                {
                    for yt_links_state.list.iter().cloned().map(|last_id|
                        html! {
                            <AddYTLink last_id={last_id.clone()} onadd={on_yt_link_add.clone()} />
                        }
                    )
                }

                <input name="cover_url" placeholder={"cover url"} />
                <input name="cover_title" placeholder={"cover display title"} />

                <input name="chords" placeholder={"chords, ex: b G D A"} />

                <button type={"submit"} >{ "Add new song" }</button>
            </form>
        </article>
    }
}
