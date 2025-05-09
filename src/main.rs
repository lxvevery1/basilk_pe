use std::{
    error::Error,
    fmt::Debug,
    io::{self, stdout},
};

use cli::Cli;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use tui_input::{backend::crossterm::EventHandler, Input};

mod cli;
mod config;
mod json;
mod migration;
mod project;
mod task;
mod ui;
mod util;
mod view;

use config::{Config, ConfigToml};
use json::Json;
use project::Project;
use task::{Task, TASK_PRIORITIES, TASK_STATUSES};
use view::View;

#[derive(Default, PartialEq, Debug)]
pub enum ViewMode {
    #[default]
    ViewProjects,
    RenameProject,
    AddProject,
    DeleteProject,

    ViewTasks,
    RenameTask,
    ChangeStatusTask,
    ChangePriorityTask,
    AddTask,
    DeleteTask,

    InfoMigration,
}

pub struct App {
    // TODO: Better list state mgmt
    selected_project_index: ListState,
    selected_task_index: ListState,
    selected_status_task_index: ListState,
    selected_priority_task_index: ListState,
    view_mode: ViewMode,
    projects: Vec<Project>,
    config: ConfigToml,
}

fn init_terminal() -> Result<Terminal<impl Backend>, Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    Cli::read();

    color_backtrace::install();
    // setup terminal
    let terminal = init_terminal()?;

    // Check the version of the json file
    let were_applied_migrations = Json::check()?;

    // create app and run it
    App::setup().run(terminal, were_applied_migrations)?;

    restore_terminal()?;

    Ok(())
}

impl App {
    fn setup() -> Self {
        Self {
            selected_project_index: ListState::default().with_selected(Some(0)),
            selected_task_index: ListState::default().with_selected(Some(0)),
            selected_status_task_index: ListState::default().with_selected(Some(0)),
            selected_priority_task_index: ListState::default().with_selected(Some(0)),
            view_mode: ViewMode::default(),
            projects: Json::read(),
            config: Config::read(),
        }
    }

