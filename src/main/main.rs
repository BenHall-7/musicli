use tui::layout::{Constraint, Direction, Layout};
use tui::style::*;
use tui::{backend::CrosstermBackend, Terminal};

mod state;

use state::{Area, WidgetType};

fn main() {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
    t.clear().unwrap();

    loop {
        t.draw(|mut f| {
            let size = f.size();
            let layout = Layout::default()
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(size);
            let areas = Area::SubArea(vec![
                (
                    layout[0],
                    Area::Widget(WidgetType::List(
                        vec![String::from("first item"), String::from("second item")],
                        None,
                    )),
                ),
                (
                    layout[1],
                    Area::Widget(WidgetType::Paragraph(
                        vec![String::from("first item"), String::from("second item")],
                        None,
                    )),
                ),
            ]);
            areas.render(&mut f, size)
        })
        .unwrap()
    }
}
