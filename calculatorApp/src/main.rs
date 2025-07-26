use eframe::egui;
use meval::eval_str;
use std::{error::Error, option};

struct CalculatorApp {
    expresssion: String,
    result: String,
}

impl Default for CalculatorApp
{
    fn default() -> Self {
        Self { expresssion: String::new(), result: String::new(), }
    }
}

impl eframe::App for CalculatorApp {

    fn update(&mut self, ctx: &egui::Context, _ : &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Rust Calculator");
            ui.text_edit_singleline(&mut self.expresssion);

            if ui.button("Calculate").clicked(){
                self.result = match eval_expr(&self.expresssion)
                {
                    Ok(val)  => val.to_string(),
                    Err(e) => format!("Error {}", e),
                };
            }
            ui.label(format!("Result {}", self.result));
        });
    }
    
}


fn eval_expr(expr: &str) -> Result<f64, String>{
    meval::eval_str(expr).map_err(|e| e.to_string())
}

fn main() -> Result<(), eframe::Error>{

    let option = eframe::NativeOptions::default();

    eframe::run_native(
        "Calculator App", option, 
        Box::new(|_cc| {
        Ok::<Box<dyn eframe::App>, Box<dyn Error + Send + Sync>>(Box::new(CalculatorApp::default()))      
        }),
    
    )}
