use crate::{project::Project, task::Task, ui::Ui, util::Util, App, ViewMode};
use grid_activity::{GridBlock, GridBlockConf};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Clear, HighlightSpacing, List, ListItem, Paragraph, Wrap},
    Frame,
};
use tui_input::Input;

pub mod grid_activity;

pub struct View {}

impl View {
    pub fn show_new_item_modal(f: &mut Frame, area: Rect, input: &Input) {
        Ui::create_input_modal("New", f, area, input)
    }

    pub fn show_migration_info_modal(f: &mut Frame, area: Rect) {
        let widget = Paragraph::new(Text::from(vec![
            Line::raw("New migrations were applied!"),
            Line::raw("Check the changelog"),
        ]))
        .alignment(Alignment::Center)
        .block(Block::bordered());

        Ui::create_modal(f, 30, 4, area, widget)
    }

    pub fn show_rename_item_modal(f: &mut Frame, area: Rect, input: &Input) {
        Ui::create_input_modal("Rename", f, area, input)
    }

    pub fn show_delete_item_modal(app: &mut App, f: &mut Frame, area: Rect) {
        let title = match app.view_mode {
            ViewMode::DeleteTask => &Task::get_current(app).title,
            ViewMode::DeleteProject => &Project::get_current(app).title,
            _ => "",
        };

        Ui::create_question_modal(
            "Are you sure to delete?",
            format!("\"{}\"", title).as_str(),
            "Delete",
            f,
            area,
        )
    }

    pub fn show_select_task_status_modal(
        app: &mut App,
        status_items: &[ListItem],
        f: &mut Frame,
        area: Rect,
    ) {
        let area = Ui::create_rect_area(10, 7, area);

        let task_status_list_widget = List::new(status_items.to_owned())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always)
            .block(Block::bordered().title("Status"));

        f.render_widget(Clear, area);
        f.render_stateful_widget(task_status_list_widget, area, app.use_state())
    }

    pub fn show_select_task_priority_modal(
        app: &mut App,
        priority_items: &[ListItem],
        f: &mut Frame,
        area: Rect,
    ) {
        let area = Ui::create_rect_area(10, 6, area);

        let task_status_list_widget = List::new(priority_items.to_owned())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always)
            .block(Block::bordered().title("Priority"));

        f.render_widget(Clear, area);
        f.render_stateful_widget(task_status_list_widget, area, app.use_state())
    }

    pub fn show_items(app: &mut App, items: &[ListItem], f: &mut Frame, area: Rect) {
        let block: Block = match app.view_mode {
            ViewMode::ViewProjects
            | ViewMode::AddProject
            | ViewMode::RenameProject
            | ViewMode::DeleteProject => Block::bordered(),
            _ => Block::bordered().title(Util::get_spaced_title(&Project::get_current(app).title)),
        };

        // Iterate through all elements in the `items` and stylize them.
        let items = items.to_owned();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        if app.view_mode == ViewMode::ChangeStatusTask
            || app.view_mode == ViewMode::ChangePriorityTask
        {
            f.render_widget(items, area)
        } else {
            f.render_stateful_widget(items, area, app.use_state());
        }
    }

    pub fn show_graph_activity(app: &mut App, f: &mut Frame, area: Rect) {
        let colors: Vec<Color> = app
            .projects
            .iter()
            .map(|project| {
                let activity_f32 =
                    grid_activity::GridActivity::convert_project_to_activityf32(project);

                let activity_i32 = activity_f32 as i32; // Cast `f32` to `i32`
                grid_activity::GridActivity::convert_activityi32_to_color(&activity_i32)
            })
            .collect();

        // Define the grid block configuration
        let grid_block_conf = GridBlockConf::new(
            2,
            2,
            char::from_u32(0x00002588) // ASCII block
                .map(|c| c.to_string())
                .unwrap_or_else(|| "?".to_string()),
        );

        let rows_n = 6;
        let cols_n = (colors.len() as f32 / rows_n as f32).ceil() as usize;

        let mut blocks: Vec<Vec<GridBlock>> = Vec::with_capacity(rows_n);

        for row in 0..rows_n {
            let mut row_blocks = Vec::with_capacity(cols_n);
            for col in 0..cols_n {
                let index = col * rows_n + row;
                if index < colors.len() {
                    row_blocks.push(GridBlock::new(colors[index]));
                } else {
                    // Fill with a default color or leave empty
                    row_blocks.push(GridBlock::new(Color::default()));
                }
            }
            blocks.push(row_blocks);
        }

        let grid = grid_activity::GridActivity::new(1, 0, 1, grid_block_conf, blocks);

        let total_rows = grid.blocks.len() as u16;
        let total_cols = grid.blocks[0].len() as u16;

        if total_cols == 0 || total_rows == 0 {
            eprintln!("total_rows or total_cols == 0!");
            return;
        }

        // Calculate the total spacing
        let total_col_spacing = (total_cols - 1) * grid.col_spacing;
        let total_row_spacing = grid.row_spacing;

        if total_col_spacing >= area.width || total_row_spacing >= area.height {
            eprintln!(
                "Not enough space to render the grid. Adjust spacing or increase area size {} {}.",
                total_cols, total_rows
            );
            return;
        }

        // Calculate the size of each block
        let block_width = grid.block_conf.width;
        let block_height = grid.block_conf.height;

        if block_width == 0 || block_height == 0 {
            eprintln!("Block size is too small. Adjust spacing or increase area size.");
            return;
        }

        // Render each block in the grid
        for (row_index, row) in grid.blocks.iter().enumerate() {
            for (col_index, block) in row.iter().enumerate() {
                // Calculate the position of the block
                let block_area = Rect {
                    x: grid.start_offset
                        + area.x
                        + col_index as u16 * (block_width + grid.col_spacing),
                    y: area.y + row_index as u16 * (block_height + grid.row_spacing),
                    width: block_width,
                    height: block_height,
                };

                // Create a paragraph for the block
                let paragraph = Paragraph::new(
                    grid.block_conf
                        .view
                        .repeat(grid.block_conf.width as usize)
                        .clone(),
                )
                .style(block.color)
                .wrap(Wrap { trim: false })
                .centered();

                f.render_widget(paragraph, block_area);
            }
        }
    }

    pub fn show_footer_helper(app: &mut App, f: &mut Frame, area: Rect) {
        let help_string = match app.view_mode {
            ViewMode::ViewProjects => {
                "<k/j> next/prev :: <l> go to tasks :: <a/n> new :: <r> rename :: <d> delete :: <q> quit"
            }
            ViewMode::RenameProject => "<Enter> confirm :: <Esc> cancel",
            ViewMode::AddProject => "<Enter> confirm :: <Esc> cancel",
            ViewMode::DeleteProject => "<y> confirm :: <n> cancel",

            ViewMode::ViewTasks => {
                "<k/j> next/prev :: <h> go to projects :: <Enter> change status :: <p> change priority :: <a/n> new :: <r> rename :: <d> delete :: <q> quit"
            }
            ViewMode::RenameTask => "<Enter> confirm :: <Esc> cancel",
            ViewMode::ChangeStatusTask => "<k/j> next/prev :: <Enter> confirm :: <Esc> cancel",
            ViewMode::ChangePriorityTask => "<k/j> next/prev :: <Enter> confirm :: <Esc> cancel",
            ViewMode::AddTask => "<Enter> confirm :: <Esc> cancel",
            ViewMode::DeleteTask => "<y> confirm :: <n> cancel",
            ViewMode::InfoMigration => ""
        };

        f.render_widget(
            Paragraph::new(help_string)
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Center),
            area,
        );
    }
}
