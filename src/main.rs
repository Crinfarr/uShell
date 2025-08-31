mod views;

use ratatui::widgets::StatefulWidget;

use crate::views::{MainState, MainView};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let term = ratatui::init();
    let mut app = AppCore::default();
    let result = app.mainloop(term).await;

    ratatui::restore();
    result
}

#[derive(Debug, Default)]
struct AppCore {
    appstate: MainState,
    mainview: MainView
}
impl AppCore {
    async fn mainloop(&mut self, mut term:ratatui::DefaultTerminal) -> std::io::Result<()> {
        loop {
            MainView::default().render(term.get_frame().area(), term.current_buffer_mut(), &mut self.appstate);
        }
        Ok(())
    }
}