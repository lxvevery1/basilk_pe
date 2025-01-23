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
    pub fn render(frame: &mut Frame, grid: GridGrid, area: Rect) {
        let total_width = (grid.grid_block_conf.width + grid.col_spacing) * grid.total_col;

        // Define the layout for the grid
        let grid_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        for chunk in grid_layout.iter() {
            // Calculate the starting x position to center the blocks horizontally
            let start_x = (chunk.width - total_width) / 2;
            for row in 0..grid.total_row {
                // Calculate the y position for each row
                let start_y =
                    grid.start_offset + chunk.y + row * (grid.col_spacing + grid.row_spacing);

                for col in 0..grid.total_col {
                    let blocks_conf = &grid.blocks[row as usize][col as usize];

                    let block = Paragraph::new(
                        grid.grid_block_conf
                            .view
                            .repeat(grid.grid_block_conf.width as usize),
                    )
                    .style(Style::default().fg(blocks_conf.color))
                    .wrap(Wrap { trim: false });

                    // Calculate the position for each block
                    let block_area = Rect {
                        x: start_x + col * (grid.grid_block_conf.width + grid.col_spacing),
                        y: start_y,
                        width: grid.grid_block_conf.width,
                        height: grid.grid_block_conf.height,
                    };

                    frame.render_widget(block, area);
                }
            }
        }
    }

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
