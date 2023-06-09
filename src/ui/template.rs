// [[file:../../bevy.note::126a69e6][126a69e6]]
use std::collections::HashMap;
use std::path::Path;

use bevy_egui::egui;
use egui::Ui;

use gchemol::Molecule;
use gut::prelude::*;
// 126a69e6 ends here

// [[file:../../bevy.note::b0bc4f80][b0bc4f80]]
fn editable_text(ui: &mut Ui, text: &mut String, label: &str) {
    ui.collapsing(label, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(text)
                    .hint_text(label)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace.resolve(ui.style())),
            );
        });
    })
    .header_response
    .on_hover_text("You are free to edit the input template");
}

// NOTE: read-only
fn selectable_text(ui: &mut Ui, mut text: &str, label: &str, hint: &str) {
    ui.collapsing(label, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut text)
                    .hint_text(label)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace.resolve(ui.style())),
            );
        });
    })
    .header_response
    .on_hover_text(hint);
}
// b0bc4f80 ends here

// [[file:../../bevy.note::897a5556][897a5556]]
use include_dir::{include_dir, Dir, DirEntry};

// the templates loaded from files
static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/tests/files");

fn render_template<S: Serialize>(template: &str, settings: S) -> Result<String> {
    let template = gchemol::io::Template::from_str(template);
    let s = template.render(settings)?;
    Ok(s)
}

#[track_caller]
fn load_templates(tpl_root_dir: &std::path::Path) -> HashMap<String, String> {
    let mut templates = HashMap::new();
    // allow user custom template
    templates.insert("custom".into(), String::new());

    // load minijinja template files
    if let Some(dir) = TEMPLATE_DIR.get_dir(tpl_root_dir) {
        for dir_entry in dir.find("**/*.jinja").unwrap() {
            if let DirEntry::File(f) = dir_entry {
                if let Some(s) = f.path().strip_prefix(tpl_root_dir).ok().and_then(|f| f.to_str()) {
                    let tpl_key = s.to_owned();
                    let tpl_txt = f.contents_utf8().unwrap().to_owned();
                    templates.insert(tpl_key, tpl_txt);
                }
            }
        }
    }
    info!("Loaded {} templates from {:?}", templates.len() - 1, tpl_root_dir);

    templates
}
// 897a5556 ends here

// [[file:../../bevy.note::e3d9d68e][e3d9d68e]]
#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct State {
    current_template: String,
    rendered_input: String,
    input_template: String,
    /// User settings in json format
    json_input: String,
    templates: HashMap<String, String>,
}

impl State {
    pub fn new(template_root_dir: &Path) -> Self {
        let templates = load_templates(template_root_dir);
        Self {
            current_template: "custom".to_owned(),
            templates,
            ..Default::default()
        }
    }
}
// e3d9d68e ends here

// [[file:../../bevy.note::27a033ae][27a033ae]]
impl State {
    pub fn show_template_selection<S: Serialize>(&mut self, settings: S, ui: &mut Ui, mol: Option<&Molecule>) {
        let templates = &self.templates;

        ui.horizontal(|ui| {
            ui.label("Render template:")
                .on_hover_text("Select predefined input templates. Swithc to `custom` for edit.");
            egui::ComboBox::from_id_source("vasp-template")
                .width(200.0)
                .selected_text(&self.current_template)
                .show_ui(ui, |ui| {
                    for t in templates.keys() {
                        ui.selectable_value(&mut self.current_template, t.to_string(), t);
                    }
                });
            // minijinja syntax reference
            ui.hyperlink_to(
                "Template Syntax Reference",
                "https://docs.rs/minijinja/latest/minijinja/syntax/index.html",
            );
        });
        // action button for render and copy to clipboard
        // write rendered input or the error when rendering
        let tooltip = "Click to copy generated input to clipboard";
        if ui.button("📋 Render & Copy").on_hover_text(tooltip).clicked() {
            let mut json_value = serde_json::to_value(settings).ok();
            // append molecule object into user settings
            if let Some(json_object) = json_value.as_mut().and_then(|v| v.as_object_mut()) {
                if let Some(mol) = mol {
                    let mut mol_object = gchemol::io::to_json_value(&mol);
                    json_object.append(mol_object.as_object_mut().unwrap());
                }
                // println!("{}", serde_json::to_string_pretty(&json_object).unwrap());
            }
            match render_template(&self.input_template, &json_value) {
                Ok(s) => {
                    self.rendered_input = s;
                }
                Err(e) => {
                    self.rendered_input = format!("minijinja template render issue:\n{e:?}");
                }
            }
            ui.output_mut(|o| o.copied_text = self.rendered_input.clone());
            // show json input for debug
            self.json_input = serde_json::to_string_pretty(&json_value.unwrap()).unwrap();
        }
        ui.separator();
        selectable_text(
            ui,
            &mut self.json_input.as_str(),
            "JSON input",
            "The json data used for rendering the template",
        );
        match self.current_template.as_str() {
            "custom" => {
                editable_text(ui, &mut self.input_template, "template");
            }
            t => {
                let mut s = templates[t].clone();
                selectable_text(
                    ui,
                    &mut s,
                    "template",
                    "Selected input template in minijinja format for rendering input file",
                );
                self.input_template = s.to_string();
            }
        }

        selectable_text(ui, &mut self.rendered_input.as_str(), "rendered", "Final input file");
    }
}
// 27a033ae ends here
