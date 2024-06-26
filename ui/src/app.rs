use grad::{
    ast::{ast_to_ascii, ASTNode, Parser},
    chunk::Chunk,
    compiler, debug,
    interner::Interner,
    run_source,
    scanner::Lexer,
    vm::{
        self,
        Result::{self as CompilerResult, CompileErr, Ok as CompilerOk, RuntimeErr},
    },
};

use eframe::egui;
use egui::RichText;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
struct CustomLanguage;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct DisassembledOutput {
    bytecode: Chunk,
    interner: Interner,
}

impl CustomLanguage {
    fn parse(&self, code: &str) -> Result<Vec<ASTNode>, String> {
        let mut lexer = Lexer::new(code.to_string());
        let ast_out = match Parser::new(&mut lexer).parse() {
            Ok(ast) => ast,
            Err(e) => return Err(format!("{:?}", e)),
        };

        Ok(ast_out)
    }

    fn compile(&self, ast: &Vec<ASTNode>) -> DisassembledOutput {
        let mut compiler = compiler::Compiler::new();
        let (bytecode, interner) = compiler.compile(ast.clone());

        DisassembledOutput { bytecode, interner }
    }

    fn execute(&self, compiled: DisassembledOutput) -> String {
        let mut vm = vm::VM::init(compiled.bytecode, compiled.interner);
        let result: CompilerResult = vm.run();

        match result {
            CompilerOk(v) => {
                let mut result = String::new();
                for i in v.iter() {
                    result.push_str(&format!("{:?}\n", i));
                }

                result
            }
            CompileErr(e) => format!("CompileError({:?})", e),
            RuntimeErr(e) => format!("RuntimeError({:?})", e),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CustomLanguageDemo {
    custom_lang: CustomLanguage,
    code: String,
    examples: Vec<(String, String)>,
    selected_example: usize,
    ast: Option<Vec<ASTNode>>,
    disassembled: Option<DisassembledOutput>,
    result: String,
    constants: HashMap<String, String>,
}

impl Default for CustomLanguageDemo {
    fn default() -> Self {
        Self {
            custom_lang: CustomLanguage,
            code: String::new(),
            examples: vec![
                (
                    "Hello World".to_string(),
                    r#"print("Hello World")"#.to_string(),
                ),
                (
                    "Micro".to_string(),
                    "let a = -4.0;
let b = 2.0;
let c = a + b;
let d = a * b + b**3;
c += c + 1;
c += 1 + c + (-a);
print(c);
                    "
                    .to_string(),
                ),
            ],
            selected_example: 0,
            ast: None,
            disassembled: None,
            result: String::new(),
            constants: HashMap::new(),
        }
    }
}

impl eframe::App for CustomLanguageDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let left_panel_width = 400.0;
        let top_panel_height = 300.0;

        // Configure fonts (call this once, typically in the first frame)
        self.configure_fonts(ctx);

        // Left panel (Code editor and Examples)
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(left_panel_width)
            .show(ctx, |ui| {
                use egui::special_emojis::GITHUB;
                ui.hyperlink_to(
                    RichText::new(format!("{GITHUB} grad")).size(16.),
                    "https://github.com/shubhamai/grad",
                );
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Select an example:");
                    egui::ComboBox::from_label("")
                        .selected_text(&self.examples[self.selected_example].0)
                        .show_ui(ui, |ui| {
                            for (idx, (title, example)) in self.examples.iter().enumerate() {
                                if ui
                                    .selectable_value(&mut self.selected_example, idx, title)
                                    .clicked()
                                {
                                    self.code = example.clone()
                                }
                            }
                        });
                });

                ui.add_space(10.0);

                let response = ui.add(
                    egui::TextEdit::multiline(&mut self.code)
                        .desired_width(f32::INFINITY)
                        .desired_rows(25)
                        .code_editor(),
                );

                // if response.changed() {
                //     self.update_output();
                // }

                ui.add_space(10.0);

                if ui.button(RichText::new("Run").size(16.)).clicked() {
                    self.update_output();
                }

                ui.add_space(10.0);

                // if ctrl+enter is pressed, run the code
                // if ui.input(|i| i.key_down(egui::Key::Enter).ctrl()).clicked() {
                //     self.update_output();
                // }

                ui.heading(RichText::new("Execution Result").strong().size(16.));
                ui.add_space(10.0);
                ui.label(&self.result);
            });

        // Right panel (AST and Disassembled Output)
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top right (AST)
            egui::TopBottomPanel::top("ast_panel")
                .resizable(true)
                .default_height(top_panel_height)
                .show_inside(ui, |ui| {
                    ui.heading(RichText::new("Abstract Syntax Tree").strong().size(16.));

                    ui.add_space(10.0);
                    if let Some(_ast) = &self.ast {
                        let mut ast_ascii = String::new();
                        for stmt in _ast.iter() {
                            ast_ascii.push_str(&ast_to_ascii(stmt, 0));
                        }

                        ui.label(ast_ascii);
                    } else {
                        ui.label("No AST available");
                    }
                });

            // Bottom right (Disassembled Output)

            ui.heading(RichText::new("Disassembled Output").strong().size(16.));

            ui.add_space(10.0);
            if let Some(_disassembled) = &self.disassembled {
                let debug = debug::Debug::new(
                    "test",
                    _disassembled.bytecode.clone(),
                    _disassembled.interner.clone(),
                );
                let disassemble_output = debug.disassemble();
                ui.label(disassemble_output);

                // Add hoverable constants
                for (name, value) in &self.constants {
                    ui.add(egui::Label::new(name).sense(egui::Sense::hover()))
                        .on_hover_text(value);
                }
            } else {
                ui.label("No disassembled output available");
            }
        });

        // Bottom panel (Execution Result)
        // egui::TopBottomPanel::bottom("result_panel")
        //     .resizable(true)
        //     .min_height(100.0)
        //     .show(ctx, |ui| {

        //     });
    }
}

impl CustomLanguageDemo {
    fn update_output(&mut self) {
        match self.custom_lang.parse(&self.code) {
            Ok(ast) => {
                self.ast = Some(ast.clone());

                let disassembled_output = self.custom_lang.compile(&ast);
                self.disassembled = Some(disassembled_output.clone());

                self.result = self.custom_lang.execute(disassembled_output);

                // Populate constants (replace this with actual constant extraction)
                // self.constants.clear();
                // self.constants
                //     .insert("CONST_1".to_string(), "42".to_string());
                // self.constants
                //     .insert("CONSTs_2".to_string(), "Hello, World!".to_string());
            }
            Err(e) => {
                self.ast = None;
                self.disassembled = None;
                self.result = format!("Error: {}", e);
                self.constants.clear();
            }
        }
    }

    fn configure_fonts(&self, ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // Load your custom font
        // Note: You need to include your font file in your project's assets
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/SpaceMono-Regular.ttf")),
        );

        // Set the custom font as the default for various text styles
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "my_font".to_owned());

        // Set the configured fonts
        ctx.set_fonts(fonts);
    }
}

impl CustomLanguageDemo {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
