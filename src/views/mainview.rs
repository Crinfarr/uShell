use ratatui::widgets::StatefulWidget;


#[derive(Debug, Default)]
pub struct MainState {

}
#[derive(Debug, Default)]
pub struct MainView {

}
impl StatefulWidget for MainView {
    type State = MainState;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        
    }
}