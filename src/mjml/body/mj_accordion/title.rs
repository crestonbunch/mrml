use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::{Context, Header, Tag};
use roxmltree::Node;
use std::collections::HashMap;

fn create_default_attributes() -> Attributes {
    Attributes::new()
        .add("font-size", "13px")
        .add("padding", "16px")
}

#[derive(Clone, Debug)]
pub struct MJAccordionTitle {
    attributes: Attributes,
    context: Option<Context>,
    content: String,
}

impl MJAccordionTitle {
    fn default_attributes<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, create_default_attributes())
    }

    pub fn parse<'a, 'b>(
        node: &Node<'a, 'b>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionTitle, Error> {
        if node.tag_name().name() != "mj-accordion-title" {
            return Err(Error::ParseError(format!(
                "element should be 'mj-accordion-title' not '{}'",
                node.tag_name().name()
            )));
        }
        let content: String = node
            .children()
            .filter(|child| child.is_text())
            .filter_map(|child| child.text())
            .collect::<String>();
        let attributes = Self::default_attributes(node, header)
            .concat(attrs)
            .concat(node);
        Ok(MJAccordionTitle {
            attributes,
            context: None,
            content,
        })
    }

    pub fn new(attributes: Attributes) -> Self {
        MJAccordionTitle {
            attributes,
            context: None,
            content: "".into(),
        }
    }

    fn set_style_img(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none")
            .maybe_set_style("height", self.get_attribute("icon-height"))
            .maybe_set_style("width", self.get_attribute("icon-width"))
    }

    fn render_title(&self) -> String {
        Tag::td()
            .maybe_set_style("background-color", self.get_attribute("background-color"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
            .set_style("width", "100%")
            .maybe_set_class(self.get_attribute("css-class"))
            .render(&self.content)
    }

    fn render_icons(&self) -> String {
        let img_more = self
            .set_style_img(Tag::new("img"))
            .maybe_set_attribute("src", self.get_attribute("icon-wrapped-url"))
            .maybe_set_attribute("alt", self.get_attribute("icon-wrapped-alt"))
            .set_class("mj-accordion-more")
            .closed();
        let img_less = self
            .set_style_img(Tag::new("img"))
            .maybe_set_attribute("src", self.get_attribute("icon-unwrapped-url"))
            .maybe_set_attribute("alt", self.get_attribute("icon-unwrapped-alt"))
            .set_class("mj-accordion-less")
            .closed();
        let td = Tag::new("td")
            .set_style("padding", "16px")
            .maybe_set_style("background", self.get_attribute("background-color"))
            .maybe_set_style("vertical-align", self.get_attribute("icon-align"))
            .set_class("mj-accordion-ico")
            .render(img_more + &img_less);
        negation_conditional_tag(td)
    }
}

impl Component for MJAccordionTitle {
    fn update_header(&self, header: &mut Header) {
        header.maybe_add_font_families(self.get_attribute("font-family"));
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let mut content = vec![self.render_title(), self.render_icons()];
        let icon_position = self
            .get_attribute("icon-position")
            .and_then(|value| Some(value.as_str()));
        if icon_position != Some("right") {
            content.reverse();
        }
        let content = content.join("");
        let tr = Tag::tr().render(content);
        let tbody = Tag::tbody().render(tr);
        let table = Tag::table()
            .set_attribute("cellspacing", 0)
            .set_attribute("cellpadding", 0)
            .set_style("width", "100%")
            .maybe_set_style("border-bottom", self.get_attribute("border"))
            .render(tbody);
        Ok(Tag::div().set_class("mj-accordion-title").render(table))
    }
}

impl ComponentWithAttributes for MJAccordionTitle {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes.inner())
    }
}

impl BodyComponent for MJAccordionTitle {}
impl BodyContainedComponent for MJAccordionTitle {}
impl ComponentWithSizeAttribute for MJAccordionTitle {}