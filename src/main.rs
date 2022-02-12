mod components;
mod content;
mod generator;
mod pages;
mod parser;

use crate::content::Blog;
use parser::{Order, Parser};
use std::path::PathBuf;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

use wasm_bindgen_futures::spawn_local;

use pages::{home::Home, page_not_found::PageNotFound, post::Post, post_list::PostList};
use yew::html::Scope;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: u64 },
    #[at("/posts")]
    Posts,
    /*
     *#[at("/authors/:id")]
     *Author { id: u64 },
     *#[at("/authors")]
     *Authors,
     */
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app(props: &Parser) -> Html {
    let ctx = use_state(|| props.clone());
    parser::log(&format!("in app parser len: {:?}", props.len()));

    html! {
        <ContextProvider<Parser> context={(*ctx).clone()}>
            <Model />
        </ContextProvider<Parser>>
    }
}

pub enum Msg {
    ToggleNavbar,
    ChangeOrder(Order),
}

pub struct Model {
    navbar_active: bool,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        //let state = use_state(|| _ctx.props().clone());
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
            Msg::ChangeOrder(ord) => {
                //_ctx.props().change_ord(ord);
                let (mut p, _) = _ctx
                    .link()
                    .context::<Parser>(Callback::noop())
                    .expect("Context to be set");
                p.change_ord(ord);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
                /*
                 *<footer class="footer">
                 *    <div class="content has-text-centered">
                 *        { "Powered by " }
                 *        <a href="https://yew.rs">{ "Yew" }</a>
                 *        { " using " }
                 *        <a href="https://bulma.io">{ "Bulma" }</a>
                 *        { " and images from " }
                 *        <a href="https://unsplash.com">{ "Unsplash" }</a>
                 *    </div>
                 *</footer>
                 */
                </BrowserRouter>
        }
    }
}
impl Model {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "VPS Blog" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Posts}>
                            { "Posts" }
                        </Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            /*
                             *<div class="navbar-link">
                             *    { "More" }
                             *</div>
                             */
                            /*
                             *<div class="navbar-dropdown">
                             *    <Link<Route> classes={classes!("navbar-item")} to={Route::Authors}>
                             *        { "Meet the authors" }
                             *    </Link<Route>>
                             *</div>
                             */
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Post { id } => {
            html! { <Post offset={id} /> }
        }
        Route::Posts => {
            html! { <PostList /> }
        }
        /*
         *Route::Author { id } => {
         *    html! { <Author seed={id} /> }
         *}
         *Route::Authors => {
         *    html! { <AuthorList /> }
         *}
         */
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    parser::log("before parse");
    static mut INITED: bool = false;
    spawn_local(async move {
        log::info!("parsing");
        let mut parse = parser::Parser::new();
        parse.add_dir("data/");
        unsafe {
            if !INITED {
                parse.parse().await;
                INITED = true;
            }
        }
        log::debug!("posts len: {}, indexs: {:?}", parse.len(), parse.indexs());
        //p2.lock().unwrap().push(parse);
        //yew::start_app::<Model>();
        yew::start_app_with_props::<App>(parse);
    });
    //wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    ////yew::start_app::<Model>();
    //loop {
    //match p.lock().unwrap().pop() {
    //Some(parse) => {
    //if parse.parsed {
    //yew::start_app_with_props::<Model>(parse);
    //break;
    //} else {
    //// wait for the Parse to be done
    //log::debug!("val: some, parser is not ready yet");
    ////std::thread::sleep(std::time::Duration::from_millis(10));
    //}
    //}
    //None => {
    //// wait for the Parse to be done
    //log::debug!("val: None, parser is not ready yet");
    ////std::thread::sleep(std::time::Duration::from_millis(10));
    //}
    //}
    //}
}
