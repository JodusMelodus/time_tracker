use std::{
    sync::{Arc, mpsc},
    time::Duration,
};

use eframe::{NativeOptions, egui};
use egui::{
    Align, Align2, CentralPanel, Color32, Context, CursorIcon, Layout, MenuBar, Order, ScrollArea,
    Slider, TopBottomPanel, ViewportBuilder, ViewportCommand, Window, panel::TopBottomSide,
};

use crate::{APP_ICON_BYTES, agent, config, ui};

pub fn run_ui(
    command_tx: mpsc::Sender<agent::AgentCommand>,
    window_rx: crossbeam_channel::Receiver<ui::viewmodels::UIEvent>,
    settings: Arc<config::Settings>,
) {
    let icon = ui::utils::load_icon_from_bytes(APP_ICON_BYTES);
    let mut options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_min_inner_size([250.0, 500.0])
            .with_max_inner_size([500.0, 750.0]),
        ..Default::default()
    };

    options.viewport.icon = Some(Arc::new(icon));
    eframe::run_native(
        "Time Tracker",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                command_tx,
                window_rx,
                new_task: agent::Task::default(),
                session_comment: "".to_string(),
                elapsed_time: Duration::ZERO,
                _settings: settings,
                active_task_id: -1,
                dialog_info: ui::DialogInfo::default(),
                tasks: Vec::new(),
                show_new_task_dialog: false,
                user_state: ui::viewmodels::UserState::Active,
            }))
        }),
    )
    .unwrap();
}

struct MyApp {
    command_tx: mpsc::Sender<agent::AgentCommand>,
    window_rx: crossbeam_channel::Receiver<ui::viewmodels::UIEvent>,
    new_task: agent::tasks::Task,
    session_comment: String,
    elapsed_time: Duration,
    _settings: Arc<config::Settings>,
    active_task_id: i64,
    dialog_info: ui::DialogInfo,

    tasks: Vec<agent::tasks::Task>,
    show_new_task_dialog: bool,
    user_state: ui::viewmodels::UserState,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.window_rx.try_recv() {
            match event {
                ui::viewmodels::UIEvent::TaskList { task_list } => self.tasks = task_list,
                ui::viewmodels::UIEvent::ElapsedTime { elapsed } => self.elapsed_time = elapsed,
                ui::viewmodels::UIEvent::Quit => ctx.send_viewport_cmd(ViewportCommand::Close),
                ui::UIEvent::Repaint { time_out } => {
                    ctx.request_repaint_after(Duration::from_secs(time_out));
                }
                ui::UIEvent::UserState { state } => self.user_state = state,
            }

            ctx.request_repaint();
        }

        if self.tasks.is_empty() {
            if let Err(e) = self.command_tx.send(agent::AgentCommand::RequestTaskList) {
                self.dialog_info = ui::DialogInfo {
                    title: "Error",
                    message: format!("{}", e),
                    shown: false,
                }
            }
        }

        if self.active_task_id != -1 {
            if let Err(e) = self.command_tx.send(agent::AgentCommand::RequestElapsedTime) {
                self.dialog_info = ui::DialogInfo {
                    title: "Error",
                    message: format!("{}", e),
                    shown: false,
                }
            }
        }

        // Menu bar
        self.menu_bar(ctx);

        // Status bar
        self.status_bar(ctx);

        // Main window
        self.main_window(ctx);

        if self.show_new_task_dialog {
            self.new_task_dialog(ctx);
        } else {
            self.new_task = agent::tasks::Task::default();
        }

        if !self.dialog_info.shown {
            self.show_dialog(ctx);
        }
    }
}

