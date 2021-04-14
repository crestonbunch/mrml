use crate::mj_body::MJBody;
use crate::mj_head::MJHead;

#[cfg(feature = "parse")]
pub mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mjml";

#[derive(Debug, Default)]
pub struct MJML {
    head: Option<MJHead>,
    body: Option<MJBody>,
}

impl MJML {
    pub fn body(&self) -> Option<&MJBody> {
        self.body.as_ref()
    }

    pub fn head(&self) -> Option<&MJHead> {
        self.head.as_ref()
    }
}