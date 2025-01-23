use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::task::{
    TASK_STATUS_DONE, TASK_STATUS_HALF, TASK_STATUS_QUARTER, TASK_STATUS_TREE_QUARTER,
    TASK_STATUS_ZERO,
};

pub const COLORS: [Color; 5] = [
    ratatui::style::Color::Rgb(57, 211, 83), // Lightest Green
    ratatui::style::Color::Rgb(38, 166, 65), // Light Green
    ratatui::style::Color::Rgb(0, 109, 50),  // Green
    ratatui::style::Color::Rgb(14, 68, 41),  // Dark Green
    ratatui::style::Color::Rgb(22, 27, 34),  // Darkest Green
];

pub struct GridGrid {
    pub total_row: u16,
    pub total_col: u16,
    pub start_offset: u16,
    pub row_spacing: u16,
    pub col_spacing: u16,
    pub grid_block_conf: GridBlock,
    pub blocks: Vec<Vec<GridBlock>>,
}

pub struct GridBlock {
    pub view: String,
    pub width: u16,
    pub height: u16,
    pub color: Color,
}

pub struct Grid {}

impl Grid {
    pub fn convert_task_status_to_activity(task_status: &str) -> Color {
        match task_status {
            TASK_STATUS_ZERO => {
                return COLORS[4];
            }
            TASK_STATUS_QUARTER => {
                return COLORS[3];
            }
            TASK_STATUS_HALF => {
                return COLORS[2];
            }
            TASK_STATUS_TREE_QUARTER => {
                return COLORS[1];
            }
            TASK_STATUS_DONE => {
                return COLORS[0];
            }
            _ => {
                return COLORS[4];
            }
        }
    }
}
