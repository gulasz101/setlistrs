use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{SetlistDetails, SetlistList, SongAdd, SongsList};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    SongList,
    #[at("/add-song")]
    SongAdd,
    #[at("/setlists/:id")]
    SetlistDetails { id: i64 },
    #[at("/setlists")]
    SetlistList,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::SongList => html! { <SongsList /> },
        Route::SongAdd => html! { <SongAdd/> },
        Route::SetlistDetails { id } => html! { <SetlistDetails seed={id} /> },
        Route::SetlistList => html! { <SetlistList /> },
        Route::NotFound => html! { <h1> {"404"} </h1> },
    }
}

#[function_component(NavItems)]
pub fn nav_items() -> Html {
    let navigator = use_navigator().unwrap();

    let all_songs_list_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::SongList));
        html! {
            <button {onclick}>{ "All songs" }</button>
        }
    };

    let add_song_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::SongAdd));
        html! {
            <button {onclick}>{ "Add new song" }</button>
        }
    };

    let setlist_list = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::SetlistList));
        html! {
            <button {onclick}>{ "Setlists" }</button>
        }
    };

    html! {
        <ul>
            <li>{ all_songs_list_button }</li>
            <li>{ add_song_button }</li>
            <li>{ setlist_list }</li>
        </ul>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <header class={classes!("container")}>
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
