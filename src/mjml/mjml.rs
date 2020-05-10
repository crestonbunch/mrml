use super::mj_body::MJBody;
use super::mj_head::MJHead;
use super::{Component, Error};
use crate::util::Properties;
use log::debug;
use roxmltree::Node;

const DOCTYPE: &str = "<!doctype html>";
const HTML_OPEN: &str = "<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">";
const HTML_CLOSE: &str = "</html>";

pub struct MJMLElement<'a, 'b> {
    context: Option<Properties>,
    head: MJHead<'a, 'b>,
    body: MJBody<'a, 'b>,
    node: Node<'a, 'b>,
}

fn get_head<'a, 'b>(node: Node<'a, 'b>) -> Result<MJHead<'a, 'b>, Error> {
    for child in node.children() {
        if child.tag_name().name() == "mj-head" {
            return MJHead::parse(child);
        }
    }
    Ok(MJHead::empty())
}

fn get_body<'a, 'b>(node: Node<'a, 'b>) -> Result<MJBody<'a, 'b>, Error> {
    for child in node.children() {
        if child.tag_name().name() == "mj-body" {
            return MJBody::parse(child);
        }
    }
    Ok(MJBody::empty())
}

impl MJMLElement<'_, '_> {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJMLElement<'a, 'b>, Error> {
        debug!("parse");
        let head = get_head(node)?;
        let body = get_body(node)?;
        let element = MJMLElement {
            context: None,
            node,
            head,
            body,
        };
        Ok(element)
    }
}

impl Component for MJMLElement<'_, '_> {
    fn default_attribute(_key: &str) -> Option<String> {
        None
    }

    fn node(&self) -> Option<Node> {
        Some(self.node)
    }

    fn set_context(&mut self, ctx: Properties) {
        self.context = Some(ctx.clone());
        // TODO
        self.body.set_context(ctx.clone());
        self.head.set_context(ctx.clone());
    }

    fn render(&self) -> Result<String, Error> {
        debug!("render");
        let mut res: Vec<String> = vec![];
        res.push(DOCTYPE.into());
        res.push(HTML_OPEN.into());
        res.push(self.head.render()?);
        res.push(self.body.render()?);
        res.push(HTML_CLOSE.into());
        Ok(res.join(""))
    }
}
