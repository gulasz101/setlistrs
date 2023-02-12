use gloo_net::http::Request;
use setlistrs_types::{Setlist, SetlistSong};
use yew::{function_component, html, use_effect_with_deps, use_state, AttrValue, Html, Properties};

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub seed: i64,
}

#[function_component(SetlistDetails)]
pub fn setlist_details(props: &Props) -> Html {
    let setlist = use_state(|| None);
    let setlist_id = props.seed;
    {
        let setlist = setlist.clone();
        use_effect_with_deps(
            move |_| {
                let setlist = setlist.clone();
                let setlist_id = setlist_id.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get(
                        format!("http://127.0.0.1:8081/setlists/{}", setlist_id).as_str(),
                    )
                    .send()
                    .await;
                    match response {
                        Ok(response) => {
                            let possible_setlist: Result<Setlist, _> = response.json().await;
                            match possible_setlist {
                                Ok(s) => setlist.set(Some(s)),
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
        match &*setlist {
            Some(setlist) => {
                let songs: Vec<(AttrValue, AttrValue)> = setlist.songs
                    .iter()
                    .map(|(_song_id, song)| (
                            AttrValue::from(song.display_title.clone()),
                            AttrValue::from(song.chords.clone())
                            )
                        )
                    .collect();
                html! { <SetlistDetailsContent display_title={setlist.display_title.clone()} songs={songs} />}
            },
            None=> html! { <p> { "Fetching data..." } </p> }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct SetlistDetailsContentProps {
    pub display_title: AttrValue,
    pub songs: Vec<(AttrValue, AttrValue)>,
}
#[function_component(SetlistDetailsContent)]
pub fn setlist_details_content(props: &SetlistDetailsContentProps) -> Html {
    html! {
    <table>
        <caption> { &props.display_title } </caption>
        <thead>
        <th>{"Song"}</th>
        <th>{"Chords"}</th>
        </thead>
        <tbody>
        {
            for props.songs
                .iter()
                .map(|(display_title, chords)| html! {
                <tr>
                    <td>{display_title}</td>
                    <td>{chords}</td>
                </tr>
            })
        }
        </tbody>
    </table>
    }
}
