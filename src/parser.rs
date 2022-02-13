#![allow(dead_code)]
//! Here provide a module that accepts a markdown file to
//! reproduce a struct `Post`
//! make it static based blog
use crate::{content::Blog, pages::post_list::ITEMS_PER_PAGE};
use std::collections::HashMap;
use std::path::PathBuf;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = String)]
    pub type JsString;
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log2(s1: &str, s2: &str);
}
/*
 *#[wasm_bindgen(module = "/src/ffs.js")]
 *extern "C" {
 *    #[wasm_bindgen(js_name = "read_file")]
 *    pub fn read_file(path: &str) -> JsString;
 *    #[wasm_bindgen(js_name = "read_dir")]
 *    pub fn read_dir(path: &str) -> JsValue;
 *    pub async fn fetch_dir() -> JsValue;
 *    pub async fn fetch_file(path: &str) -> JsValue;
 *}
 */
/*
 *pub fn read_dir(path: &str) -> Vec<&str> {
 *    const INDEXS: &str = include_str!("../data/.index");
 *    INDEXS
 *        .split("\n")
 *        .map(|e| e.trim())
 *        .filter(|e| e.len() > 0)
 *        .collect::<_>()
 *}
 */

//pub fn read_file(path: &str) -> &str {
//include_str!(path)
//}

/// the order of post sort
#[derive(PartialEq, Clone, Debug)]
pub enum Order {
    Dec,
    Inc,
    Hot,
}

impl Default for Order {
    fn default() -> Self {
        Self::Dec
    }
}

pub struct BlogPath {
    pub timestamp: u64,
    pub path: PathBuf,
}
impl BlogPath {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            path: PathBuf::new(),
        }
    }

    pub fn with_str(&mut self, s: &str) -> Self {
        Self {
            timestamp: 0,
            path: PathBuf::from(s),
        }
    }
}

/// the inner data of Parser
#[derive(Clone, Default, Debug, PartialEq)]
pub struct InnerParser {
    blogs: HashMap<PathBuf, Blog>,
    indexs: Vec<(u64, PathBuf)>,
    len: usize,
}

//const KEYWORDSY: [&str; 5] = ["title", "description", "tags", "hero", "published"];
/// it represents a `Parser` that parse all markdown file of directory
#[derive(Clone, Default, Debug, PartialEq, Properties)]
pub struct Parser {
    pub dirs: Vec<PathBuf>,
    pub blog_paths: HashMap<PathBuf, (usize, Vec<PathBuf>)>,
    inner: InnerParser,
    pub order: Order,
    pub parsed: bool,
}

/// implementation of Parser to load markdown file as Blog
impl Parser {
    pub fn new() -> Self {
        Self {
            dirs: vec![],
            blog_paths: HashMap::new(),
            inner: InnerParser {
                blogs: HashMap::new(),
                indexs: vec![],
                len: 0,
            },
            order: Order::Dec,
            parsed: false,
        }
    }

    pub fn with_dir(dirs: &str) -> Self {
        Self {
            dirs: vec![dirs.into()],
            blog_paths: HashMap::new(),
            inner: InnerParser {
                blogs: HashMap::new(),
                indexs: vec![],
                len: 0,
            },
            order: Order::Dec,
            parsed: false,
        }
    }

    pub fn change_ord(&mut self, ord: Order) {
        if ord == self.order {
            // nothing to change
            log::debug!("ord not changed");
        } else {
            // change the order of blogs
            self.order = ord;
            self.order();
        }
    }

    /// to add anther directory containing markdown file to be parsed
    pub fn add_dir(&mut self, dir: &str) {
        self.dirs.push(dir.into());
    }

    ///sort the blog with the Order
    pub fn order(&mut self) {
        let cmp_fn = match self.order {
            Order::Inc => |a: &(u64, PathBuf), b: &(u64, PathBuf)| -> std::cmp::Ordering {
                a.0.partial_cmp(&b.0).unwrap()
            },
            Order::Dec => |a: &(u64, PathBuf), b: &(u64, PathBuf)| -> std::cmp::Ordering {
                b.0.partial_cmp(&a.0).unwrap()
            },
            Order::Hot => |a: &(u64, PathBuf), b: &(u64, PathBuf)| -> std::cmp::Ordering {
                b.0.partial_cmp(&a.0).unwrap()
            },
        };
        self.inner.indexs.sort_by(cmp_fn);
    }

    /// get the length of the blogs
    pub fn len(&self) -> usize {
        self.inner.len
    }

    pub fn indexs(&self) -> &Vec<(u64, PathBuf)> {
        &self.inner.indexs
    }

