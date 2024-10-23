use serde::{Serialize, Deserialize};

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{ColorBy, CoordinateSystem, Color, BorderType},
};


#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickItemStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color0: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color0: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color_doji: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_radius: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_type: Option<BorderType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_blur: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_y: Option<f64>,
}

impl Default for CandlestickItemStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl CandlestickItemStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            color0: None,
            border_color: None,
            border_color0: None,
            border_color_doji: None,
            border_width: None,
            border_radius: None,
            border_type: None,
            opacity: None,
            shadow_color: None,
            shadow_blur: None,
            shadow_offset_x: None,
            shadow_offset_y: None,
        }
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }
    pub fn color0<C: Into<Color>>(mut self, color: C) -> Self {
        self.color0 = Some(color.into());
        self
    }

    pub fn border_color<C: Into<Color>>(mut self, border_color: C) -> Self {
        self.border_color = Some(border_color.into());
        self
    }

    pub fn border_color0<C: Into<Color>>(mut self, border_color: C) -> Self {
        self.border_color0 = Some(border_color.into());
        self
    }

    pub fn border_color_doji<C: Into<Color>>(mut self, border_color: C) -> Self {
        self.border_color_doji = Some(border_color.into());
        self
    }

    pub fn border_width<F: Into<f64>>(mut self, border_width: F) -> Self {
        self.border_width = Some(border_width.into());
        self
    }

    pub fn border_radius<F: Into<f64>>(mut self, border_radius: F) -> Self {
        self.border_radius = Some(border_radius.into());
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = Some(border_type);
        self
    }

    pub fn opacity<F: Into<f64>>(mut self, opacity: F) -> Self {
        self.opacity = Some(opacity.into());
        self
    }

    pub fn shadow_color<C: Into<Color>>(mut self, shadow_color: C) -> Self {
        self.shadow_color = Some(shadow_color.into());
        self
    }

    pub fn shadow_blur<F: Into<f64>>(mut self, shadow_blur: F) -> Self {
        self.shadow_blur = Some(shadow_blur.into());
        self
    }

    pub fn shadow_offset_x<F: Into<f64>>(mut self, shadow_offset_x: F) -> Self {
        self.shadow_offset_x = Some(shadow_offset_x.into());
        self
    }

    pub fn shadow_offset_y<F: Into<f64>>(mut self, shadow_offset_y: F) -> Self {
        self.shadow_offset_y = Some(shadow_offset_y.into());
        self
    }
}

impl From<Color> for CandlestickItemStyle {
    fn from(color: Color) -> Self {
        Self::new().color(color)
    }
}



#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Candlestick {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordiate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<CandlestickItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: DataFrame,
}

impl Default for Candlestick {
    fn default() -> Self {
        Self::new()
    }
}

impl Candlestick {
    pub fn new() -> Self {
        Self {
            type_: "candlestick".to_string(),
            id: None,
            name: None,
            coordiate_system: None,
            color_by: None,
            legend_hover_link: None,
            item_style: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn coordiate_system<C: Into<CoordinateSystem>>(mut self, coordiate_system: C) -> Self {
        self.coordiate_system = Some(coordiate_system.into());
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn item_style(mut self, item_style: CandlestickItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
