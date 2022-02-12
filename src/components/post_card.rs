use crate::content::Blog;
use crate::Parser;
use crate::{content::PostMeta, generator::Generated, Route};
use std::path::PathBuf;
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub offset: u64,
}

pub struct BlogCard {
    post: Blog,
}
impl Component for BlogCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (parser, _) = ctx
            .link()
            .context::<Parser>(Callback::noop())
            .expect("Parser Context not found");
        log::debug!("offset: {}", ctx.props().offset,);
        log::debug!("posts len: {}, indexs: {:?}", parser.len(), parser.indexs());
        let post = parser.get(ctx.props().offset as usize).unwrap().clone();
        Self { post }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let (parser, _) = ctx
            .link()
            .context::<Parser>(Callback::noop())
            .expect("Parser Context not found");
        let post = parser.get(ctx.props().offset as usize).unwrap();
        self.post = post.clone();
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { post } = self;
        html! {
            <div class="card">
                <div class="card-image">
                    <figure class="image is-2by1">
                        <img alt="post's hero image" src={post.hero.clone()} loading="lazy" />
                    </figure>
                </div>
                <div class="card-content">
                    <Link<Route> classes={classes!("title", "is-block")} to={Route::Post { id: _ctx.props().offset }}>
                        { &post.title }
                    </Link<Route>>
                    //<Link<Route> classes={classes!("subtitle", "is-block")} to={Route::Author { id: post.author.seed }}>
                        //{ &post.author.name }
                    //</Link<Route>>
                </div>
            </div>
        }
    }
}

/*
 *pub struct PostCard {
 *    post: PostMeta,
 *}
 *impl Component for PostCard {
 *    type Message = ();
 *    type Properties = Props;
 *
 *    fn create(ctx: &Context<Self>) -> Self {
 *        Self {
 *            post: PostMeta::generate_from_seed(ctx.props().seed),
 *        }
 *    }
 *    fn changed(&mut self, ctx: &Context<Self>) -> bool {
 *        self.post = PostMeta::generate_from_seed(ctx.props().seed);
 *        true
 *    }
 *
 *    fn view(&self, _ctx: &Context<Self>) -> Html {
 *        let Self { post } = self;
 *        html! {
 *            <div class="card">
 *                <div class="card-image">
 *                    <figure class="image is-2by1">
 *                        <img alt="This post's image" src={post.image_url.clone()} loading="lazy" />
 *                    </figure>
 *                </div>
 *                <div class="card-content">
 *                    <Link<Route> classes={classes!("title", "is-block")} to={Route::Post { id: post.seed }}>
 *                        { &post.title }
 *                    </Link<Route>>
 *                    <Link<Route> classes={classes!("subtitle", "is-block")} to={Route::Author { id: post.author.seed }}>
 *                        { &post.author.name }
 *                    </Link<Route>>
 *                </div>
 *            </div>
 *        }
 *    }
 *}
 */
