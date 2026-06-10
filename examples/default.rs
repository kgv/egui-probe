use egui_probe::{Probe, ProbeDefault};
use egui_probe_proc::EguiProbe;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-probe default app",
        native_options,
        Box::new(|cc| Ok(Box::new(EguiProbeDemoApp::new(cc)))),
    )
    .unwrap();
}

#[derive(EguiProbe)]
#[egui_probe(transparent)]
struct UpTo7(#[egui_probe(range = ..=7)] u32);

#[derive(EguiProbe)]
#[egui_probe(tags inlined)]
enum InlinedTags {
    // #[default]
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
    #[egui_probe(default, transparent)]
    Num {
        #[egui_probe(range = 2..=9, default = 2)]
        value: usize,
    },
}

#[derive(EguiProbe)]
struct DemoValue {
    // #[egui_probe(default = true)]
    boolean: bool,

    // #[egui_probe(default = 1.0)]
    float32: f32,

    // #[egui_probe(default = 1)]
    #[egui_probe(tags inlined, range = 0..=9)]
    u8: Option<u8>,

    // #[egui_probe(default = false)]
    maybe_boolean1: Option<bool>,
    #[egui_probe(tags inlined, default = true)]
    maybe_boolean2: Option<bool>,

    // #[egui_probe(default = InlinedTags::InlinedFloat(4.9))]
    // #[egui_probe(default = ProbeDefault::probe_default())]
    #[egui_probe(default)]
    inlined_tags: InlinedTags,
    // #[egui_probe(default = ProbeDefault::probe_default())]
    #[egui_probe(default)]
    option_combobox_tags: Option<ComboBoxTags>,
    //
    // #[egui_probe(with custom_probe)]
    // custom: Foo,

    // // #[egui_probe(skip, name = "renamed ^_^")]
    // #[egui_probe(name = "renamed ^_^")]
    // renamed: u8,

    // // #[egui_probe(default = true)]
    // maybe_boolean: Option<bool>,

    // character: char,

    // inner: InnerValue,

    // array: [u8; 3],

    // vector: Vec<bool>,

    // #[egui_probe(frozen)]
    // frozen_vector: Vec<bool>,

    // map: HashMap<String, u32>,

    // #[egui_probe(frozen)]
    // frozen_map: HashMap<String, u32>,
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
            // value: Default::default(),
            value: DemoValue {
                boolean: true,
                float32: 1.0,
                u8: Some(1),
                maybe_boolean1: None,
                maybe_boolean2: None,
                inlined_tags: InlinedTags::Empty,
                option_combobox_tags: None,
            },
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
