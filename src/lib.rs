//! # Egui Probe
//!
//! Effortlessly create UI widgets to display and modify value types using a derive macro with rich customization via attributes. This library is exclusively for the [egui](https://github.com/emilk/egui) UI framework.
//!
//! ## Features
//!
//! - 🪄 **Derive Macro**: Automatically generate UI widgets for your types.
//! - 🎨 **Rich Customization**: Customize the generated widgets using attributes.
//! - 🚀 **Seamless Integration**: Designed to work seamlessly with egui.
//!
//! ## Getting Started
//!
//! Add `egui_probe` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! egui_probe = "0.5.2"
//! ```
//!
//! ## Usage
//!
//! Derive `EguiProbe` for your types and use attributes to customize the UI:
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(
    not(feature = "derive"),
    doc = "```ignore\n// This example requires the `derive` feature."
)]
//! use egui_probe::{EguiProbe, Probe, angle};
//! use eframe::App;
//!
//! #[derive(EguiProbe)]
//! struct DemoValue {
//!     boolean: bool,
//!
//!     #[egui_probe(toggle_switch)]
//!     boolean_toggle: bool,
//!
//!     float: f32,
//!
//!     #[egui_probe(range = 22..=55)]
//!     range: usize,
//!
//!     #[egui_probe(as angle)]
//!     angle: f32,
//!
//!     #[egui_probe(name = "renamed ^_^")]
//!     renamed: u8,
//!
//!     inner: InnerValue,
//! }
//!
//! #[derive(Default, EguiProbe)]
//! struct InnerValue {
//!     line: String,
//!
//!     #[egui_probe(multiline)]
//!     multi_line: String,
//! }
//!
//! struct EguiProbeDemoApp {
//!     value: DemoValue,
//! }
//!
//! impl App for EguiProbeDemoApp {
//!     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//!         egui::CentralPanel::default().show(ctx, |ui| {
//!             Probe::new(&mut self.value).show(ui);
//!         });
//!     }
//! }
//! ```
//!
//! ## Attributes
//!
//! - `#[egui_probe(toggle_switch)]`: Render a boolean as a toggle switch.
//! - `#[egui_probe(range = 22..=55)]`: Specify a range for numeric values.
//! - `#[egui_probe(as angle)]`: Render a float as an angle.
//! - `#[egui_probe(name = "custom name")]`: Rename the field in the UI.
//! - `#[egui_probe(multiline)]`: Render a string as a multiline text box.
//!
//! ## License
//!
//! This project is licensed under either of
//!
//! - MIT License
//! - Apache License, Version 2.0
//!
//! at your option.
//!
//! ## Contributing
//!
//! Contributions are welcome! Please open an issue or submit a pull request.
//!
//! Enjoy building your UI with Egui Probe! 🚀

#![allow(clippy::inline_always, clippy::use_self)]

mod default;
mod probes;
mod style;
mod ui;
mod widget;

pub use self::{
    default::ProbeDefault,
    probes::{
        boolean::toggle_switch,
        collections::{DeleteMe, EguiProbeDragAndDrop},
        option::{EguiProbeOption, option_probe_with},
    },
    style::{BooleanStyle, Style, VariantsStyle},
    widget::{Probe, ProbeLayout},
};
pub use egui;

/// Provides ability to show probbing UI to values.
pub trait EguiProbe {
    /// Shows probbing UI to edit the value.
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response;

    /// Shows probbing UI to edit the inner values.
    ///
    /// It should add pairs of widgets to the UI for each record.
    /// If record has sub-records it should flatten them.
    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        let _ = (ui, f);
    }
}

impl<P> EguiProbe for &mut P
where
    P: EguiProbe,
{
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        P::probe(*self, ui, style)
    }

    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        P::iterate_inner(*self, ui, f);
    }
}

impl<P> EguiProbe for Box<P>
where
    P: EguiProbe,
{
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        P::probe(&mut *self, ui, style)
    }

    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        P::iterate_inner(&mut *self, ui, f);
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EguiProbeFn<F>(pub F);

impl<F> EguiProbe for EguiProbeFn<F>
where
    F: FnMut(&mut egui::Ui, &Style) -> egui::Response,
{
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        (self.0)(ui, style)
    }
}

/// Wrap a function into probe-able.
#[inline(always)]
pub const fn probe_fn<F>(f: F) -> EguiProbeFn<F> {
    EguiProbeFn(f)
}

#[inline(always)]
pub fn angle(value: &mut f32) -> impl EguiProbe + '_ {
    probe_fn(move |ui: &mut egui::Ui, _style: &Style| ui.drag_angle(value))
}

pub mod customize {
    use super::{
        EguiProbe, EguiProbeOption, Style, egui, probe_fn,
        probes::{
            boolean::ToggleSwitch,
            collections::EguiProbeFrozen,
            color::{
                EguiProbeRgb, EguiProbeRgba, EguiProbeRgbaPremultiplied, EguiProbeRgbaUnmultiplied,
            },
            num::{EguiProbeRange, StepUnset},
            text::EguiProbeMultiline,
        },
    };
    use std::ops::RangeFull;

