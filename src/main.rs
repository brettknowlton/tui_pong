use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use game_input::{GameInputState, PaddleState};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::{cmp::{max, min}, io, ops::Range, time::Duration, vec};

mod tui;
mod game_input;

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
    input_state: game_input::GameInputState,
}


impl App {
    //runs the application's main loop until quit
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {

        self.state = State::Game;
        self.game = Game::default();
        self.input_state = game_input::GameInputState::default();

        while !self.exit{
            self.update()?;
            
            self.handle_events()?;
            
            terminal.draw(|frame| self.render_frame(frame))?;
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        match self.state{
            State::Menu => {
                //self.update_menu()?;
            }
            State::Game => {
                self.game.update(self.input_state);
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

        if event::poll(Duration::from_millis(82))? {
            // It's guaranteed that `read` won't block, because `poll` returned
            // `Ok(true)`.
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if (key_event.kind == KeyEventKind::Press) || (key_event.kind == KeyEventKind::Release) => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.kind{
            KeyEventKind::Press =>{
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Left => self.decrement_counter(),
                    KeyCode::Right => self.increment_counter(),
        
                    // KeyCode::W => self.
                    _ => {}
                }
            },
            KeyEventKind::Release=>{
                match key_event.code{
                    _ => {}
                }
            },
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
        let layoutH = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(100), Constraint::Min(10)].as_ref())
            .split(area);

        let layoutV = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(30), Constraint::Min(10)].as_ref())
            .split(layoutH[0]);

        let title = Title::from(" Stats : ".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .white();

        let counter_text = Text::from(vec![Line::from(vec![
            "Ball_x: ".into(),
            self.game.ball.0.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(layoutH[1], buf);

        match self.state {
            State::Menu => {
                //self.render_menu(layout[0], buf);
            }
            State::Game => {
                self.game.render(layoutV[0], buf);
            }
            State::GameOver => {
                //self.render_game_over(layout[0], buf);
            }
        }

        
    }
}

#[derive(Debug)]
struct Game{
    score: (u8, u8),
    //ball position + velocity
    ball: (f32, f32),
    ball_v: (f32, f32),

    //player 1 position + velocity
    p1_pos: (f32, f32),
    p1_v: f32,

    //player 2 position + velocity
    p2_pos: (f32, f32),
    p2_v: f32,
}
impl Default for Game {
    fn default() -> Self {
        Self {
            score: (0, 0),

            ball: (0.0, 0.0),//the "top-left" of the ball
            ball_v: (0.055, 0.0275),

            p1_pos: (0.0, 0.5),//the "top-left" of player 1's (left) paddle (normalized)
            p1_v: 0.0,

            p2_pos: (0.0, 0.5),//the "center" of player 2's (left) paddle (normalized)
            p2_v: 0.0,
        }
    }
}

impl Game{
    fn update(&mut self, input: GameInputState) {
        self.collide_ball_with_border();
        self.update_player_paddles(input);
    }

    fn update_player_paddles(&mut self, input: GameInputState){
        match input.p1_paddle_state {
            PaddleState::MovingUp => {
                self.p1_pos.1 += 0.22;
                self.p1_pos.1 = self.p1_pos.1.clamp(0.0, 1.0);
            },
            PaddleState::MovingDown => {
                self.p1_pos.1 -= 0.22;
            }
            _ => {}
        }
    }

    fn collide_ball_with_border(&mut self){
        if(self.ball.0 + self.ball_v.0) >= 1.0 || (self.ball.0 + self.ball_v.0) < 0.0 {
            self.ball_v.0 = -self.ball_v.0;
        }
        self.ball.0 += self.ball_v.0;

        if(self.ball.1 + self.ball_v.1) >= 1.0 || (self.ball.1 + self.ball_v.1) <= 0.0 {
            self.ball_v.1 = -self.ball_v.1;
        }
        self.ball.1 += self.ball_v.1;
    }

    fn draw_ball(&self, area: Rect, buf: &mut Buffer) {
        let ball_text_styles: Vec<Vec<(&str, &str)>> = vec!(
            vec!(("▗▄", "▝▀"), ("▗▄▖", "▝▀▘"), ("▄▖", "▀▘")),
            vec!(("▐█▌", ""), ("██", ""), ("▐█▌", "")),
            vec!(("▗▄", "▝▀"), ("▗▄▖", "▝▀▘"), ("▄▖", "▀▘")),
        );
            
        let denormalized_x = self.ball.0 * area.width as f32;
        let x_coord: i16;

        if denormalized_x - denormalized_x.floor() > 0.33 && denormalized_x - denormalized_x.floor() < 0.66{
            x_coord = 0;
        }else if denormalized_x - denormalized_x.floor() > 0.66{
            x_coord = 1;
        }else{
            x_coord = -1;
        }


        let denormalized_y = self.ball.1 * area.height as f32;
        let mut y_coord: i16;

        if denormalized_y - denormalized_y.floor() > 0.33 && denormalized_y - denormalized_y.floor() < 0.66{
            y_coord = 1;
        }else if denormalized_y - denormalized_y.floor() > 0.66{
            y_coord = 2;
        }else{
            y_coord = 0;
        }

        let ball_text = ball_text_styles[y_coord as usize][(x_coord + 1) as usize];

        y_coord = min(y_coord, 1);

        let line1 = ball_text.0;
        let line1_x = max((self.ball.0 * area.width as f32) as i16 + (x_coord - 1) as i16 - 2, 0);
        let line1_y = ((self.ball.1 * area.height as f32) as i16 + y_coord - 1).clamp(1, area.height as i16 - 2) as u16;
        buf.set_string(line1_x as u16, line1_y, line1, Color::Yellow);

        let line2 = ball_text.1;
        let line2_x = max((self.ball.0 * area.width as f32) as i16 + (x_coord - 1) as i16 - 2, 0);
        let line2_y = ((self.ball.1 * area.height as f32) as i16 + y_coord).clamp(1, area.height as i16 - 2) as u16;
        buf.set_string(line2_x as u16, line2_y, line2, Color::Yellow);
    }

    fn draw_paddles(&self, area: Rect, buf: &mut Buffer){
        let paddle_top = "┓";
        let paddle_mid = "┃";
        let paddle_bot = "┛";

        let mut x: i16;
        let p1y_denormalized = (self.p1_pos.1 * area.height as f32) as i16;

        for i in (Range{start: 0, end: 3}){

            let paddle_text;

            if i == 0 {
                paddle_text = paddle_top;
            }else if i==3{
                paddle_text = paddle_bot;
            }else{
                paddle_text = paddle_mid;
            }


            let line_x = 1;
            let mut line_y = i - 2 + p1y_denormalized;

            line_y = line_y.clamp(0, area.width as i16);
            

            buf.set_string(line_x as u16, line_y as u16, paddle_text, Color::Yellow);
        }

        

    }
}



impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //draw ball

        let title = Title::from(format!("({}) - ({})", self.score.0, self.score.1).bold());
        let description = Title::from(Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                description
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .white()
            .render(area, buf);

        Game::draw_ball(self, area, buf);
        Game::draw_paddles(self, area, buf);
        
    }

}




fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}