    fn run(
        &mut self,
        mut terminal: Terminal<impl Backend>,
        were_applied_migrations: bool,
    ) -> io::Result<()> {
        let mut input = Input::default();

        // only 3 items:
        // pushups
        // squats
        // dumbbell
        let mut items: Vec<ListItem> = vec![];
        Project::load_items(self, &mut items);

        let mut status_items: Vec<ListItem> = vec![];
        Task::load_statuses_items(&mut status_items);

        let mut priority_items: Vec<ListItem> = vec![];
        Task::load_priority_items(&mut priority_items);

        Project::create(self, &mut items, "".to_string());

        if were_applied_migrations {
            self.view_mode = ViewMode::InfoMigration
        }

        loop {
            terminal.draw(|f| {
                self.render(f, f.size(), &input, &items, &status_items, &priority_items)
            })?;

            if let Event::Key(key) = event::read()? {
                // Capture only the "Press" event to prevent double input on Windows
                if key.kind == KeyEventKind::Press {
                    use KeyCode::*;
                    match self.view_mode {
                        ViewMode::ViewProjects => match key.code {
                            Enter | Right | Char('l') => {
                                if items.is_empty() {
                                    continue;
                                }

                                Task::load_items(self, &mut items);
                                self.selected_task_index.select(Some(0));

                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            Char('r') => {
                                if items.is_empty() {
                                    continue;
                                }

                                input = input
                                    .clone()
                                    .with_value(Project::get_current(self).title.clone());

                                App::change_view(self, ViewMode::RenameProject);
                            }
                            Char('a') | Char('n') => {
                                input.reset();
                                App::change_view(self, ViewMode::AddProject);
                            }
                            Char('d') => {
                                if items.is_empty() {
                                    continue;
                                }

                                App::change_view(self, ViewMode::DeleteProject);
                            }
                            Down | Char('j') => {
                                self.next(&items);
                            }
                            Up | Char('k') => {
                                self.previous(&items);
                            }
                            Char('q') => {
                                return Ok(());
                            }
                            _ => {}
                        },
                        ViewMode::RenameProject => match key.code {
                            Enter => {
                                Project::rename(self, &mut items, input.value());
                                input.reset();

                                App::change_view(self, ViewMode::ViewProjects);
                            }
                            Esc => {
                                input.reset();

                                App::change_view(self, ViewMode::ViewProjects);
                            }
                            _ => {
                                input.handle_event(&Event::Key(key));
                            }
                        },
                        ViewMode::AddProject => match key.code {
                            Esc => {
                                App::change_view(self, ViewMode::ViewProjects);
                            }
                            Enter => {
                                Project::create(self, &mut items, input.value().to_string());
                                App::change_view(self, ViewMode::ViewProjects);
                            }
                            _ => {
                                input.handle_event(&Event::Key(key));
                            }
                        },
                        ViewMode::DeleteProject => match key.code {
                            Char('y') => {
                                Project::delete(self, &mut items);
                                self.selected_project_index.select_previous();

                                App::change_view(self, ViewMode::ViewProjects);
                            }
                            Char('n') => {
                                App::change_view(self, ViewMode::ViewProjects);
                            }
                            _ => {}
                        },

                        ViewMode::ViewTasks => match key.code {
                            Esc | Left | Char('h') => {
                                Project::load_items(self, &mut items);

                                App::change_view(self, ViewMode::ViewProjects);
                            }
                            Enter => {
                                if items.is_empty() {
                                    continue;
                                }

                                let index = TASK_STATUSES
                                    .into_iter()
                                    .position(|t| t == Task::get_current(self).status)
                                    .unwrap();

                                self.selected_status_task_index.select(Some(index));

                                App::change_view(self, ViewMode::ChangeStatusTask);
                            }
                            Char('p') => {
                                if items.is_empty() {
                                    continue;
                                }

                                let index = TASK_PRIORITIES
                                    .into_iter()
                                    .position(|t| t == Task::get_current(self).priority)
                                    .unwrap();

                                self.selected_priority_task_index.select(Some(index));

                                App::change_view(self, ViewMode::ChangePriorityTask);
                            }
                            Char('r') => {
                                if items.is_empty() {
                                    continue;
                                }

                                input = input
                                    .clone()
                                    .with_value(Task::get_current(self).title.clone());

                                App::change_view(self, ViewMode::RenameTask);
                            }
                            Char('a') | Char('n') => {
                                input.reset();

                                App::change_view(self, ViewMode::AddTask);
                            }
                            Char('d') => {
                                if items.is_empty() {
                                    continue;
                                }

                                App::change_view(self, ViewMode::DeleteTask);
                            }
                            Down | Char('j') => {
                                self.next(&items);
                            }
                            Up | Char('k') => {
                                self.previous(&items);
                            }
                            Char('q') => {
                                return Ok(());
                            }
                            _ => {}
                        },
                        ViewMode::RenameTask => match key.code {
                            Enter => {
                                Task::rename(self, &mut items, input.value());
                                input.reset();

                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            Esc => {
                                input.reset();

                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            _ => {
                                input.handle_event(&Event::Key(key));
                            }
                        },
                        ViewMode::ChangeStatusTask => match key.code {
                            Enter => {
                                Task::change_status(
                                    self,
                                    &mut items,
                                    TASK_STATUSES
                                        [self.selected_status_task_index.selected().unwrap()],
                                );

                                self.selected_status_task_index.select(Some(0));
                                App::change_view(self, ViewMode::ViewTasks);
                            }

                            Down | Char('j') => {
                                self.next(&status_items);
                            }
                            Up | Char('k') => {
                                self.previous(&status_items);
                            }
                            Esc => {
                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            _ => {}
                        },
                        ViewMode::ChangePriorityTask => match key.code {
                            Enter => {
                                Task::change_priority(
                                    self,
                                    &mut items,
                                    TASK_PRIORITIES
                                        [self.selected_priority_task_index.selected().unwrap()],
                                );

                                self.selected_priority_task_index.select(Some(0));
                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            Down | Char('j') => {
                                self.next(&priority_items);
                            }
                            Up | Char('k') => {
                                self.previous(&priority_items);
                            }
                            Esc => {
                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            _ => {}
                        },
                        ViewMode::AddTask => match key.code {
                            Enter => {
                                Task::create(self, &mut items, input.value());

                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            Esc => {
                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            _ => {
                                input.handle_event(&Event::Key(key));
                            }
                        },
                        ViewMode::DeleteTask => match key.code {
                            Char('y') => {
                                Task::delete(self, &mut items);
                                self.selected_task_index.select_previous();

                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            Char('n') => {
                                App::change_view(self, ViewMode::ViewTasks);
                            }
                            _ => {}
                        },

                        ViewMode::InfoMigration => {
                            App::change_view(self, ViewMode::ViewProjects);
                        }
                    }
                }
            }
        }
    }

    fn render(
        &mut self,
        f: &mut Frame,
        area: Rect,
        input: &Input,
        items: &[ListItem],
        status_items: &[ListItem],
        priority_items: &[ListItem],
    ) {
        // defaults
        let header_area = 2;
        let mut list_area = 73;
        let mut grid_area = 22;
        let mut help_area = 2;

        if !self.config.ui.show_help {
            list_area += help_area;
            help_area = 0;
        } else if !self.config.ui.show_grid_activity {
            list_area += grid_area;
            grid_area = 0;
        }

        let layout = Layout::vertical([
            Constraint::Percentage(header_area),
            Constraint::Percentage(list_area),
            Constraint::Percentage(grid_area),
            // Space for the footer helper
            Constraint::Percentage(help_area),
        ]);

        let [header_area, rest_area, grid_activity_area, footer_area] = layout.areas(area);

        View::show_items(self, items, f, rest_area);

        if self.view_mode == ViewMode::InfoMigration {
            View::show_migration_info_modal(f, area);
        }

        if self.view_mode == ViewMode::AddTask || self.view_mode == ViewMode::AddProject {
            View::show_new_item_modal(f, area, input)
        }

        if self.view_mode == ViewMode::RenameTask || self.view_mode == ViewMode::RenameProject {
            View::show_rename_item_modal(f, area, input)
        }

        if self.view_mode == ViewMode::DeleteTask || self.view_mode == ViewMode::DeleteProject {
            View::show_delete_item_modal(self, f, area)
        }

        if self.view_mode == ViewMode::ChangeStatusTask {
            View::show_select_task_status_modal(self, status_items, f, area)
        }

        if self.view_mode == ViewMode::ChangePriorityTask {
            View::show_select_task_priority_modal(self, priority_items, f, area)
        }

        f.render_widget(
            Paragraph::new(format!("::{}::", env!("CARGO_PKG_NAME"))).centered(),
            header_area,
        );

        if self.config.ui.show_grid_activity {
            View::show_grid_activity(self, f, grid_activity_area);
        }

        if self.config.ui.show_help {
            View::show_footer_helper(self, f, footer_area)
        }
    }

    fn next(&mut self, items: &[ListItem]) {
        let i = match self.use_state().selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.use_state().select(Some(i))
    }

    fn previous(&mut self, items: &[ListItem]) {
        let i = match self.use_state().selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.use_state().select(Some(i))
    }

    fn use_state(&mut self) -> &mut ListState {
        match self.view_mode {
            ViewMode::ViewProjects
            | ViewMode::RenameProject
            | ViewMode::AddProject
            | ViewMode::DeleteProject => &mut self.selected_project_index,

            ViewMode::ViewTasks => &mut self.selected_task_index,
            ViewMode::RenameTask => &mut self.selected_task_index,
            ViewMode::ChangeStatusTask => &mut self.selected_status_task_index,
            ViewMode::ChangePriorityTask => &mut self.selected_priority_task_index,
            ViewMode::AddTask => &mut self.selected_task_index,
            ViewMode::DeleteTask => &mut self.selected_task_index,

            ViewMode::InfoMigration => &mut self.selected_project_index,
        }
    }

    fn change_view(&mut self, mode: ViewMode) {
        self.view_mode = mode
    }
}
