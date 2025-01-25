use crate::{project::Project, task::Task, ui::Ui, util::Util, App, ViewMode};
use grid_activity::{GridBlock, GridBlockConf, COLORS};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Clear, HighlightSpacing, List, ListItem, Paragraph, Wrap},
    Frame,
};
use tui_input::Input;

mod grid_activity;

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

    pub fn show_graph_activity(app: &mut App, items: &[ListItem], f: &mut Frame, area: Rect) {
        // Define the grid block configuration
        let grid_block_conf = GridBlockConf::new(
            2,
            2,
            char::from_u32(0x00002588)
                .map(|c| c.to_string())
                .unwrap_or_else(|| "?".to_string()),
        );
        // Define the blocks
        let blocks = vec![
            vec![
                GridBlock::new(COLORS[0]), // Very Light Green
                GridBlock::new(COLORS[2]), // Light Green
                GridBlock::new(COLORS[1]), // Medium Green
            ],
            vec![
                GridBlock::new(COLORS[3]), // Light Green
                GridBlock::new(COLORS[4]), // Medium Green
                GridBlock::new(COLORS[2]), // Dark Green
            ],
            vec![
                GridBlock::new(COLORS[2]), // Medium Green
                GridBlock::new(COLORS[3]), // Dark Green
                GridBlock::new(COLORS[4]), // Dark Green
            ],
        ];

        let grid = grid_activity::GridGrid::new(3, 3, 1, 1, 1, grid_block_conf, blocks);

        let total_rows = grid.blocks.len() as u16;
        let total_cols = grid.blocks[0].len() as u16;

        // Calculate the size of each block
        let block_width = (area.width - (total_cols - 1) * grid.col_spacing) / total_cols;
        let block_height = (area.height - (total_rows - 1) * grid.row_spacing) / total_rows;

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
                let paragraph =
                    Paragraph::new(grid.block_conf.view.repeat(block_width as usize).clone())
                        .style(block.color)
                        .wrap(Wrap { trim: true })
                        .centered();

                // Render the block
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
