use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// it represents a `Blog`
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Blog {
    pub path: PathBuf,
    pub timestamp: u64,
    pub title: String,
    pub tags: Vec<String>,
    pub hero: Option<String>,
    pub content: Vec<String>,
    pub published: bool,
    pub ignored: bool,
}

impl Blog {
    /// extract date infomation from blog
    pub fn date_info(&mut self, date: Option<&str>) {
        // try get it from path
        // eg: 2019-10-07-bolg-title-here;  19-3-7-bolg-title-here
        // eg: 2019-10-07-13-32-bolg-title-here;  19-3-7-01-59-bolg-title-here
        // note that the format is descending: yyyy-mm-dd-hh-MM-ss
        // and the accuracy is second and
        // year,month,day are required
        let pat = regex::Regex::new(r"(\d{2,4}\D\d{1,2}\D\d{1,2}(\D\d{1,2}){0,3})").unwrap();
        //let path1 = "19-10-07-13-32-bolg-title-here";
        //let path1 = "data/19-10-07-13-bolg-title-here.md";
        //let path1 = "";
        //self.path = path1.into();
        let mut time_items = [0u64; 6];
        const UNITS: [u64; 6] = [365 * 24 * 3600, 30 * 24 * 3600, 24 * 3600, 60 * 60, 60, 1];
        // if path is matched
        if let Some(cap) = pat.captures(self.path.to_str().unwrap()) {
            cap[1]
                .split(|c: char| !c.is_ascii_digit())
                .enumerate()
                .for_each(|(ind, e)| {
                    let num = e.parse::<u64>().unwrap_or(0);
                    time_items[ind] = num;
                });
            if time_items[0] < 100 {
                time_items[0] += 32;
            }
            let sum = UNITS
                .iter()
                .zip(time_items.iter())
                .fold(0, |acc, (e1, e2)| acc + e1 * e2);
            self.timestamp = sum;
            log::debug!("time items: {:?}, its sum: {}", time_items, sum);
        } else {
            // path is not matched but pre-defined
            // try get it from meta data in pre-defined info
            // eg: date: 2019-10-07
            match date {
                Some(s) => {
                    if let Some(cap) = pat.captures(s) {
                        cap[1]
                            .split(|c: char| !c.is_ascii_digit())
                            .enumerate()
                            .for_each(|(ind, e)| {
                                let num = e.parse::<u64>().unwrap_or(0);
                                time_items[ind] = num;
                            });
                        if time_items[0] < 100 {
                            time_items[0] += 2000;
                        }
                        let sum = UNITS
                            .iter()
                            .zip(time_items.iter())
                            .fold(0, |acc, (e1, e2)| acc + e1 * e2);
                        log::debug!("time items: {:?}, its sum: {}", time_items, sum);
                        self.timestamp = sum;
                    }
                }
                // not know
                None => {
                    log::error!("Time Stampe is not found in file name nor defined in file");
                    log::error!("file is ignored to proceed: {:?}", self.path);
                    self.ignored = true;
                }
            }
        }
    }
}

#[test]
fn test_date_info() {
    let mut blog = Blog {
        path: PathBuf::new(),
        timestamp: 0,
        title: "".into(),
        tags: vec![],
        hero: None,
        content: vec![],
        published: false,
        ignored: false,
    };
    blog.date_info(Some("2019-10-07"));
    blog.date_info(Some("2019-10-07-02-01"));
    blog.date_info(Some("2019/10/07/02/01"));
    blog.date_info(Some("2019/10/07 19:57"));
    blog.date_info(Some("2019-10/07 19:57:36"));
    blog.date_info(None);
}

use crate::generator::{Generated, Generator};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Author {
    pub seed: u64,
    pub name: String,
    pub keywords: Vec<String>,
    pub image_url: String,
}
impl Generated for Author {
    fn generate(gen: &mut Generator) -> Self {
        let name = gen.human_name();
        let keywords = gen.keywords();
        let image_url = gen.face_image_url((600, 600));
        Self {
            seed: gen.seed,
            name,
            keywords,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostMeta {
    pub seed: u64,
    pub title: String,
    pub author: Author,
    pub keywords: Vec<String>,
    pub image_url: String,
}
impl Generated for PostMeta {
    fn generate(gen: &mut Generator) -> Self {
        let title = gen.title();
        let author = Author::generate_from_seed(gen.new_seed());
        let keywords = gen.keywords();
        let image_url = gen.image_url((1000, 500), &keywords);

        Self {
            seed: gen.seed,
            title,
            author,
            keywords,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Post {
    pub meta: PostMeta,
    pub content: Vec<PostPart>,
}
impl Generated for Post {
    fn generate(gen: &mut Generator) -> Self {
        const PARTS_MIN: usize = 1;
        const PARTS_MAX: usize = 10;

        let meta = PostMeta::generate(gen);

        let n_parts = gen.range(PARTS_MIN, PARTS_MAX);
        let content = (0..n_parts).map(|_| PostPart::generate(gen)).collect();

        Self { meta, content }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PostPart {
    Section(Section),
    Quote(Quote),
}
impl Generated for PostPart {
    fn generate(gen: &mut Generator) -> Self {
        // Because we pass the same (already used) generator down,
        // the resulting `Section` and `Quote` aren't be reproducible with just the seed.
        // This doesn't matter here though, because we don't need it.
        if gen.chance(1, 10) {
            Self::Quote(Quote::generate(gen))
        } else {
            Self::Section(Section::generate(gen))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Section {
    pub title: String,
    pub paragraphs: Vec<String>,
    pub image_url: String,
}
impl Generated for Section {
    fn generate(gen: &mut Generator) -> Self {
        const PARAGRAPHS_MIN: usize = 1;
        const PARAGRAPHS_MAX: usize = 8;

        let title = gen.title();
        let n_paragraphs = gen.range(PARAGRAPHS_MIN, PARAGRAPHS_MAX);
        let paragraphs = (0..n_paragraphs).map(|_| gen.paragraph()).collect();
        let image_url = gen.image_url((600, 300), &[]);

        Self {
            title,
            paragraphs,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Quote {
    pub author: Author,
    pub content: String,
}
impl Generated for Quote {
    fn generate(gen: &mut Generator) -> Self {
        // wouldn't it be funny if the author ended up quoting themselves?
        let author = Author::generate_from_seed(gen.new_seed());
        let content = gen.paragraph();
        Self { author, content }
    }
}