    /// get the blog with the index
    pub fn get(&self, index: usize) -> Option<&Blog> {
        match self.inner.indexs.get(index) {
            Some(path) => self.inner.blogs.get(&path.1),
            None => None,
        }
    }

    /// get the blog by the path
    pub fn get_by_path(&self, path: PathBuf) -> Option<&Blog> {
        self.inner.blogs.get(&path)
    }

    /// read the index of directory
    pub async fn read_dir(&mut self) {
        use wasm_bindgen_futures::JsFuture;
        log::trace!("{}", &format!("read dir",));
        if self.dirs.is_empty() {
            use crate::constant;
            if constant::use_gitpage {
                let path = format!("{}{}", constant::subpath, "public/");
                self.dirs.push(path.into());
            } else {
                self.dirs.push("public/".into());
            }
        }
        for dir in self.dirs.iter() {
            log::debug!("{}", &format!("parse dir: {:?}", dir));
            let window = web_sys::window().unwrap();
            let protocol = window
                .location()
                .protocol()
                .expect("Protocol is NULL")
                .to_string();
            let host = window
                .location()
                .host()
                .expect("HOST is not NULL")
                .to_string();
            let mut url = format!("{}//{}/{}", protocol, host, dir.to_str().unwrap());
            if !url.ends_with("/") {
                url.push_str("/");
            }
            url.push_str("markdown.index");
            log::debug!("{}", &format!("url: {:?}", url));
            let res = JsFuture::from(window.fetch_with_str(&url)).await.unwrap();
            assert!(res.is_instance_of::<web_sys::Response>());
            let res_val = res.dyn_into::<web_sys::Response>().unwrap();
            let text = JsFuture::from(res_val.text().unwrap())
                .await
                .unwrap()
                .as_string()
                .expect("the .index shall return string");
            log::trace!("{}", &format!("content {:?}", text));
            let paths = text
                .split("\n")
                .map(|e| e.trim().to_string())
                .filter(|e| e.len() != 0 && (e.ends_with("rmd") || e.ends_with("rmarkdown")))
                .map(|e| e.into())
                .collect();
            self.blog_paths.insert(dir.into(), (0, paths));
        }
        self.order();
    }

