use super::Node;
use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::{Component, Error};
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl Node {
    fn closed_element(&self) -> bool {
        self.children.is_empty() && ["img"].contains(&self.name.as_str())
    }
}

impl Component for Node {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let tag = Tag::new(self.name.as_str()).insert_attributes(self.attributes.inner());
        if self.closed_element() {
            Ok(tag.closed())
        } else {
            let mut res = vec![];
            for child in self.children.iter() {
                res.push(child.render(header)?);
            }
            Ok(tag.render(res.join("")))
        }
    }
}

impl BodyComponent for Node {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }
}