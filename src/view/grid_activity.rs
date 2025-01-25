use ratatui::style::Color;

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
    pub block_conf: GridBlockConf,
    pub blocks: Vec<Vec<GridBlock>>,
}

pub struct GridBlockConf {
    pub width: u16,
    pub height: u16,
    pub view: String,
}

pub struct GridBlock {
    pub color: Color,
}

impl GridGrid {
    /// Constructor for GridGrid
    pub fn new(
        total_row: u16,
        total_col: u16,
        start_offset: u16,
        row_spacing: u16,
        col_spacing: u16,
        grid_block_conf: GridBlockConf,
        blocks: Vec<Vec<GridBlock>>,
    ) -> Self {
        Self {
            total_row,
            total_col,
            start_offset,
            row_spacing,
            col_spacing,
            block_conf: grid_block_conf,
            blocks,
        }
    }
    pub fn convert_task_status_to_activity(task_status: &str) -> Color {
        match task_status {
            TASK_STATUS_ZERO => COLORS[4],
            TASK_STATUS_QUARTER => COLORS[3],
            TASK_STATUS_HALF => COLORS[2],
            TASK_STATUS_TREE_QUARTER => COLORS[1],
            TASK_STATUS_DONE => COLORS[0],
            _ => COLORS[4],
        }
    }
}

impl GridBlock {
    /// Constructor for GridBlock
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn color(&self) -> Color {
        self.color
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
