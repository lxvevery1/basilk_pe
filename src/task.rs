use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

use crate::{json::Json, util::Util, App};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub status: String,
    pub priority: u8,
}

pub const TASK_STATUS_ZERO: &str = "0";
pub const TASK_STATUS_QUARTER: &str = "25";
pub const TASK_STATUS_HALF: &str = "50";
pub const TASK_STATUS_TREE_QUARTER: &str = "75";
pub const TASK_STATUS_DONE: &str = "100";

const TASK_STATUSES_SORT_ORDER: [&str; 5] = [
    TASK_STATUS_ZERO,
    TASK_STATUS_QUARTER,
    TASK_STATUS_HALF,
    TASK_STATUS_TREE_QUARTER,
    TASK_STATUS_DONE,
];
pub const TASK_STATUSES: [&str; 5] = [
    TASK_STATUS_ZERO,
    TASK_STATUS_QUARTER,
    TASK_STATUS_HALF,
    TASK_STATUS_TREE_QUARTER,
    TASK_STATUS_DONE,
];
pub const TASK_ITEMS_PE: [&str; 3] = ["pushups", "squats", "dumbbell"];

// Ascending order: 1 highest priority; 2 medium; 3 lowest
pub const TASK_PRIORITIES: [u8; 4] = [1, 2, 3, 0];

impl Task {
    fn get_status_color(status: &str) -> ratatui::prelude::Color {
        match status {
            TASK_STATUS_ZERO => Color::Gray,
            TASK_STATUS_QUARTER => Color::White,
            TASK_STATUS_HALF => Color::LightBlue,
            TASK_STATUS_TREE_QUARTER => Color::LightMagenta,
            TASK_STATUS_DONE => Color::LightYellow,
            _ => Color::Gray,
        }
    }

    pub fn load_statuses_items(items: &mut Vec<ListItem>) {
        items.clear();

        for status in TASK_STATUSES {
            let span = Span::styled(status, Style::new().fg(Task::get_status_color(status)));

            items.push(ListItem::from(span))
        }
    }

    pub fn load_priority_items(items: &mut Vec<ListItem>) {
        items.clear();

        for priority_value in TASK_PRIORITIES {
            let span = Span::styled(
                Util::get_priority_indicator(priority_value),
                Style::new().fg(Color::Red),
            );

            items.push(ListItem::from(span))
        }
    }

    pub fn load_items(app: &mut App, items: &mut Vec<ListItem>) {
        let tasks = &mut app.projects[app.selected_project_index.selected().unwrap()].tasks;

        let last_task_title_selected = tasks
            .clone()
            .get(app.selected_task_index.selected().unwrap_or(0))
            .unwrap_or(&Task {
                title: "".to_string(),
                status: "".to_string(),
                priority: 0,
            })
            .clone()
            .title;

        // Sort by status
        tasks.sort_by_key(|t| {
            TASK_STATUSES_SORT_ORDER
                .into_iter()
                .position(|o| o == t.status)
        });

        // Sort by priority
        tasks.sort_by_key(|t| TASK_PRIORITIES.into_iter().position(|o| o == t.priority));

        let new_index = tasks
            .iter_mut()
            .position(|t| t.title == last_task_title_selected)
            .unwrap_or(0);

        items.clear();

        for task in tasks.iter() {
            let modifier = if task.status == TASK_STATUS_DONE {
                Modifier::CROSSED_OUT
            } else {
                Modifier::empty()
            };

            let mut repr = vec![
                Span::styled(
                    format!("[{}] ", task.status),
                    Style::default()
                        .fg(Task::get_status_color(&task.status))
                        .add_modifier(modifier),
                ),
                Span::styled(task.title.clone(), Style::default().add_modifier(modifier)),
            ];

            if task.priority != 0 {
                let priority_repr = vec![Span::styled(
                    format!("[{}] ", Util::get_priority_indicator(task.priority)),
                    Style::new().fg(Color::Red),
                )];
                repr = [priority_repr, repr].concat()
            }

            let line = Line::from(repr);

            items.push(ListItem::from(line))
        }

        app.selected_task_index.select(Some(new_index))
    }

    pub fn reload(app: &mut App, items: &mut Vec<ListItem>) {
        app.projects = Json::read();
        Task::load_items(app, items)
    }

    pub fn _get_all(app: &App) -> &Vec<Task> {
        &app.projects[app.selected_project_index.selected().unwrap()].tasks
    }

    pub fn get_current(app: &mut App) -> &Task {
        &app.projects[app.selected_project_index.selected().unwrap()].tasks
            [app.selected_task_index.selected().unwrap()]
    }

    pub fn create(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        if value.is_empty() {
            return;
        }

        let new_task = Task {
            title: value.to_string(),
            status: TASK_STATUS_ZERO.to_string(),
            priority: 0,
        };

        let mut internal_projects = app.projects.clone();
        internal_projects[app.selected_project_index.selected().unwrap()]
            .tasks
            .push(new_task);

        Json::write(internal_projects);
        Task::reload(app, items)
    }

    pub fn rename(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()].tasks
            [app.selected_task_index.selected().unwrap()]
        .title = value.to_string();

        Json::write(internal_projects);
        Task::reload(app, items)
    }

    pub fn change_status(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()].tasks
            [app.selected_task_index.selected().unwrap()]
        .status = value.to_string();

        Json::write(internal_projects);
        Task::reload(app, items)
    }

    pub fn change_priority(app: &mut App, items: &mut Vec<ListItem>, value: u8) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()].tasks
            [app.selected_task_index.selected().unwrap()]
        .priority = value;

        Json::write(internal_projects);
        Task::reload(app, items)
    }

    pub fn delete(app: &mut App, items: &mut Vec<ListItem>) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()]
            .tasks
            .remove(app.selected_task_index.selected().unwrap());

        Json::write(internal_projects);
        Task::reload(app, items)
    }
}
