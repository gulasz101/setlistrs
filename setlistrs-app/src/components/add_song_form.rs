use yew::prelude::*;

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
        <x-grid columns="7" class="margin20">

            <x-cell span="1-3" span-s="row">
                <input placeholder={"title"}/>
            </x-cell>

            <x-cell span="4-6" span-s="row">
                <input placeholder={"yt-url"}/>
            </x-cell>

            <x-cell span="7.." span-s="row">
                <button title={"Another link"} onclick={on_another_link_click}>{"+"}</button>
            </x-cell>
        </x-grid>
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

#[function_component(AddSongForm)]
pub fn add_song_form() -> Html {
    let yt_links_state = use_reducer(|| YtLinksAdded {
        list: Vec::from([1]),
    });

    let on_yt_link_add = {
        let yt_links_state = yt_links_state.clone();
        Callback::from(move |value: i32| {
            yt_links_state.dispatch(YtLinksAddedAction::Add(value));
        })
    };

    let onsubmit = { Callback::from(move |event: SubmitEvent| event.prevent_default()) };

    html! {
    <x-grid columns="8" class="margin20">
        <x-cell span="row" span-s="row">
            <fieldset >
                <legend>{ "New song" }</legend>
                <form onsubmit={onsubmit}>
                    <x-grid columns="7" class="margin20">
                        <x-cell span="row">
                            <input name="song_title" placeholder={"song title"}/>
                        </x-cell>
                    </x-grid>
                    {
                        for yt_links_state.list.iter().cloned().map(|last_id|
                        html! {
                            <AddYTLink last_id={last_id.clone()} onadd={on_yt_link_add.clone()} />
                        }
                                                      )
                    }

                    <x-grid columns="7" class="margin20">
                        <x-cell span="1-3" span-s="row">
                            <input name="cover_url" placeholder={"cover url"}/>
                        </x-cell>
                        <x-cell span="4-6" span-s="row">
                            <input name="cover_title" placeholder={"cover display title"}/>
                        </x-cell>
                    </x-grid>

                    <x-grid columns="7" class="margin20">
                        <x-cell span="row">
                            <input name="chords" placeholder={"chords, ex: b G D A"}/>
                        </x-cell>
                    </x-grid>

                    <x-grid columns="7" class="margin20">
                        <x-cell span="row">
                            <button type={"submit"} >{ "Add new song" }</button>
                        </x-cell>
                    </x-grid>
                </form>
            </fieldset>
        </x-cell>
    </x-grid>
                    }
}
