use ratatui::{Frame, layout::Rect, style::Stylize, widgets::{Block, Padding, Paragraph}};
use tui_input::Input;

pub fn render_input(frame: &mut Frame, area: Rect, input: &Input,
                title: &str, placeholder: &str, focused: bool) {
    let block = Block::bordered()
        .title(title)
        .padding(Padding::left(1));
    let inner = block.inner(area);

    let width = inner.width.saturating_sub(1);
    let scroll = input.visual_scroll(width as usize);

    let text = match input.value().is_empty() {
        true => placeholder.dark_gray(),
        false => input.value().white(),
    };
    let paragraph = Paragraph::new(text)
        .scroll((0, scroll as u16))
        .block(block);
    frame.render_widget(paragraph, area);

    if focused {
        let x = input.visual_cursor().max(scroll) - scroll;
        frame.set_cursor_position((inner.x + x as u16, inner.y));
    }
}