    /// read the file
    pub async fn read_file(&self, path: &str) -> Option<String> {
        let window = web_sys::window().unwrap();
        let protocol = window
            .location()
            .protocol()
            .expect("Protocol is NULL")
            .to_string();
        let host = window
            .location()
            .host()
            .expect("HOST is not NULL")
            .to_string();
        let mut url = format!("{}//{}/", protocol, host,);
        use crate::constant;
        if constant::use_gitpage {
            url.push_str(constant::subpath);
        }
        url.push_str(path);
        log::debug!("{}", &format!("url: {:?}", url));
        if let Ok(res) = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(&url)).await {
            assert!(res.is_instance_of::<web_sys::Response>());
            if let Ok(res_val) = res.dyn_into::<web_sys::Response>() {
                if let Ok(item) = res_val.text() {
                    if let Ok(text) = wasm_bindgen_futures::JsFuture::from(item).await {
                        log::trace!(
                            "{}",
                            &format!("path: {:?} \n content: {:?}", url, text.as_string())
                        );
                        return text.as_string();
                    }
                }
            }
        }
        None
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(&self.inner)
    }

    pub fn str2blog(&self, s: &str, path: PathBuf) -> Option<Blog> {
        log::trace!("parsing a string into a blog");
        let sp = s.splitn(3, "---").collect::<Vec<_>>();
        //println!("meta: {:?}, ", sp);
        let meta = sp[1].trim();
        let cont = sp[2]
            .splitn(2, "<!–-break-–>")
            .map(|e| e.trim().to_owned())
            .collect::<Vec<String>>();
        //println!("meta: {}, des: {:?}, cont: {}", meta, cont[0], cont[1]);
        let pat = regex::Regex::new(r"(?P<key>\w+)\s*:\s*(?P<value>.*?$)").unwrap();
        let pat_tag = regex::Regex::new(r"\s*-\s+(?P<tag>.*?$)").unwrap();
        let pat_ml = regex::Regex::new(r"(\s{2,}||\t{1,})(?P<ln>.*?$)").unwrap();
        let mut map = HashMap::new();
        let mut tags = Vec::new();
        let mut lastkey = "";
        for line in meta.split("\n") {
            if line.trim().len() < 1 {
                // empty line skip it
                lastkey = "";
                continue;
            }
            //println!("line: {}", line);
            if let Some(caps) = pat.captures(line) {
                if caps.len() == 1 {
                    log::trace!("Not Matched: {}", line);
                    break;
                }
                let key = caps.name("key").map_or("", |e| e.as_str());
                lastkey = key;
                let value = caps.name("value").map_or("", |e| e.as_str());
                //println!("key: {}, val: {}", key, value);
                if key == "tags" {
                    lastkey = key;
                } else {
                    map.insert(key, value.trim().to_owned());
                }
                continue;
            }
            if let Some(caps) = pat_tag.captures(line) {
                if let Some(tag) = caps.name("tag") {
                    let tag = tag.as_str().trim().to_owned();
                    //println!("tag: {}", tag);
                    tags.push(tag);
                    continue;
                }
            }
            if let Some(caps) = pat_ml.captures(line) {
                let ln = caps.name("ln").unwrap().as_str().trim();
                if let Some(item) = map.get_mut(&lastkey) {
                    item.push_str(ln);
                    //println!("lastkey: {:?}, line: {:?}, map: {:?}", lastkey, ln, map);
                } else {
                    log::debug!("Line ingnored: {}", ln);
                }
            }
        }
        //println!("tags: {:?}, map: {:?}", tags, map);
        if map.get(&"title").is_none() {
            log::error!("Title Missing for {:?}", path);
            return None;
        }
        if map.get(&"published").is_none() {
            log::error!("Attribute `published` Missing for {:?}", path);
            return None;
        }
        let mut blog = Blog {
            path,
            title: map.remove("title").unwrap(),
            timestamp: 0,
            tags,
            //hero: map.remove("hero"),
            hero: Some("https://source.unsplash.com/random/1200x400/?yew".into()),
            content: cont,
            published: map
                .remove("published")
                .unwrap_or("false".into())
                .parse::<bool>()
                .unwrap(),
            ignored: false,
        };
        match map.remove("date") {
            Some(s) => blog.date_info(Some(&s)),
            None => blog.date_info(None),
        }
        println!("blog path: {:?}", blog.path);
        Some(blog)
    }

    pub async fn parse(&mut self) {
        log::trace!("parse begining");
        self.read_dir().await;
        self.load(None, ITEMS_PER_PAGE as u64).await;
        self.parsed = true;
    }

    pub async fn load(&mut self, dir: Option<&str>, batch_size: u64) {
        let key = match dir {
            None => {
                use crate::constant;
                if constant::use_gitpage {
                    let path = format!("{}{}", constant::subpath, "public/");
                    PathBuf::from(path)
                } else {
                    PathBuf::from("public/")
                }
            }
            Some(d) => PathBuf::from(d),
        };
        if let Some((offset, paths)) = self.blog_paths.get(&key) {
            let mut cnt = 0;
            if paths.len() > *offset {
                for ind in 0..batch_size as usize {
                    let path = &paths[ind + *offset];
                    log::debug!("path: {:?}", path.to_str().unwrap());
                    if let Some(buf) = self.read_file(path.to_str().unwrap()).await {
                        cnt += 1;
                        if !buf.is_empty() {
                            // parse the string to Blog
                            log::debug!("parsing file: {:?}", path);
                            let blog = self.str2blog(&buf, path.clone().into());
                            if let Some(Blog { ignored: false, .. }) = blog {
                                let blog = blog.unwrap();
                                self.inner
                                    .indexs
                                    .push((blog.timestamp, PathBuf::from(path.clone())));
                                self.inner.blogs.insert(PathBuf::from(path), blog);
                                self.inner.len += 1;
                                assert_eq!(self.inner.indexs.len(), self.inner.len);
                                assert_eq!(self.inner.blogs.len(), self.inner.len);
                            }
                            //let self.blog_paths.get_mut(key).unwrap();
                        } else {
                            // empty file
                            log::info!("Empty File: {:?}", path);
                        }
                    }
                }
                self.blog_paths.get_mut(&key).unwrap().0 += cnt;
            }
        }
        self.order();
    }
}

pub struct Iter<'a> {
    data: &'a HashMap<PathBuf, Blog>,
    index: &'a Vec<(u64, PathBuf)>,
    offset: usize,
    len: usize,
}
impl<'a> Iter<'a> {
    pub fn new(data: &'a InnerParser) -> Self {
        Self {
            data: &data.blogs,
            index: &data.indexs,
            offset: 0,
            len: data.indexs.len(),
        }
    }
}

unsafe impl Sync for Iter<'_> {}
unsafe impl Send for Iter<'_> {}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Blog;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.len {
            let (_, path) = self.index.get(self.offset).unwrap();
            self.offset += 1;
            return Some(self.data.get(path).unwrap());
        }
        None
    }
}
