use crate::components::pagination::PageQuery;
use crate::components::pagination::Pagination;
use crate::components::post_card::BlogCard;
use crate::Parser;
use crate::Route;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

pub const ITEMS_PER_PAGE: u64 = 12;
//const TOTAL_PAGES: u64 = u64::MAX / ITEMS_PER_PAGE;

pub enum Msg {
    PageUpdated,
}

pub struct PostList {
    page: u64,
    _listener: LocationHandle,
}

fn current_page(ctx: &Context<PostList>) -> u64 {
    let location = ctx.link().location().unwrap();

    location.query::<PageQuery>().map(|it| it.page).unwrap_or(1)
}

impl Component for PostList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let listener = ctx
            .link()
            .add_location_listener(link.callback(move |_| {
                //link.send_message(Msg::PageUpdated);
                Msg::PageUpdated
            }))
            .unwrap();

        Self {
            page: current_page(ctx),
            _listener: listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PageUpdated => self.page = current_page(ctx),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page;
        let (parser, _) = ctx
            .link()
            .context::<Parser>(Callback::noop())
            .expect("Parser Context not found");
        let total_pages = parser.len() as u64 / ITEMS_PER_PAGE;

        html! {
            <div class="section container">
                <h1 class="title">{ "Posts" }</h1>
                <h2 class="subtitle">{ "All of our quality writing in one place" }</h2>
                { self.view_posts(ctx) }
                <Pagination
                    {page}
                    total_pages={total_pages}
                    route_to_page={Route::Posts}
                />
            </div>
        }
    }
}
impl PostList {
    fn view_posts(&self, _ctx: &Context<Self>) -> Html {
        let start_seed = (self.page - 1) * ITEMS_PER_PAGE;
        let mut cards = (0..ITEMS_PER_PAGE).map(|seed_offset| {
            html! {
                <li class="list-item mb-5">
                    <BlogCard offset={start_seed + seed_offset} />
                </li>
            }
        });
        html! {
            <div class="columns">
                <div class="column">
                    <ul class="list">
                        { for cards.by_ref().take(ITEMS_PER_PAGE as usize / 3) }
                    </ul>
                </div>
                <div class="column">
                    <ul class="list">
                        { for cards.by_ref().take(ITEMS_PER_PAGE as usize / 3) }
                    </ul>
                </div>
                <div class="column">
                    <ul class="list">
                        { for cards }
                    </ul>
                </div>
            </div>
        }
    }
}
