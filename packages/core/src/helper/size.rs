use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq)]
pub enum Size {
    Pixel(Pixel),
    Percent(Percent),
    Raw(f32),
}

impl Size {
    pub fn value(&self) -> f32 {
        match self {
            Self::Pixel(p) => p.value(),
            Self::Percent(p) => p.value(),
            Self::Raw(v) => *v,
        }
    }
}

impl TryFrom<&str> for Size {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.ends_with("px") {
            Ok(Self::Pixel(Pixel::try_from(value)?))
        } else if value.ends_with("%") {
            Ok(Self::Percent(Percent::try_from(value)?))
        } else {
            Ok(Self::Raw(
                value.parse::<f32>().map_err(|err| err.to_string())?,
            ))
        }
    }
}

impl ToString for Size {
    fn to_string(&self) -> String {
        match self {
            Self::Pixel(px) => px.to_string(),
            Self::Percent(prc) => prc.to_string(),
            Self::Raw(value) => value.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pixel(f32);

impl Pixel {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl TryFrom<&str> for Pixel {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.ends_with("px") {
            value[..value.len() - 2]
                .parse::<f32>()
                .map(Pixel::new)
                .map_err(|err| err.to_string())
        } else {
            Err(String::from("pixel value should end with px"))
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self(0.0)
    }
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        format!("{}px", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Percent(f32);

impl Percent {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl TryFrom<&str> for Percent {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.ends_with("%") {
            value[..value.len() - 1]
                .parse::<f32>()
                .map(Percent::new)
                .map_err(|err| err.to_string())
        } else {
            Err(String::from("percent value should end with %"))
        }
    }
}

impl Default for Percent {
    fn default() -> Self {
        Self(0.0)
    }
}

impl ToString for Percent {
    fn to_string(&self) -> String {
        format!("{}%", self.0)
    }
}