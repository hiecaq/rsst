use crate::upstream::Article;
use html5ever::driver::ParseOpts;
use html5ever::interface::{ElementFlags, QualName};
use html5ever::rcdom::{Node, RcDom};
use html5ever::tendril::{format_tendril, TendrilSink};
use html5ever::tree_builder::{NodeOrText, TreeBuilderOpts, TreeSink};
use html5ever::{local_name, namespace_url, ns};
use html5ever::{parse_document, serialize};

use std::io;
use std::rc::Rc;

enum Tag {
    TD,
    TH,
}

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

pub struct HTML {
    doc: Rc<Node>,
    title: String,
}

impl HTML {
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

    pub fn serialize(&self, out: &mut io::Write) {
        serialize(out, &self.doc, serialize::SerializeOpts::default()).expect("yes");
    }

    pub fn to_string(&self) -> String {
        let mut output = vec![];
        self.serialize(&mut output);
        String::from_utf8(output).expect("failed to serialize into string")
    }

    pub fn filename(&self) -> String {
        format!("{}.html", &self.title)
    }
}
