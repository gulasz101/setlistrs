use gloo_net::http::Request;
use setlistrs_types::Setlist;
use yew::prelude::*;

#[function_component(AllSongsList)]
pub fn all_songs_list() -> Html {
    let setlist = use_state(|| None);

    {
        let setlist = setlist.clone();
        use_effect_with_deps(
            move |_| {
                let setlist = setlist.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("http://127.0.0.1:8081/songs").send().await;
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
            <x-grid columns="8" class="margin20">
            <x-cell span="row" span-s="row">
                <table>
                            <caption>{ "Setlist" }</caption>
                <thead>
                    <th>{ "Song" }</th>
                    <th>{ "Cover" }</th>
                    <th>{ "Chords" }</th>
                </thead>
                <tbody>
                {match &*setlist {
                    Some(setlist) => {
                        html! {

                 for setlist.data.iter().map(|song|
                    html! {
                        <tr>
                            <td>{ song.name.clone() }</td>
                            <td>{ match &song.cover {
                                Some(cover) => html! {<a href={ cover.url.clone() } target="_blank">{ cover.description.clone() }</a>},
                                None => "".into(),
                                }
                            }</td>
                            <td>{ song.chords.clone() }</td>
                        </tr>
                    })

                        }
                    },
                    None => html! {
    <tr>
        <td>{"No data"}</td>
        </tr>
                    },
                }
                }
                </tbody>
                </table>
            </x-cell>
            </x-grid>
        }
}
