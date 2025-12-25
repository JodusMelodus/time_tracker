use std::{
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
    time::Duration,
};

use eframe::{NativeOptions, egui};
use egui::{
    Align, Align2, CentralPanel, Color32, Context, CursorIcon, IconData, Layout, MenuBar, Order,
    ScrollArea, Slider, TopBottomPanel, ViewportBuilder, ViewportCommand, Window,
    panel::TopBottomSide,
};

use crate::{agent, config, ui, utils};

fn load_icon(path: &str) -> IconData {
    let (rgba, width, height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        (image.into_raw(), width, height)
    };
    IconData {
        rgba,
        width,
        height,
    }
}

pub fn run_ui(
    command_tx: Sender<agent::AgentCommand>,
    event_rx: Receiver<ui::UIEvent>,
    settings: Arc<config::settings::Settings>,
) {
    let icon = load_icon("icon.ico");
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
                agent_tx: command_tx,
                ui_rx: event_rx,
                new_task: agent::tasks::Task::default(),
                task_state: false,
                session_comment: "".to_string(),
                elapsed_time: Duration::ZERO,
                settings,
                tasks: Vec::new(),
                show_new_task_dialog: false,
                last_user_activity_time_stamp: chrono::Utc::now(),
                user_state: agent::UserState::Active,
            }))
        }),
    )
    .unwrap();
}

struct MyApp {
    agent_tx: Sender<agent::AgentCommand>,
    ui_rx: Receiver<ui::UIEvent>,
    new_task: agent::tasks::Task,
    task_state: bool,
    session_comment: String,
    elapsed_time: Duration,
    settings: Arc<config::settings::Settings>,

    tasks: Vec<agent::tasks::Task>,
    show_new_task_dialog: bool,
    user_state: agent::UserState,
    last_user_activity_time_stamp: chrono::DateTime<chrono::Utc>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.ui_rx.try_recv() {
            match event {
                ui::UIEvent::TaskList { task_list } => self.tasks = task_list,
                ui::UIEvent::UserActivity { time_stamp } => {
                    self.user_state = agent::UserState::Active;
                    self.last_user_activity_time_stamp = time_stamp;
                    self.agent_tx
                        .send(agent::AgentCommand::UpdateStopWatch { running: true })
                        .unwrap();
                }
                ui::UIEvent::ProgressState { state } => self.task_state = state,
                ui::UIEvent::ElapsedTime { elapsed } => self.elapsed_time = elapsed,
            }

            ctx.request_repaint();
        }

        self.agent_tx
            .send(agent::AgentCommand::ElapsedTime)
            .unwrap();

        let idle_after = self.last_user_activity_time_stamp
            + chrono::Duration::seconds(self.settings.active_timeout_seconds.try_into().unwrap());
        let now = chrono::Utc::now();

        if self.user_state == agent::UserState::Active {
            if now >= idle_after {
                self.user_state = agent::UserState::Idle;
                self.agent_tx
                    .send(agent::AgentCommand::UpdateStopWatch { running: false })
                    .unwrap();
                ctx.request_repaint();
            } else {
                let remaining = idle_after - now;
                ctx.request_repaint_after(std::time::Duration::from_secs(
                    remaining.num_seconds() as u64
                ));
            }
        }

        // Menu bar
        TopBottomPanel::new(TopBottomSide::Top, "Menu Bar").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui
                        .button("Quit")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.agent_tx.send(agent::AgentCommand::Quit).unwrap();
                        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                    }
                })
                .response
                .on_hover_cursor(CursorIcon::PointingHand);

                ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    if ui
                        .button("Add Task")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.show_new_task_dialog = true;
                    }
                });
            });
        });

        // Status bar
        TopBottomPanel::new(TopBottomSide::Bottom, "Status Bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    match self.user_state {
                        agent::UserState::Active => ui.colored_label(Color32::DARK_GREEN, "Active"),
                        agent::UserState::Idle => ui.colored_label(Color32::DARK_GRAY, "Idle"),
                    };
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                    if ui
                        .button("↻")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.agent_tx
                            .send(agent::AgentCommand::RequestTaskList)
                            .unwrap();
                    }

                    ui.label(utils::time::format_duration(self.elapsed_time));
                });
            });
        });

        // Main window
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
                                        match task.t_priority {
                                            0 => ui.colored_label(
                                                Color32::DARK_GREEN,
                                                agent::tasks::PRIORITY_LEVELS[task.t_priority],
                                            ),
                                            1 => ui.colored_label(
                                                Color32::YELLOW,
                                                agent::tasks::PRIORITY_LEVELS[task.t_priority],
                                            ),
                                            2 => ui.colored_label(
                                                Color32::RED,
                                                agent::tasks::PRIORITY_LEVELS[task.t_priority],
                                            ),
                                            _ => ui.label(
                                                agent::tasks::PRIORITY_LEVELS[task.t_priority],
                                            ),
                                        };
                                    });

                                    ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                                        self.agent_tx
                                            .send(agent::AgentCommand::RequestTaskState)
                                            .unwrap();
                                        match self.task_state {
                                            true => {
                                                if ui
                                                    .button("Stop")
                                                    .on_hover_cursor(CursorIcon::PointingHand)
                                                    .clicked()
                                                {
                                                    self.agent_tx
                                                        .send(agent::AgentCommand::EndSession {
                                                            comment: self.session_comment.clone(),
                                                        })
                                                        .unwrap();
                                                }
                                            }
                                            false => {
                                                if ui
                                                    .button("Start")
                                                    .on_hover_cursor(CursorIcon::PointingHand)
                                                    .clicked()
                                                {
                                                    self.agent_tx
                                                        .send(agent::AgentCommand::StartSession {
                                                            id: task.t_id,
                                                        })
                                                        .unwrap();
                                                }
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

        if self.show_new_task_dialog {
            self.new_task_dialog(ctx);
        } else {
            self.new_task = agent::tasks::Task::default();
        }
    }
}

impl MyApp {
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
                            self.agent_tx
                                .send(agent::AgentCommand::AddTask {
                                    task: self.new_task.clone(),
                                })
                                .unwrap();
                            self.show_new_task_dialog = false;
                            self.agent_tx
                                .send(agent::AgentCommand::RequestTaskList)
                                .unwrap();
                        }
                    });
                });
            });
    }
}
