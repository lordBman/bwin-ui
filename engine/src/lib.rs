use dear_imgui_rs::*;

#[unsafe(no_mangle)]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[unsafe(no_mangle)]
fn start() {
    let mut context = Context::create();
    let ui = context.frame();

    render_ui(ui);
}

fn render_ui(ui: &mut Ui) {
    ui.window("Hello world").size([800.0, 480.0], Condition::FirstUseEver).build(|| {
        ui.text("Hello world!");
        ui.text("こんにちは世界！");
        ui.text("This...is...imgui-rs!");
        ui.separator();
        let mouse_pos = ui.io().mouse_pos();
        ui.text(format!(
            "Mouse Position: ({:.1},{:.1})",
            mouse_pos[0], mouse_pos[1]
        ));
    });
}