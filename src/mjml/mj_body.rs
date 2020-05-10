use super::{Component, Element, Error};
use crate::{close_tag, open_tag};
use crate::util::Properties;
use log::debug;
use roxmltree::Node;

pub struct MJBody<'a, 'b> {
    node: Option<Node<'a, 'b>>,
    context: Option<Properties>,
    children: Vec<Element<'a, 'b>>,
}

impl MJBody<'_, '_> {
    pub fn empty<'a, 'b>() -> MJBody<'a, 'b> {
        MJBody {
            node: None,
            children: vec![],
            context: None,
        }
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJBody<'a, 'b>, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(Element::parse(child)?);
        }
        Ok(MJBody {
            node: Some(node),
            children,
            context: None,
        })
    }

    pub fn digest_context(&mut self) {}

    fn get_style(&self, key: &str) -> Properties {
        let mut res = Properties::new();
        match key {
            "div" => {
                res.maybe_set("background-color", self.get_attribute("background-color"));
            }
            _ => (),
        };
        res
    }
}

impl Component for MJBody<'_, '_> {
    fn default_attribute(key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "width" => Some("600".into()),
            _ => None,
        }
    }

    fn node(&self) -> Option<Node> {
        self.node
    }

    fn set_context(&mut self, ctx: Properties) {
        let mut child_ctx = Properties::from(ctx.inner());
        self.context = Some(ctx);
        //
        child_ctx.maybe_set("container-width", self.get_attribute("width"));
        //
        for child in self.children.iter_mut() {
            child.set_context(child_ctx.clone());
        }
    }

    fn render(&self) -> Result<String, Error> {
        debug!("render");
        let mut res: Vec<String> = vec![];
        res.push(open_tag!("body"));
        if self.node.is_some() {
            let mut attrs = Properties::new();
            attrs.maybe_set("class", self.get_attribute("css-class"));
            attrs.set("style", self.get_style("div").as_style());
            res.push(open_tag!("div", attrs.as_attributes()));
            for child in self.children.iter() {
                res.push(child.render()?);
            }
            res.push(close_tag!("div"));
        }
        res.push(close_tag!("body"));
        Ok(res.join(""))
    }
}
