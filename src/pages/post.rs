use crate::Parser;
use crate::{content, generator::Generated, Route};
use content::PostPart;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub offset: u64,
}

pub struct Post {
    post: content::Blog,
}
impl Component for Post {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (parser, _) = ctx
            .link()
            .context::<Parser>(Callback::noop())
            .expect("Parser Context not found");
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

        let keywords = post
            .tags
            .iter()
            .map(|tag| html! { <span class="tag is-info">{ tag }</span> });

        html! {
            <>
                <section class="hero is-medium is-light has-background">
                    <img alt="The hero's background" class="hero-background is-transparent" src={post.hero.clone()} />
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">
                                { &post.title }
                            </h1>
                            //<h2 class="subtitle">
                                //{ "by " }
                                //<Link<Route> classes={classes!("has-text-weight-semibold")} to={Route::Author { id: post.author.seed }}>
                                    //{ &post.meta.author.name }
                                //</Link<Route>>
                            //</h2>
                            <div class="tags">
                                { for keywords }
                            </div>
                        </div>
                    </div>
                </section>
                <div class="section container">
                    { self.view_content() }
                </div>
            </>
        }
    }
}
impl Post {
    fn render_quote(&self, quote: &content::Quote) -> Html {
        html! {
            <article class="media block box my-6">
                <figure class="media-left">
                    <p class="image is-64x64">
                        <img alt="The author's profile" src={quote.author.image_url.clone()} loading="lazy" />
                    </p>
                </figure>
                <div class="media-content">
                    <div class="content">
                        /*
                         *<Link<Route> classes={classes!("is-size-5")} to={Route::Author { id: quote.author.seed }}>
                         *    <strong>{ &quote.author.name }</strong>
                         *</Link<Route>>
                         */
                        <p class="is-family-secondary">
                            { &quote.content }
                        </p>
                    </div>
                </div>
            </article>
        }
    }

    fn render_section_hero(&self, section: &content::Section) -> Html {
        html! {
            <section class="hero is-dark has-background mt-6 mb-3">
                <img alt="This section's image" class="hero-background is-transparent" src={section.image_url.clone()} loading="lazy" />
                <div class="hero-body">
                    <div class="container">
                        <h2 class="subtitle">{ &section.title }</h2>
                    </div>
                </div>
            </section>
        }
    }

    fn render_section(&self, section: &content::Section, show_hero: bool) -> Html {
        let hero = if show_hero {
            self.render_section_hero(section)
        } else {
            html! {}
        };
        let paragraphs = section.paragraphs.iter().map(|paragraph| {
            html! {
                <p>{ paragraph }</p>
            }
        });
        html! {
            <section>
                { hero }
                <div>{ for paragraphs }</div>
            </section>
        }
    }

    fn view_content(&self) -> Html {
        // don't show hero for the first section
        let mut show_hero = false;

        /*
         *let parts = self.post.content.iter().map(|part| match part {
         *    PostPart::Section(section) => {
         *        let html = self.render_section(section, show_hero);
         *        // show hero between sections
         *        show_hero = true;
         *        html
         *    }
         *    PostPart::Quote(quote) => {
         *        // don't show hero after a quote
         *        show_hero = false;
         *        self.render_quote(quote)
         *    }
         *});
         */
        let dom_parser = web_sys::DomParser::new().unwrap();
        let mut parts = Vec::new();
        self.post.content.iter().for_each(|part| {
            use pulldown_cmark::{Event, Options, Tag};
            let mut options = Options::empty();
            options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
            options.insert(Options::ENABLE_SMART_PUNCTUATION);
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TASKLISTS);
            let parser = pulldown_cmark::Parser::new_ext(&part, options);
            /*
             *.map(|event| match event {
             *    Event::Start( Tag::Paragraph) | Event::End( Tag::Paragraph) => Event::Text( ),
             *    _ => event,
             *}
             *);
             */
            let mut output = String::with_capacity(part.len() * 3 / 2);
            pulldown_cmark::html::push_html(&mut output, parser);
            let output_div = format!(
                "<div class = \"container\"> {} </div>",
                output //.replace("<p>", "\n").replace("</p>", "<br/>")
            );
            log::debug!("{}", &format!("parsing markdown into html"));
            if let Ok(element) =
                dom_parser.parse_from_string(&output_div, web_sys::SupportedType::TextHtml)
            {
                log::debug!("{}", &format!("Done: parsing markdown into html"));
                //log::debug!("document: {}", &format!("{:?}", element));
                //web_sys::console::log_1("");
                //Html::VRef(element.body().unwrap().iter().into())
                let eles = element.body().unwrap().children();
                for ind in 0..eles.length() {
                    let node = eles.get_with_index(ind).unwrap();
                    let vnode = Html::VRef(node.into());
                    parts.push(vnode);
                }
            } else {
                //Html::default()
                log::debug!("{}", &format!("failed to parsed markdown into html"));
                let node = html! { <p> {"the markdown file is not parsed"} </p> };
                parts.push(node);
            }
            //Html::from(output)
        });
        html! { for parts }
    }
}
