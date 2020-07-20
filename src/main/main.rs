use tui::{backend::CrosstermBackend, Terminal};

mod state;
mod structure;
mod widgets;

fn main() {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
    t.clear().unwrap();

    loop {
        t.draw(|mut f| {
            let size = f.size();
            structure::APP.render(&mut f, size);
        })
        .unwrap()
    }
}
