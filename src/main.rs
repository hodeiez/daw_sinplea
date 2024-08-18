mod audio_core;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
            ui.set_decounter(ui.get_decounter() - 1)
        }
    });

    ui.on_record_capture({
     move || {audio_core::start_capture("default");}
    });
        

    ui.run()
}