use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct Dimensions {
    pub percent_x: u16,
    pub percent_y: u16,
    pub min_x: u16,
    pub min_y: u16,
}

pub fn popup(frame: &mut Frame, dimensions: Dimensions, title: &str, text: &str) {
    let size = frame.area();

    // Define a centered popup size
    let popup_area = centered_rect(dimensions, size);

    // Create the popup widget
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White).bg(Color::Black));

    let text = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);

    // Render the popup
    frame.render_widget(text, popup_area);
}

// Helper function to create a centered rectangle
fn centered_rect(dim: Dimensions, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - dim.percent_y) / 2), // Top padding
            Constraint::Min(dim.min_y),                        // Popup height
            Constraint::Percentage((100 - dim.percent_y) / 2), // Bottom padding
        ])
        .split(r);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - dim.percent_x) / 2), // Left padding
            Constraint::Min(dim.min_x),                        // Popup width
            Constraint::Percentage((100 - dim.percent_x) / 2), // Right padding
        ])
        .split(popup_layout[1]);

    popup_area[1] // The centered area
}