    #[inline(always)]
    pub fn probe_option<'a, T, F>(value: &'a mut Option<T>, default: F) -> EguiProbeOption<'a, T, F>
    where
        T: EguiProbe,
        F: FnMut() -> T,
    {
        EguiProbeOption { value, default }
    }

    #[inline(always)]
    pub fn probe_with<'a, T, F>(mut f: F, value: &'a mut T) -> impl EguiProbe + 'a
    where
        F: FnMut(&mut T, &mut egui::Ui, &Style) -> egui::Response + 'a,
    {
        probe_fn(move |ui: &mut egui::Ui, style: &Style| f(value, ui, style))
    }

    #[inline(always)]
    pub fn probe_as<'a, T, F, R>(f: F, value: &'a mut T) -> impl EguiProbe + 'a
    where
        F: FnOnce(&'a mut T) -> R,
        R: EguiProbe + 'a,
    {
        f(value)
    }

    #[inline(always)]
    pub const fn probe_range<'a, T, R>(range: R, value: &'a mut T) -> EguiProbeRange<'a, T, R>
    where
        EguiProbeRange<'a, T, R>: EguiProbe,
    {
        EguiProbeRange {
            value,
            range,
            step: StepUnset,
        }
    }

    #[inline(always)]
    pub const fn probe_range_step<'a, T, R, S>(
        range: R,
        step: S,
        value: &'a mut T,
    ) -> EguiProbeRange<'a, T, R, S>
    where
        EguiProbeRange<'a, T, R, S>: EguiProbe,
    {
        EguiProbeRange { value, range, step }
    }

    #[inline(always)]
    pub const fn probe_step<'a, T, S>(
        step: S,
        value: &'a mut T,
    ) -> EguiProbeRange<'a, T, RangeFull, S>
    where
        EguiProbeRange<'a, T, RangeFull, S>: EguiProbe,
    {
        EguiProbeRange {
            value,
            range: ..,
            step,
        }
    }

    #[inline(always)]
    pub const fn probe_multiline<'a, T>(string: &'a mut T) -> EguiProbeMultiline<'a, T>
    where
        EguiProbeMultiline<'a, T>: EguiProbe,
    {
        EguiProbeMultiline { string }
    }

    #[inline(always)]
    pub fn probe_toggle_switch<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        ToggleSwitch<'a, T>: EguiProbe,
    {
        ToggleSwitch(value)
    }

    #[inline(always)]
    pub fn probe_frozen<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeFrozen<'a, T>: EguiProbe,
    {
        EguiProbeFrozen { value }
    }

    #[inline(always)]
    pub fn probe_rgb<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeRgb<'a, T>: EguiProbe,
    {
        EguiProbeRgb { value }
    }

    #[inline(always)]
    pub fn probe_rgba<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeRgba<'a, T>: EguiProbe,
    {
        EguiProbeRgba { value }
    }

    #[inline(always)]
    pub fn probe_rgba_premultiplied<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeRgbaPremultiplied<'a, T>: EguiProbe,
    {
        EguiProbeRgbaPremultiplied { value }
    }

    #[inline(always)]
    pub fn probe_rgba_unmultiplied<'a, T>(value: &'a mut T) -> impl EguiProbe + 'a
    where
        EguiProbeRgbaUnmultiplied<'a, T>: EguiProbe,
    {
        EguiProbeRgbaUnmultiplied { value }
    }
}

#[cfg(feature = "derive")]
pub use egui_probe_proc::EguiProbe;

#[cfg(feature = "derive")]
extern crate self as egui_probe;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub mod private {
    pub use super::customize::*;
    pub use core::stringify;
}

#[cfg(feature = "derive")]
#[test]
fn test_all_attributes() {
    #![allow(unused)]

    trait A {}

    #[derive(EguiProbe)]
    #[egui_probe(where T: EguiProbe)]
    struct TypeAttributes<T> {
        a: T,
    }

    struct NoProbe;

    #[derive(EguiProbe)]
    #[egui_probe(rename_all = Train-Case)]
    struct FieldAttributes {
        #[egui_probe(skip)]
        skipped: NoProbe,

        #[egui_probe(name = "renamed")]
        a: u8,

        #[egui_probe(with |_, ui, _| ui.label("a label"))]
        b: u8,

        #[egui_probe(as angle)]
        c: f32,

        #[egui_probe(range = 0..=100)]
        d: u8,

        #[egui_probe(multiline)]
        e: String,

        #[egui_probe(multiline)]
        f: Option<String>,

        #[egui_probe(toggle_switch)]
        g: bool,

        #[egui_probe(toggle_switch)]
        h: Option<bool>,

        #[egui_probe(frozen)]
        i: Vec<u8>,

        #[egui_probe(rgb)]
        j: egui::Color32,

        #[egui_probe(rgba)]
        k: egui::Color32,

        #[egui_probe(rgba_premultiplied)]
        l: [u8; 4],

        #[egui_probe(rgba_unmultiplied)]
        m: [f32; 4],
    }

    #[derive(EguiProbe)]
    #[egui_probe(tags combobox)]
    enum EnumAttributes {
        #[egui_probe(name = "renamed")]
        A,

        #[egui_probe(transparent)]
        B {
            #[egui_probe(skip)]
            skipped: (),

            b: f32,
        },
    }
}
