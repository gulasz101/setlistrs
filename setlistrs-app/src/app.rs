use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{add_song_form::AddSongForm, all_songs_list::AllSongsList};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    AllSongsList,
    #[at("/add-song")]
    AddSong,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::AllSongsList => html! { <AllSongsList /> },
        Route::AddSong => html! { <AddSongForm/> },
        Route::NotFound => html! { <h1> {"404"} </h1> },
    }
}

#[function_component(NavItems)]
pub fn nav_items() -> Html {
    let navigator = use_navigator().unwrap();

    let all_songs_list_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::AllSongsList));
        html! {
            <button {onclick}>{ "To all songs list" }</button>
        }
    };

    let add_song_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::AddSong));
        html! {
            <button {onclick}>{ "To add new song form" }</button>
        }
    };

    html! {
            <ul>
            <li>{ all_songs_list_button }</li>
            <li>{ add_song_button }</li>
    // <Link<Route> to={Route::AllSongsList}>{ "-> To all songs list." }</Link<Route>>
    // <Link<Route> to={Route::AddSong}>{ "-> To add song form." }</Link<Route>>
            </ul>
        }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div id="app">
        <main>

        <BrowserRouter>
        <NavItems />
           <Switch<Route> render={switch} />
        </BrowserRouter>

        </main>
        </div>
    }
}
