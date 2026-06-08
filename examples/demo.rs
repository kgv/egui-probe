use egui_probe::{Probe, angle};
use egui_probe_proc::EguiProbe;
use std::collections::HashMap;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-probe demo app",
        native_options,
        Box::new(|cc| Ok(Box::new(EguiProbeDemoApp::new(cc)))),
    )
    .unwrap();
}

#[derive(Default)]
struct Foo;

fn custom_probe(_: &mut Foo, ui: &mut egui::Ui, _: &egui_probe::Style) -> egui::Response {
    ui.label("This is custom probe")
}

#[derive(EguiProbe)]
#[egui_probe(transparent)]
struct UpTo7(#[egui_probe(range = ..=7)] u32);

#[derive(EguiProbe)]
#[egui_probe(tags inlined)]
enum InlinedTags {
    Empty,

    #[egui_probe(transparent)]
    InlinedFloat(#[egui_probe(default = 999.0)] f32),

    Text {
        #[egui_probe(default = String::from("FROM"), multiline)]
        text: String,
    },
}

#[derive(EguiProbe)]
#[egui_probe(tags combobox)]
enum ComboBoxTags {
    Empty,
    #[egui_probe(default = ComboBoxTags::Num { value: 2 })]
    Num {
        value: usize,
    },
}

// impl Default for ComboBoxTags {
//     fn default() -> Self {
//         ComboBoxTags::Empty
//     }
// }

#[derive(EguiProbe)]
struct InnerValue {
    line: String,

    #[egui_probe(multiline)]
    multi_line: String,
}

#[derive(EguiProbe)]
struct DemoValue {
    boolean: bool,

    #[egui_probe(toggle_switch)]
    boolean_toggle: bool,

    #[egui_probe(range = 0.0..=1.0 by 0.01, bookmarks = [0.5])]
    float: f32,

    #[egui_probe(range = 22..=55)]
    range: usize,

    range_to: UpTo7,

    #[egui_probe(range = 50..)]
    range_from: u8,

    #[egui_probe(range = 1..=9, default = 9, bookmarks = [2, 3, 4])]
    range_with_bookmark: u8,

    #[egui_probe(as angle)]
    angle: f32,

    #[egui_probe(with custom_probe)]
    custom: Foo,

    // #[egui_probe(skip, name = "renamed ^_^")]
    #[egui_probe(name = "renamed ^_^")]
    renamed: u8,

    maybe_boolean: Option<bool>,

    character: char,

    inner: InnerValue,

    #[egui_probe(default = InlinedTags::Empty)]
    inlined_tags: InlinedTags,

    #[egui_probe(default = Some(ComboBoxTags::Num { value: 9 }))]
    option_combobox_tags: Option<ComboBoxTags>,

    array: [u8; 3],

    vector: Vec<bool>,

    #[egui_probe(frozen)]
    frozen_vector: Vec<bool>,

    map: HashMap<String, u32>,

    #[egui_probe(frozen)]
    frozen_map: HashMap<String, u32>,
}

struct EguiProbeDemoApp {
    value: DemoValue,
}

impl EguiProbeDemoApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);
        EguiProbeDemoApp {
            value: DemoValue::default(),
            // value: DemoValue {
            //     boolean: false,
            //     boolean_toggle: false,
            //     float: 0.0,
            //     range: 22,
            //     range_to: UpTo7(0),
            //     range_from: 100,
            //     range_with_bookmark: 2,
            //     angle: 0.0,
            //     custom: Foo,
            //     renamed: 0,
            //     maybe_boolean: None,
            //     character: 'a',
            //     inner: InnerValue {
            //         line: "Hello, world!".to_owned(),
            //         multi_line: "Hello,\nworld!".to_owned(),
            //     },
            //     inlined_tags: InlinedTags::Empty,
            //     option_combobox_tags: None,
            //     array: [0, 1, 2],
            //     vector: vec![false, true, false],
            //     frozen_vector: vec![false, true, false],

            //     map: {
            //         let mut map = HashMap::new();
            //         map.insert("foo".to_owned(), 1);
            //         map.insert("bar".to_owned(), 2);
            //         map
            //     },

            //     frozen_map: {
            //         let mut map = HashMap::new();
            //         map.insert("foo".to_owned(), 1);
            //         map.insert("bar".to_owned(), 2);
            //         map
            //     },
            // },
        }
    }
}

impl eframe::App for EguiProbeDemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("Header").show_inside(ui, |ui| {
            egui::widgets::global_theme_preference_switch(ui);
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                Probe::new(&mut self.value).show(ui);
            });
        });
    }
}
