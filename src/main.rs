use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::{io, time::Duration, vec};

mod tui;
#[derive(Debug, Default)]
pub enum State{
    #[default]
    Menu,
    Game,
    GameOver,
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    state: State,
    game: Game,
}


impl App {
    //runs the application's main loop until quit
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {

        self.state = State::Game;
        self.game = Game::default();

        while !self.exit{
            self.update()?;
            
            self.handle_events()?;
             
            terminal.draw(|frame| self.render_frame(frame))?;
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        match(self.state){
            State::Menu => {
                //self.update_menu()?;
            }
            State::Game => {
                self.game.update();
            }
            State::GameOver => {
                //self.update_game_over()?;
            }
        }
        Ok(())
    }

    fn render_frame(&self, frame:&mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {

        if event::poll(Duration::from_millis(32))? {
            // It's guaranteed that `read` won't block, because `poll` returned
            // `Ok(true)`.
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(90), Constraint::Min(10)].as_ref())
            .split(area);

        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .red();

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(layout[1], buf);

        match self.state {
            State::Menu => {
                //self.render_menu(layout[0], buf);
            }
            State::Game => {
                self.game.render(layout[0], buf);
            }
            State::GameOver => {
                //self.render_game_over(layout[0], buf);
            }
        }

        
    }
}

#[derive(Debug)]
struct Game{
    //ball position + velocity
    ball: (f32, f32),
    ball_v: (f32, f32),

    //player 1 position + velocity
    p1: (f32, f32),
    p1_v: f32,

    //player 2 position + velocity
    p2: (f32, f32),
    p2_v: f32,
}

impl Game{
    fn update(&mut self) {

        if(self.ball.0 + self.ball_v.0) > 1.0 || (self.ball.0 + self.ball_v.0) < 0.0 {
            self.ball_v.0 = -self.ball_v.0;
        }
        self.ball.0 += self.ball_v.0;

        if(self.ball.1 + self.ball_v.1) > 1.0 || (self.ball.1 + self.ball_v.1) < 0.0 {
            self.ball_v.1 = -self.ball_v.1;
        }
        self.ball.1 += self.ball_v.1;
    }

}

impl Default for Game {
    fn default() -> Self {
        Self {
            ball: (0.0, 0.0),
            ball_v: (0.015, 0.024),
            p1: (0.0, 0.0),
            p1_v: 0.0,
            p2: (0.0, 0.0),
            p2_v: 0.0,
        }
    }
}

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let ball_text_styles: Vec<Vec<(&str, &str)>> = vec!(
            vec!(("▄", "▀"), ("▗▖", "▝▘"), ("▄", "▀")),
            vec!(("▐▌", ""), ("█", ""), ("▐▌", "")),
            vec!(("▄", "▀"), ("▗▖", "▝▘"), ("▄", "▀"),
        );
            
        let denormalized_x = self.ball.0 * 100.0;
        let mut x_coord = 0;

        if denormalized_x - denormalized_x.floor() > 0.33 && denormalized_x - denormalized_x.floor() < 0.66{
            x_coord = 1;
        }else if denormalized_x - denormalized_x.floor() > 0.66{
            x_coord = 2;
        }else{
            x_coord = 0;
        }


        let denormalized_y = self.ball.1 * Frame::Height as f32;
        let mut y_coord = 0;

        if denormalized_y - denormalized_y.floor() > 0.33 && denormalized_y - denormalized_y.floor() < 0.66{
            y_coord = 1;
        }else if denormalized_y - denormalized_y.floor() > 0.66{
            y_coord = 2;
        }else{
            y_coord = 0;
        }

        let ball_text = ball_text_styles[y_coord][x_coord];
        buf.set_string(area.x + (self.ball.0 * area.width as f32) as u16 + (x_coord - 1) , area.y + (self.ball.1 * area.height as f32) as u16 - (y_coord - 1) , ball_text, Style::default());
    }


}




fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}