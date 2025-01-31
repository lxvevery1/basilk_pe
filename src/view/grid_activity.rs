use std::collections::HashMap;

use ratatui::style::Color;

use crate::{
    project::Project,
    task::{
        TASK_STATUS_DONE, TASK_STATUS_HALF, TASK_STATUS_QUARTER, TASK_STATUS_TREE_QUARTER,
        TASK_STATUS_ZERO,
    },
};

pub const COLORS: [Color; 5] = [
    ratatui::style::Color::Rgb(57, 211, 83), // Lightest Green
    ratatui::style::Color::Rgb(38, 166, 65), // Light Green
    ratatui::style::Color::Rgb(0, 109, 50),  // Green
    ratatui::style::Color::Rgb(14, 68, 41),  // Dark Green
    ratatui::style::Color::Rgb(22, 27, 34),  // Darkest Green
];

pub struct GridActivity {
    pub start_offset: u16,
    pub row_spacing: u16,
    pub col_spacing: u16,
    pub block_conf: GridBlockConf,
    pub blocks: Vec<Vec<GridBlock>>,
}

pub struct GridBlock {
    pub color: Color,
}

pub struct GridBlockConf {
    pub width: u16,
    pub height: u16,
    pub view: String,
}

// github grid activity have 6x52 size
impl GridActivity {
    /// Constructor for GridGrid
    pub fn new(
        start_offset: u16,
        row_spacing: u16,
        col_spacing: u16,
        grid_block_conf: GridBlockConf,
        blocks: Vec<Vec<GridBlock>>,
    ) -> Self {
        Self {
            start_offset,
            row_spacing,
            col_spacing,
            block_conf: grid_block_conf,
            blocks,
        }
    }

    pub fn convert_project_to_activityf32(project: &Project) -> f32 {
        let mut statuses = vec![];

        for task in project.tasks.iter() {
            let status: f32 = task.status.parse().unwrap();
            statuses.push(status);
        }

        // calc arithmetic mean
        let activity: f32 = statuses.iter().sum::<f32>() / statuses.len() as f32;
        activity
    }

    pub fn convert_activityi32_to_color(activity: &i32) -> Color {
        // make a map where key is status and activity value
        // Parse the status constants into f32
        let zero: i32 = TASK_STATUS_ZERO.parse().unwrap();
        let quarter: i32 = TASK_STATUS_QUARTER.parse().unwrap();
        let half: i32 = TASK_STATUS_HALF.parse().unwrap();
        let tree_quarter: i32 = TASK_STATUS_TREE_QUARTER.parse().unwrap();
        let done: i32 = TASK_STATUS_DONE.parse().unwrap();

        // Create the HashMap with numeric ranges
        let mut range_color_map = HashMap::new();
        range_color_map.insert(zero..=quarter, COLORS[4]);
        range_color_map.insert(quarter + 1..=half, COLORS[3]);
        range_color_map.insert(half + 1..=tree_quarter, COLORS[2]);
        range_color_map.insert(tree_quarter + 1..=done, COLORS[1]);

        Self::get_color_for_activity(&range_color_map, *activity)
    }

    fn get_color_for_activity(
        range_color_map: &HashMap<std::ops::RangeInclusive<i32>, Color>,
        activity: i32,
    ) -> Color {
        for (range, color) in range_color_map {
            if range.contains(&activity) {
                return *color;
            }
        }
        COLORS[4] // Default color if no range matches
    }
}

impl GridBlock {
    /// Constructor for GridBlock
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl GridBlockConf {
    /// Constructor for GridBlock
    pub fn new(width: u16, height: u16, view: String) -> Self {
        Self {
            width,
            height,
            view,
        }
    }
}
