//! Provides functions related to the the source.

use crate::metadata::Metadata;
use md5;
use rss;

/// A representation of an article in the feed.
#[derive(Debug)]
pub struct Article {
    /// title of this article.
    pub title: String,
    /// link to the original article.
    pub link: String,
    /// author of this article (if given)
    pub author: String,
    /// date when this article is post
    pub date: String,
    /// category that this article falls in.
    pub category: Vec<String>,
    /// the main content of this article. Defaults to
    /// `content` if given, otherwise `description`.
    pub content: String,
    /// The md5 checksum of this article.
    pub checksum: String,
}

impl Article {
    /// Constructs an `Article` with the given RSS item.
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
    /// construct a metadata with the given `Channel`.
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

/// A representation of a feed.
#[derive(Debug)]
pub struct Source {
    /// articles given in the feed.
    pub article: Vec<Article>,
    /// metadata about this feed.
    pub metadata: Metadata,
}

#[derive(Debug)]
pub enum Error {
    /// failed to parse the given RSS feed.
    RSSParseFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

/// Try serializing the feed at the `url` into a `Source`.
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
