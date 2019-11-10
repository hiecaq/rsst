//! Provides functions related to the the dumped file.

use crate::upstream::Article;
use html5ever::driver::ParseOpts;
use html5ever::interface::{ElementFlags, QualName};
use html5ever::rcdom::{Node, RcDom};
use html5ever::tendril::{format_tendril, TendrilSink};
use html5ever::tree_builder::{Attribute, NodeOrText, TreeBuilderOpts, TreeSink};
use html5ever::{local_name, namespace_url, ns};
use html5ever::{parse_document, serialize};

use std::io;
use std::rc::Rc;

enum Tag {
    TD,
    TH,
}

/// Returns `<td>value</td>` or `<th>value</td>`.
#[allow(clippy::needless_pass_by_value)]
fn to_entry(dom: &mut RcDom, value: &str, tag: Tag) -> Rc<Node> {
    let td = dom.create_element(
        QualName::new(
            None,
            ns!(),
            match tag {
                Tag::TD => local_name!("td"),
                Tag::TH => local_name!("th"),
            },
        ),
        vec![],
        ElementFlags::default(),
    );
    dom.append(&td, NodeOrText::AppendText(format_tendril!("{}", value)));
    td
}

/// Returns `<tr><th>key</th> <td>value</td></tr>`.
fn to_pair(dom: &mut RcDom, key: &str, value: &str) -> Rc<Node> {
    let tr = dom.create_element(
        QualName::new(None, ns!(), local_name!("tr")),
        vec![],
        ElementFlags::default(),
    );
    let th = NodeOrText::AppendNode(to_entry(dom, key, Tag::TH));
    dom.append(&tr, th);
    let td = NodeOrText::AppendNode(to_entry(dom, value, Tag::TD));
    dom.append(&tr, td);
    tr
}

/// Returns `<table> <tr>...</tr> <tr>...</tr> ... </table>` that
/// contains metadata about `a`.
fn to_table(dom: &mut RcDom, a: &Article) -> Rc<Node> {
    let table = dom.create_element(
        QualName::new(None, ns!(), local_name!("table")),
        vec![],
        ElementFlags::default(),
    );
    let pairs = vec![
        NodeOrText::AppendNode(to_pair(dom, "title", &a.title)),
        NodeOrText::AppendNode(to_pair(dom, "author", &a.author)),
        NodeOrText::AppendNode(to_pair(dom, "date", &a.date)),
        NodeOrText::AppendNode(to_pair(dom, "link", &a.link)),
        NodeOrText::AppendNode(to_pair(dom, "category", &a.category.join(" "))),
    ];
    for pair in pairs {
        dom.append(&table, pair);
    }
    table
}

/// Returns `<title>value</title>`.
#[allow(clippy::needless_pass_by_value)]
fn to_title(dom: &mut RcDom, value: &str) -> Rc<Node> {
    let title = dom.create_element(
        QualName::new(None, ns!(), local_name!("title")),
        vec![],
        ElementFlags::default(),
    );
    dom.append(&title, NodeOrText::AppendText(format_tendril!("{}", value)));
    title
}

/// Returns a list of nodes that should be added to `<head>`.
fn to_headinfo(dom: &mut RcDom, a: &Article) -> Vec<Rc<Node>> {
    vec![
        dom.create_element(
            QualName::new(None, ns!(), local_name!("meta")),
            vec![Attribute {
                name: QualName::new(None, ns!(), local_name!("charset")),
                value: format_tendril!("UTF-8"),
            }],
            ElementFlags::default(),
        ),
        dom.create_element(
            QualName::new(None, ns!(), local_name!("meta")),
            vec![
                Attribute {
                    name: QualName::new(None, ns!(), local_name!("name")),
                    value: format_tendril!("viewpoint"),
                },
                Attribute {
                    name: QualName::new(None, ns!(), local_name!("content")),
                    value: format_tendril!("width=device-width"),
                },
            ],
            ElementFlags::default(),
        ),
        dom.create_element(
            QualName::new(None, ns!(), local_name!("link")),
            vec![
                Attribute {
                    name: QualName::new(None, ns!(), local_name!("rel")),
                    value: format_tendril!("stylesheet"),
                },
                Attribute {
                    name: QualName::new(None, ns!(), local_name!("href")),
                    value: format_tendril!("../style.css"),
                },
            ],
            ElementFlags::default(),
        ),
        to_title(dom, &a.title),
    ]
}

/// HTML is a format that `RSSt` could dump to.
pub struct HTML {
    /// saved document.
    doc: Rc<Node>,
    /// saved title.
    title: String,
}

impl HTML {
    /// Build an `HTML` with the given `Article`.
    pub fn from(a: &Article) -> Self {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..TreeBuilderOpts::default()
            },
            ..ParseOpts::default()
        };
        let mut dom = parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from(&mut a.content.as_bytes())
            .unwrap();
        let body = Rc::clone(&dom.document.children.borrow()[0].children.borrow()[1]);
        let head = Rc::clone(&dom.document.children.borrow()[0].children.borrow()[0]);
        for e in to_headinfo(&mut dom, &a) {
            dom.append(&head, NodeOrText::AppendNode(e));
        }
        let table = NodeOrText::AppendNode(to_table(&mut dom, a));
        let sibling = match body.children.borrow().get(0) {
            Some(sibling) => Some(Rc::clone(sibling)),
            None => None,
        };
        match sibling {
            Some(sibling) => dom.append_before_sibling(&sibling, table),
            None => dom.append(&body, table),
        }
        Self {
            doc: Rc::clone(&dom.document),
            title: String::clone(&a.title),
        }
    }

    /// Output `self` into the given `Write`.
    pub fn serialize(&self, out: &mut dyn io::Write) {
        serialize(out, &self.doc, serialize::SerializeOpts::default()).expect("yes");
    }

    /// Serialize `self` into a String.
    pub fn to_string(&self) -> String {
        let mut output = vec![];
        self.serialize(&mut output);
        String::from_utf8(output).expect("failed to serialize into string")
    }

    /// Returns the filename of `self`.
    pub fn filename(&self) -> String {
        format!("{}.html", &self.title)
    }
}
