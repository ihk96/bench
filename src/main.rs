use ratatui::{
    crossterm::event::{self}
};
use vllm_bench::app::App;


fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let mut app = App::new();

        loop {
            terminal.draw(|frame| app.render(frame))?;

            let event = event::read()?;
            if !app.handle_event(&event) {
                break Ok(());
            }
        }
    })
}
