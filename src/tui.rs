use std::time::Duration;

use crate::{crops::crop_registry, harvest_cmd, models::FarmState, persistence};
use humantime::format_duration;
use ratatui_notifications::{Notification, Notifications, Level};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    prelude::Stylize,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    style::{Color, Style}
};

static NAVIGATION_TEXT: &str =
    " Move <Up/Down/Left/Right>, Change Tabs: <Tab/Shift+Tab>, Quit <q> ";

pub fn run() {
    let _ = ratatui::run(|terminal| App::new().run(terminal));
}

#[derive(PartialEq)]
enum Tabs {
    Farm,
    Inventory,
    Market,
}

pub struct App {
    active_tab: Tabs,
    running: bool,
    farm: FarmState,
    notifications: Notifications,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: Tabs::Farm,
            running: true,
            farm: persistence::load_farm(),
            notifications: Notifications::new(),
        }
    }

    fn tab(&mut self, reverse: bool) {
        match self.active_tab {
            Tabs::Farm => match reverse {
                true => self.active_tab = Tabs::Market,
                false => self.active_tab = Tabs::Inventory,
            },
            Tabs::Inventory => match reverse {
                true => self.active_tab = Tabs::Farm,
                false => self.active_tab = Tabs::Market,
            },
            Tabs::Market => match reverse {
                true => self.active_tab = Tabs::Inventory,
                false => self.active_tab = Tabs::Farm,
            },
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            self.keybinds()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.farm = persistence::load_farm();
        let registry = crop_registry();

        let mut farm_vertical_constraints: Vec<Constraint> = Vec::new();
        let mut farm_horizontal_constraints: Vec<Constraint> = Vec::new();
        for _ in &self.farm.plots {
            farm_vertical_constraints.push(Constraint::Length(
                100 / (self.farm.plots.iter().count() as u16),
            ));
            farm_horizontal_constraints.push(Constraint::Length(
                100 / (self.farm.plots.iter().count() as u16),
            ));
        }

        // MARK: master layout
        let master_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(frame.area());

        // MARK: Farm tab layouts
        let farm_vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(farm_vertical_constraints)
            .split(master_layout[1]);
        let farm_horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(farm_horizontal_constraints)
            .split(farm_vertical_layout[0]);

        // MARK: Inventory tab layouts
        let inventory_main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(master_layout[1]);

        match self.active_tab {
            Tabs::Farm => {
                frame.render_widget(
                    Paragraph::new("[Farm] | Inventory | Market").block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Green))
                            .border_type(BorderType::Thick)
                            .title_top(" termfarm ")
                            .title_bottom(Line::from(" Harvest <h>,".to_string() + NAVIGATION_TEXT).right_aligned()),
                    ),
                    master_layout[0],
                );
                for (i, plot) in self.farm.plots.iter().enumerate() {
                    match plot.planted_crop.clone() {
                        Some(crop_id) => {
                            let crop = &registry[&crop_id];
                            let elapsed = plot.planted_at.unwrap().elapsed().unwrap();
                            let remaining = crop.grow_time as i64 - elapsed.as_secs() as i64;
                            let mut color = Color::White;

                            let dur = if remaining <= 0 {
                                color = Color::Green;
                                "ready to harvest".to_string()
                            } else {
                                format_duration(Duration::from_secs(remaining as u64)).to_string()
                                    + " remaining"
                            };
                            let text = format!("{} {}\n{}", crop.id, crop.icon, dur,);
                            frame.render_widget(
                                Paragraph::new(text)
                                    .block(
                                        Block::new()
                                            .borders(Borders::ALL)
                                            .border_style(Style::default().fg(color))
                                            .border_type(BorderType::Double),
                                    )
                                    .wrap(Wrap { trim: true }),
                                farm_horizontal_layout[i],
                            )
                        }
                        None => frame.render_widget(
                            Paragraph::new("<empty>".gray()).block(
                                Block::new()
                                    .borders(Borders::ALL)
                                    .border_style(Style::default().fg(Color::Gray))
                                    .border_type(BorderType::Double),
                            ),
                            farm_horizontal_layout[i],
                        ),
                    };
                }
            }
            Tabs::Inventory => {
                frame.render_widget(
                    Paragraph::new("Farm | [Inventory] | Market").block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Green))
                            .border_type(BorderType::Thick)
                            .title_top(" termfarm ")
                            .title_bottom(Line::from(NAVIGATION_TEXT).right_aligned()),
                    ),
                    master_layout[0],
                );
                frame.render_widget(
                    Paragraph::new(format!(" Coins: {}", self.farm.coins).yellow()).block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow))
                            .border_type(BorderType::Double)
                    ),
                    inventory_main_layout[0]
                )
            },
            Tabs::Market => frame.render_widget(
                Paragraph::new("Farm | Inventory | [Market]").block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green))
                        .border_type(BorderType::Thick)
                        .title_top(" termfarm ")
                        .title_bottom(Line::from(NAVIGATION_TEXT).right_aligned()),
                ),
                master_layout[0],
            ),
        }

        self.notifications.tick(Duration::from_millis(16));
        self.notifications.render(frame, frame.area());
    }

    fn keybinds(&mut self) -> std::io::Result<()> {
        let tick_rate = Duration::from_millis(1);

        if event::poll(tick_rate)? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            persistence::save_farm(&self.farm);
                            self.running = false
                        }
                        KeyCode::Tab => self.tab(false),
                        KeyCode::BackTab => self.tab(true),
                        KeyCode::Char('h') if self.active_tab == Tabs::Farm => {
                            let text = harvest_cmd::harvest(false);
                            let notif = Notification::new(text)
                                .title(" Harvested:")
                                .level(Level::Info)
                                .build()
                                .unwrap();

                            self.notifications.add(notif).unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
