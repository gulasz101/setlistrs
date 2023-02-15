use gloo_net::http::Request;
use setlistrs_types::NewSetlist;
use web_sys::{FormData, SubmitEvent};
use yew::callback::Callback;
use yew::{function_component, html, Html, Properties, Reducible, TargetCast};
use yew_router::prelude::use_navigator;

use crate::app::Route;

pub enum SongChockboxAction {
    Add(i64),
    Remove(i64),
}

pub struct SongsForSetlist {
    pub data: Vec<i64>,
}

impl Reducible for SongsForSetlist {
    type Action = SongChockboxAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            SongChockboxAction::Add(value) => {
                let mut list = self.data.clone();

                list.push(value);

                SongsForSetlist { data: list }.into()
            }
            SongChockboxAction::Remove(value) => {
                let mut list = self.data.clone();
                list.retain(|song_id| *song_id != value);
                SongsForSetlist { data: list }.into()
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SetlistAddProps {
    pub songs: Vec<i64>,
    pub disabled: bool,
}

#[function_component(SetlistAdd)]
pub fn setlist_add(props: &SetlistAddProps) -> Html {
    let navigator = use_navigator().expect("There is no reason it will not work.");
    let on_setlist_submit = {
        let songs_for_setlist = props.songs.clone();

        Callback::from(move |e: SubmitEvent| {
            let navigator = navigator.clone();
            e.prevent_default();
            let form_data = FormData::new_with_form(&e.target_unchecked_into())
                .expect("This is going to work since we are sure form exists.");

            let new_setlist = NewSetlist {
                display_title: match form_data.get("display_title").as_string() {
                    Some(display_title) => display_title,
                    None => panic!(),
                },
                songs: songs_for_setlist.clone(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("http://127.0.0.1:8081/setlists")
                    .json(&new_setlist)
                    .expect("This will work")
                    .send()
                    .await;

                match response {
                    Ok(possible_setlist_id) => {
                        let setlist_id: Result<i64, _> = possible_setlist_id.json().await;
                        match setlist_id {
                            Ok(id) => navigator.push(&Route::Setlist { id }),
                            Err(_) => panic!(),
                        }
                    }
                    Err(_) => panic!(),
                };
            });
        })
    };
    html! {
        <form onsubmit={ on_setlist_submit }>
            <fieldset disabled={ props.disabled }>
                <label for="display_title">{"Setlist name"}</label>
                <input type="text" name="display_title" id="display_title"/>
                <button type="submit">
                    {"Create setlist from checked songs"}
                </button>
            </fieldset>
        </form>
    }
}
