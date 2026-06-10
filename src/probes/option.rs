use crate::{EguiProbe, Style};
use egui::{ComboBox, Response, Ui};

pub struct EguiProbeOption<'a, T, F> {
    pub value: &'a mut Option<T>,
    pub default_some: F,
}

impl<'a, T, F> EguiProbe for EguiProbeOption<'a, T, F>
where
    T: EguiProbe,
    F: FnMut() -> T,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        let Self {
            value,
            default_some,
        } = self;
        option_probe_with(value, ui, style, default_some, |value, ui, style| {
            value.probe(ui, style)
        })
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        if let Some(value) = self.value.as_mut() {
            value.iterate_inner(ui, f);
        }
    }
}

#[inline(always)]
pub fn option_probe_with<T>(
    value: &mut Option<T>,
    ui: &mut Ui,
    style: &Style,
    mut default_some: impl FnMut() -> T,
    probe: impl FnOnce(&mut T, &mut Ui, &Style) -> Response,
) -> Response {
    let mut changed = false;
    let mut response = ui
        .horizontal(|ui| {
            let mut checked = value.is_some();
            // ui.checkbox(&mut checked, ());
            match style.variants {
                crate::style::VariantsStyle::ComboBox => {
                    let selected_text = match value {
                        None => "Disable",
                        Some(_) => "Enable",
                    };
                    ComboBox::from_id_salt(ui.make_persistent_id("ComboBox"))
                        .width(ui.style().spacing.combo_width / 2.0)
                        .selected_text(selected_text)
                        .show_ui(ui, |ui| {
                            if ui.selectable_label(!checked, "Disable").clicked() {
                                checked = false;
                            }
                            if ui.selectable_label(checked, "Enable").clicked() {
                                checked = true;
                            }
                        });
                }
                crate::style::VariantsStyle::Inline => {
                    ui.horizontal(|ui| {
                        if ui.selectable_label(!checked, "Disable").clicked() {
                            checked = false;
                        }
                        if ui.selectable_label(checked, "Enable").clicked() {
                            checked = true;
                        }
                    });
                }
            }

            match (checked, value.is_some()) {
                (true, false) => {
                    *value = Some(default_some());
                    changed = true;
                }
                (false, true) => {
                    *value = None;
                    changed = true;
                }
                _ => {}
            }

            // let id = ui.next_auto_id();
            if let Some(value) = value {
                if probe(value, ui, style).changed() {
                    // State::new(value.clone()).store(ui, id);
                    changed = true;
                }
            } else {
                ui.disable();
                // let mut state = State::load(ui, id).unwrap_or_else(|| State::new(default_some()));
                // probe(&mut state.default_some, ui, style);
                probe(&mut default_some(), ui, style);
            }
        })
        .response;

    if changed {
        response.mark_changed();
    }

    response
}

// struct State<T> {
//     default_some: T,
// }

// impl<T> State<T> {
//     fn new(default_some: T) -> Self {
//         Self { default_some }
//     }
// }

// impl<T: 'static + Clone + Send + Sync> State<T> {
//     fn load(ctx: &Context, id: Id) -> Option<State<T>> {
//         let default_some = ctx.data_mut(|data| data.get_temp(id))?;
//         Some(State { default_some })
//     }

//     fn store(self, ctx: &Context, id: Id) {
//         ctx.data_mut(|data| data.insert_temp(id, self.default_some));
//     }
// }

// impl<T> EguiProbe for Option<T>
// where
//     T: EguiProbe + Default,
// {
//     #[inline(always)]
//     fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
//         option_probe_with(self, ui, style, T::default, |value, ui, style| {
//             value.probe(ui, style)
//         })
//     }
//
//     #[inline(always)]
//     fn iterate_inner(
//         &mut self,
//         ui: &mut egui::Ui,
//         f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
//     ) {
//         if let Some(value) = self {
//             value.iterate_inner(ui, f);
//         }
//     }
// }

// #[inline(always)]
// pub fn option_probe_with<T>(
//     value: &mut Option<T>,
//     ui: &mut egui::Ui,
//     style: &Style,
//     mut default: impl FnMut() -> T,
//     probe: impl FnOnce(&mut T, &mut egui::Ui, &Style) -> egui::Response,
// ) -> egui::Response {
//     let mut changed = false;
//     let mut r = ui
//         .horizontal(|ui| {
//             let mut checked = value.is_some();

//             // ui.checkbox(&mut checked, ());
//             let selected_text = match value {
//                 None => "None",
//                 Some(_) => "Some",
//             };
//             let combo_width = ui.style().spacing.combo_width / 2.0;
//             ComboBox::from_id_salt(ui.make_persistent_id("ComboBox"))
//                 .width(combo_width)
//                 .selected_text(selected_text)
//                 .show_ui(ui, |ui| {
//                     if ui.selectable_label(!checked, "None").clicked() {
//                         checked = false;
//                     }
//                     if ui.selectable_label(checked, "Some").clicked() {
//                         checked = true;
//                     }
//                 });

//             match (checked, value.is_some()) {
//                 (true, false) => {
//                     *value = Some(default());
//                     changed = true;
//                 }
//                 (false, true) => {
//                     *value = None;
//                     changed = true;
//                 }
//                 _ => {}
//             }

//             if let Some(value) = value {
//                 changed |= probe(value, ui, style).changed();
//             }
//         })
//         .response;

//     if changed {
//         r.mark_changed();
//     }

//     r
// }
