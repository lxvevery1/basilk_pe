use chrono::Local;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

use crate::{
    json::Json,
    task::{Task, TASK_ITEMS_PE, TASK_STATUS_DONE, TASK_STATUS_ZERO},
    App,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl Project {
    fn get_indicator_done_tasks_color(percentage: usize) -> ratatui::prelude::Color {
        match percentage {
            0 => Color::DarkGray,
            p if (25..=49).contains(&p) => Color::LightMagenta,
            p if (50..99).contains(&p) => Color::LightYellow,
            100 => Color::LightGreen,
            _ => Color::White,
        }
    }

    pub fn load_items(app: &mut App, items: &mut Vec<ListItem>) {
        items.clear();

        for project in app.projects.iter() {
            let tasks = &project.tasks;

            let done_tasks: Vec<Task> = tasks
                .clone()
                .into_iter()
                .filter(|t| t.status == TASK_STATUS_DONE)
                .collect();

            let percentage = if tasks.is_empty() {
                0
            } else {
                (done_tasks.len() * 100) / tasks.len()
            };

            let mut lines = vec![Line::from(vec![
                Span::raw(format!("[{}/{}] ", done_tasks.len(), tasks.len(),)).style(
                    Style::default().fg(Project::get_indicator_done_tasks_color(percentage)),
                ),
                Span::raw(project.title.clone()),
            ])];
            lines.sort_by_key(|line| line.to_string());

            items.push(ListItem::from(lines));
        }

        Project::create_all_projects(app, items);
    }

    pub fn create_all_projects(app: &mut App, items: &mut Vec<ListItem>) {
        let format_str = "%d.%m.%Y";

        let first_date_str = Project::get_first(app).clone().title.to_string();
        let last_date_str = chrono::Local::now().format(format_str).to_string();
        let first_date = chrono::NaiveDate::parse_from_str(&first_date_str, format_str)
            .expect("Invalid first date format!");
        let last_date = Local::now().naive_local().date();

        let mut dates = Vec::new();

        let mut current = first_date;
        while current <= last_date {
            dates.push(current.format(format_str).to_string());
            Project::create(app, items, current.format(format_str).to_string());
            current += chrono::Duration::days(1);
        }
    }

    pub fn reload(app: &mut App, items: &mut Vec<ListItem>) {
        app.projects = Json::read();
        Project::load_items(app, items)
    }

    pub fn get_current(app: &mut App) -> &Project {
        &app.projects[app.selected_project_index.selected().unwrap()]
    }

    pub fn get_first(app: &mut App) -> &Project {
        &app.projects[0]
    }

    pub fn create(app: &mut App, items: &mut Vec<ListItem>, mut value: String) {
        if value.is_empty() {
            let now = chrono::Local::now();
            value = now.format("%d.%m.%Y").to_string();
        }

        // create new project with items from TASK_ITEMS_PE
        let new_project = Project {
            title: value.to_string(),
            tasks: TASK_ITEMS_PE
                .iter()
                .take(3)
                .map(|&item| Task {
                    title: item.to_string(),
                    status: TASK_STATUS_ZERO.to_string(),
                    priority: 0,
                })
                .collect(),
        };

        let mut internal_projects = app.projects.clone();

        // duplicate case
        if internal_projects.contains(&new_project) {
            return;
        }

        internal_projects.push(new_project);

        Json::write(internal_projects);
        Project::reload(app, items);
    }

    pub fn rename(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()].title = value.to_string();

        Json::write(internal_projects);
        Project::reload(app, items)
    }

    pub fn delete(app: &mut App, items: &mut Vec<ListItem>) {
        let mut internal_projects = app.projects.clone();

        internal_projects.remove(app.selected_project_index.selected().unwrap());

        Json::write(internal_projects);
        Project::reload(app, items)
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title // Compare only the title field
    }
}
