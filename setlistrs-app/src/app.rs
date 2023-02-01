use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{AddSongForm, AllSongsList};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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
            <button {onclick}>{ "All songs" }</button>
        }
    };

    let add_song_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::AddSong));
        html! {
            <button {onclick}>{ "Add new song" }</button>
        }
    };

    html! {
        <ul>
            <li>{ all_songs_list_button }</li>
            <li>{ add_song_button }</li>
        </ul>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <header>
                <hgroup>
                    <h1>{ "setlistrs" }</h1>
                </hgroup>
                <nav>
                    <NavItems />
                </nav>
            </header>
            <main class={classes!("container")}>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}
