use egui_phosphor::regular::{MINUS, PLUS};

#[derive(Clone, Copy, Debug)]
pub enum BooleanStyle {
    Checkbox,
    ToggleSwitch,
}

impl Default for BooleanStyle {
    #[inline]
    fn default() -> Self {
        Self::Checkbox
    }
}

#[derive(Clone, Copy, Debug)]
pub enum VariantsStyle {
    Inlined,
    ComboBox,
}

impl Default for VariantsStyle {
    #[inline]
    fn default() -> Self {
        Self::ComboBox
    }
}

/// Controls the style of probbing UI.
#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub boolean: BooleanStyle,
    pub variants: VariantsStyle,
    pub field_indent_size: Option<f32>,
    pub add_button_text: Option<&'static str>,
    pub remove_button_text: Option<&'static str>,
}

impl Default for Style {
    #[inline]
    fn default() -> Self {
        Style {
            boolean: BooleanStyle::default(),
            variants: VariantsStyle::default(),
            field_indent_size: None,
            add_button_text: None,
            remove_button_text: None,
        }
    }
}

impl Style {
    #[must_use]
    pub fn add_button_text(&self) -> String {
        self.add_button_text.unwrap_or(PLUS).to_string()
    }

    #[must_use]
    pub fn remove_button_text(&self) -> String {
        self.remove_button_text.unwrap_or(MINUS).to_string()
    }
}
