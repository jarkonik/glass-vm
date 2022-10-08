#![feature(bigint_helper_methods)]

use crate::cpu::{Run, CPU};
use cpu::{Get, Reg};
use eframe::egui;

mod asm;
mod cpu;

struct App {
    cpu: CPU,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("A {}", self.cpu.get(&Reg::A)));
            ui.label(format!("B {}", self.cpu.get(&Reg::B)));
            ui.label(format!("C {}", self.cpu.get(&Reg::C)));
            ui.label(format!("pc {}", self.cpu.pc()));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let mut cpu = CPU::default();
    cpu.run(&asm::assembly(
        "
        add 1 A
        cmp 255 A

        mov 72 B
        int 0

        mov 69 B
        int 0

        mov 76 B
        int 0

        mov 76 B
        int 0

        mov 79 B
        int 0

        mov 10 B
        int 0

        jne 0
    ",
    ));
    eframe::run_native("VM", options, Box::new(|_cc| Box::new(App { cpu })));
}
