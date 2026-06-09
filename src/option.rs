use crate::{EguiProbe, Style};

pub struct EguiProbeOption<'a, T, F> {
    pub value: &'a mut Option<T>,
    pub default: F,
}

impl<'a, T, F> EguiProbe for EguiProbeOption<'a, T, F>
where
    T: EguiProbe,
    F: FnMut() -> Option<T>,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        ui.horizontal(|ui| {
            let mut checked = self.value.is_some();
            let mut response = ui.checkbox(&mut checked, "");

            if response.clicked() {
                if checked {
                    *self.value = (self.default)();
                } else {
                    *self.value = None;
                }
                response.mark_changed();
            }

            if let Some(value) = self.value.as_mut() {
                response |= value.probe(ui, style);
            }

            response
        })
        .inner
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

#[inline(always)]
pub fn option_probe_with<T>(
    value: &mut Option<T>,
    ui: &mut egui::Ui,
    style: &Style,
    default: impl FnOnce() -> T,
    probe: impl FnOnce(&mut T, &mut egui::Ui, &Style) -> egui::Response,
) -> egui::Response {
    let mut changed = false;
    let mut r = ui
        .horizontal(|ui| {
            let mut checked = value.is_some();

            ui.checkbox(&mut checked, ());

            match (checked, value.is_some()) {
                (true, false) => {
                    *value = Some(default());
                    changed = true;
                }
                (false, true) => {
                    *value = None;
                    changed = true;
                }
                _ => {}
            }

            if let Some(value) = value {
                changed |= probe(value, ui, style).changed();
            }
        })
        .response;

    if changed {
        r.mark_changed();
    }

    r
}
