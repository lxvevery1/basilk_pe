use crate::{project::Project, task::Task, ui::Ui, util::Util, App, ViewMode};
use ratatui::{
    layout::{Alignment, Rect},
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
        let total_width = (grid::grid_block_conf.width + grid.col_spacing) * grid.total_col;

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
