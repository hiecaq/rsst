use crate::metadata::Metadata;
use md5;
use rss;

#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub link: String,
    pub author: String,
    pub date: String,
    pub category: Vec<String>,
    pub content: String,
    pub checksum: String,
}

impl Article {
    fn new(x: &rss::Item) -> Self {
        let date = x.pub_date().unwrap_or("");
        let title = x.title().unwrap_or("");
        let description = x.description().unwrap_or("");
        let checksum = if date != "" {
            date
        } else if title == "" {
            description
        } else {
            title
        };
        Self {
            author: String::from(x.author().unwrap_or("")),
            content: String::from(match x.content() {
                Some(v) => v,
                None => description,
            }),
            date: String::from(date),
            title: String::from(title),
            link: String::from(match x.source() {
                Some(v) => v.url(),
                None => match x.link() {
                    Some(v) => v,
                    None => "",
                },
            }),
            category: x
                .categories()
                .iter()
                .map(|c| String::from(c.name()))
                .collect(),
            checksum: format!("{:x}", md5::compute(checksum)),
        }
    }
}

impl Metadata {
    fn new(ch: &rss::Channel) -> Self {
        let candidate = if ch.items().is_empty() {
            ch.title()
        } else {
            match ch.items()[0].pub_date() {
                Some(v) => v,
                None => match ch.items()[0].title() {
                    Some(v) => v,
                    None => match ch.items()[0].description() {
                        Some(v) => v,
                        None => ch.title(), // should not be possible
                    },
                },
            }
        };
        Self {
            title: String::from(ch.title()),
            checksum: format!("{:x}", md5::compute(candidate)),
        }
    }
}

#[derive(Debug)]
pub struct Source {
    pub article: Vec<Article>,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub enum Error {
    RSSParseFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub fn to_source(url: &str) -> Result<Source, Error> {
    let channel = match rss::Channel::from_url(url) {
        Ok(v) => v,
        Err(_) => return Err(Error::RSSParseFailed),
    };
    let metadata = Metadata::new(&channel);
    let article = channel
        .into_items()
        .into_iter()
        .map(|x| Article::new(&x))
        .collect();
    Ok(Source { metadata, article })
}

// TODO: tests