impl MyApp {
    fn show_dialog(&mut self, ctx: &Context) {
        Window::new(self.dialog_info.title)
            .collapsible(false)
            .fixed_size([300.0, 50.0])
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .order(Order::Foreground)
            .show(ctx, |ui| {
                ui.label(format!("{}", self.dialog_info.message));

                ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    if ui
                        .button("OK")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.dialog_info.shown = true;
                    }
                });
            });
    }

    fn new_task_dialog(&mut self, ctx: &Context) {
        Window::new("New Task")
            .collapsible(false)
            .fixed_size([400.0, 100.0])
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .order(Order::Foreground)
            .show(ctx, |ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.new_task.t_name);

                ui.horizontal(|ui| {
                    ui.label("Priority");
                    let level = self.new_task.t_priority;
                    ui.add(
                        Slider::new(&mut self.new_task.t_priority, 0..=2)
                            .step_by(0.33)
                            .text(agent::tasks::PRIORITY_LEVELS[level])
                            .show_value(false),
                    );
                });

                ui.separator();
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                        if ui
                            .button("Cancel")
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            self.show_new_task_dialog = false;
                        }

                        if ui
                            .button("Add")
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            if let Err(e) = self.command_tx.send(agent::AgentCommand::AddTask {
                                task: self.new_task.clone(),
                            }) {
                                self.dialog_info = ui::DialogInfo {
                                    title: "Error",
                                    message: format!("{}", e),
                                    shown: false,
                                }
                            }
                            self.show_new_task_dialog = false;
                            self.dialog_info = ui::DialogInfo {
                                title: "Information",
                                message: format!("{}", "Added new task successfully!"),
                                shown: false,
                            };
                        }
                    });
                });
            });
    }

    fn menu_bar(&mut self, ctx: &Context) {
        TopBottomPanel::new(TopBottomSide::Top, "Menu Bar").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui
                        .button("Exit")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                })
                .response
                .on_hover_cursor(CursorIcon::PointingHand);

                ui.menu_button("Task", |ui| {
                    if ui
                        .button("New Task...")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.show_new_task_dialog = !self.show_new_task_dialog;
                    }
                })
                .response
                .on_hover_cursor(CursorIcon::PointingHand)
            });
        });
    }

    fn main_window(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading("Tasks");
                ScrollArea::vertical().show(ui, |ui| {
                    for task in &mut self.tasks {
                        ui.group(|ui| {
                            ui.take_available_width();
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(&task.t_name).on_hover_text("Name");
                                    ui.text_edit_multiline(&mut self.session_comment)
                                        .on_hover_text("Comment");
                                });

                                ui.vertical(|ui| {
                                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                        ui.colored_label(
                                            match task.t_priority {
                                                0 => Color32::DARK_GREEN,
                                                1 => Color32::YELLOW,
                                                2 => Color32::DARK_RED,
                                                _ => Color32::DARK_BLUE,
                                            },
                                            "⏺",
                                        )
                                        .on_hover_cursor(CursorIcon::Default)
                                        .on_hover_text("Priority")
                                    });

                                    ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                                        if self.active_task_id == task.t_id {
                                            if ui
                                                .button("⏸")
                                                .on_hover_cursor(CursorIcon::PointingHand)
                                                .on_hover_text("Stop")
                                                .clicked()
                                            {
                                                if let Err(e) = self.command_tx.send(
                                                    agent::AgentCommand::EndSession {
                                                        comment: self.session_comment.clone(),
                                                    },
                                                ) {
                                                    self.dialog_info = ui::DialogInfo {
                                                        title: "Error",
                                                        message: format!("{}", e),
                                                        shown: false,
                                                    }
                                                }
                                                self.active_task_id = -1;
                                                self.session_comment = "".into();
                                            }
                                        } else {
                                            if ui
                                                .button("▶")
                                                .on_hover_cursor(CursorIcon::PointingHand)
                                                .on_hover_text("Start")
                                                .clicked()
                                            {
                                                if let Err(e) = self.command_tx.send(
                                                    agent::AgentCommand::StartSession {
                                                        id: task.t_id,
                                                    },
                                                ) {
                                                    self.dialog_info = ui::DialogInfo {
                                                        title: "Error",
                                                        message: format!("{}", e),
                                                        shown: false,
                                                    }
                                                }
                                                self.active_task_id = task.t_id;
                                            }
                                        }
                                    });
                                });
                            });
                        });
                    }
                });
            });
        });
    }

    fn status_bar(&mut self, ctx: &Context) {
        TopBottomPanel::new(TopBottomSide::Bottom, "Status Bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    match self.user_state {
                        ui::viewmodels::UserState::Active => ui
                            .colored_label(Color32::DARK_GREEN, "⏺")
                            .on_hover_cursor(CursorIcon::Default)
                            .on_hover_text("Active"),
                        ui::viewmodels::UserState::Idle => ui
                            .colored_label(Color32::DARK_GRAY, "⏺")
                            .on_hover_cursor(CursorIcon::Default)
                            .on_hover_text("Idle"),
                    };
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                    if ui
                        .button("⟳")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .on_hover_text("Sync")
                        .clicked()
                    {}

                    ui.label(ui::utils::format_duration(self.elapsed_time));
                });
            });
        });
    }
}
