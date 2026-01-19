use iced::widget::{column, container, row, scrollable, text, button, progress_bar, Space};
use iced::{Application, Command, Element, Length, Settings, Theme, Subscription};
use sysinfo::{System, Pid, ProcessStatus};
use std::time::Duration;

fn main() -> iced::Result {
    MonitorApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1000.0, 700.0),
            min_size: Some(iced::Size::new(800.0, 500.0)),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    SelectTab(Tab),
    SelectProcess(u32),
    KillProcess(u32),
    SortProcesses(SortBy),
    ToggleSortOrder,
    RefreshProcesses,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Overview,
    Processes,
    Resources,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortBy {
    #[default]
    Cpu,
    Memory,
    Name,
    Pid,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
    pub status: String,
}

pub struct MonitorApp {
    system: System,
    current_tab: Tab,
    selected_process: Option<u32>,
    processes: Vec<ProcessInfo>,
    sort_by: SortBy,
    sort_ascending: bool,
    cpu_history: Vec<f32>,
    memory_history: Vec<f32>,
}

impl Application for MonitorApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut system = System::new_all();
        system.refresh_all();

        let processes = collect_processes(&system);

        (
            Self {
                system,
                current_tab: Tab::default(),
                selected_process: None,
                processes,
                sort_by: SortBy::Cpu,
                sort_ascending: false,
                cpu_history: vec![0.0; 60],
                memory_history: vec![0.0; 60],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "RururuOS System Monitor".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                self.system.refresh_all();
                self.processes = collect_processes(&self.system);
                self.sort_processes();

                // Update history
                let cpu = self.system.global_cpu_usage();
                let mem = self.system.used_memory() as f32 / self.system.total_memory() as f32 * 100.0;

                self.cpu_history.push(cpu);
                self.memory_history.push(mem);

                if self.cpu_history.len() > 60 {
                    self.cpu_history.remove(0);
                }
                if self.memory_history.len() > 60 {
                    self.memory_history.remove(0);
                }
            }
            Message::SelectTab(tab) => {
                self.current_tab = tab;
            }
            Message::SelectProcess(pid) => {
                self.selected_process = Some(pid);
            }
            Message::KillProcess(pid) => {
                if let Some(process) = self.system.process(Pid::from_u32(pid)) {
                    process.kill();
                }
                self.system.refresh_all();
                self.processes = collect_processes(&self.system);
            }
            Message::SortProcesses(sort_by) => {
                if self.sort_by == sort_by {
                    self.sort_ascending = !self.sort_ascending;
                } else {
                    self.sort_by = sort_by;
                    self.sort_ascending = false;
                }
                self.sort_processes();
            }
            Message::ToggleSortOrder => {
                self.sort_ascending = !self.sort_ascending;
                self.sort_processes();
            }
            Message::RefreshProcesses => {
                self.system.refresh_all();
                self.processes = collect_processes(&self.system);
                self.sort_processes();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let tabs = row![
            tab_button("Overview", Tab::Overview, self.current_tab),
            tab_button("Processes", Tab::Processes, self.current_tab),
            tab_button("Resources", Tab::Resources, self.current_tab),
        ]
        .spacing(4);

        let content: Element<Message> = match self.current_tab {
            Tab::Overview => self.view_overview(),
            Tab::Processes => self.view_processes(),
            Tab::Resources => self.view_resources(),
        };

        container(
            column![
                tabs,
                Space::with_height(Length::Fixed(16.0)),
                content,
            ]
            .padding(16),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl MonitorApp {
    fn sort_processes(&mut self) {
        match self.sort_by {
            SortBy::Cpu => {
                self.processes.sort_by(|a, b| {
                    if self.sort_ascending {
                        a.cpu.partial_cmp(&b.cpu).unwrap()
                    } else {
                        b.cpu.partial_cmp(&a.cpu).unwrap()
                    }
                });
            }
            SortBy::Memory => {
                self.processes.sort_by(|a, b| {
                    if self.sort_ascending {
                        a.memory.cmp(&b.memory)
                    } else {
                        b.memory.cmp(&a.memory)
                    }
                });
            }
            SortBy::Name => {
                self.processes.sort_by(|a, b| {
                    if self.sort_ascending {
                        a.name.to_lowercase().cmp(&b.name.to_lowercase())
                    } else {
                        b.name.to_lowercase().cmp(&a.name.to_lowercase())
                    }
                });
            }
            SortBy::Pid => {
                self.processes.sort_by(|a, b| {
                    if self.sort_ascending {
                        a.pid.cmp(&b.pid)
                    } else {
                        b.pid.cmp(&a.pid)
                    }
                });
            }
        }
    }

    fn view_overview(&self) -> Element<Message> {
        let cpu_usage = self.system.global_cpu_usage();
        let mem_used = self.system.used_memory();
        let mem_total = self.system.total_memory();
        let mem_percent = mem_used as f32 / mem_total as f32 * 100.0;

        let swap_used = self.system.used_swap();
        let swap_total = self.system.total_swap();
        let swap_percent = if swap_total > 0 {
            swap_used as f32 / swap_total as f32 * 100.0
        } else {
            0.0
        };

        let process_count = self.processes.len();

        column![
            // CPU
            text("CPU").size(18),
            row![
                text(format!("{:.1}%", cpu_usage)),
                Space::with_width(Length::Fixed(16.0)),
                progress_bar(0.0..=100.0, cpu_usage).height(Length::Fixed(20.0)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            Space::with_height(Length::Fixed(16.0)),

            // Memory
            text("Memory").size(18),
            row![
                text(format!("{:.1}%", mem_percent)),
                Space::with_width(Length::Fixed(16.0)),
                progress_bar(0.0..=100.0, mem_percent).height(Length::Fixed(20.0)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),
            text(format!(
                "{:.1} GB / {:.1} GB",
                mem_used as f64 / 1024.0 / 1024.0 / 1024.0,
                mem_total as f64 / 1024.0 / 1024.0 / 1024.0
            ))
            .size(12),

            Space::with_height(Length::Fixed(16.0)),

            // Swap
            text("Swap").size(18),
            row![
                text(format!("{:.1}%", swap_percent)),
                Space::with_width(Length::Fixed(16.0)),
                progress_bar(0.0..=100.0, swap_percent).height(Length::Fixed(20.0)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8),

            Space::with_height(Length::Fixed(16.0)),

            // Stats
            text("System").size(18),
            row![
                text("Processes:"),
                Space::with_width(Length::Fixed(8.0)),
                text(format!("{}", process_count)),
            ]
            .padding(8),

            row![
                text("Uptime:"),
                Space::with_width(Length::Fixed(8.0)),
                text(format_uptime(System::uptime())),
            ]
            .padding(8),
        ]
        .spacing(4)
        .into()
    }

    fn view_processes(&self) -> Element<Message> {
        let header = row![
            button(text("PID").size(12))
                .style(iced::theme::Button::Text)
                .on_press(Message::SortProcesses(SortBy::Pid))
                .width(Length::Fixed(70.0)),
            button(text("Name").size(12))
                .style(iced::theme::Button::Text)
                .on_press(Message::SortProcesses(SortBy::Name))
                .width(Length::FillPortion(3)),
            button(text("CPU %").size(12))
                .style(iced::theme::Button::Text)
                .on_press(Message::SortProcesses(SortBy::Cpu))
                .width(Length::Fixed(80.0)),
            button(text("Memory").size(12))
                .style(iced::theme::Button::Text)
                .on_press(Message::SortProcesses(SortBy::Memory))
                .width(Length::Fixed(100.0)),
            text("Status").size(12).width(Length::Fixed(80.0)),
        ]
        .spacing(8)
        .padding(8);

        let processes: Vec<Element<Message>> = self
            .processes
            .iter()
            .take(100)
            .map(|p| {
                let is_selected = self.selected_process == Some(p.pid);
                let mem_mb = p.memory as f64 / 1024.0 / 1024.0;

                let row_content = row![
                    text(format!("{}", p.pid)).size(12).width(Length::Fixed(70.0)),
                    text(&p.name).size(12).width(Length::FillPortion(3)),
                    text(format!("{:.1}", p.cpu)).size(12).width(Length::Fixed(80.0)),
                    text(format!("{:.1} MB", mem_mb)).size(12).width(Length::Fixed(100.0)),
                    text(&p.status).size(12).width(Length::Fixed(80.0)),
                ]
                .spacing(8)
                .padding(4);

                let style = if is_selected {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Text
                };

                button(row_content)
                    .style(style)
                    .width(Length::Fill)
                    .on_press(Message::SelectProcess(p.pid))
                    .into()
            })
            .collect();

        let actions = if let Some(pid) = self.selected_process {
            row![
                button(text("Kill Process"))
                    .style(iced::theme::Button::Destructive)
                    .on_press(Message::KillProcess(pid)),
                Space::with_width(Length::Fixed(8.0)),
                button(text("Refresh"))
                    .style(iced::theme::Button::Secondary)
                    .on_press(Message::RefreshProcesses),
            ]
        } else {
            row![
                button(text("Refresh"))
                    .style(iced::theme::Button::Secondary)
                    .on_press(Message::RefreshProcesses),
            ]
        };

        column![
            actions,
            Space::with_height(Length::Fixed(8.0)),
            header,
            scrollable(column(processes).spacing(2)).height(Length::Fill),
        ]
        .spacing(4)
        .into()
    }

    fn view_resources(&self) -> Element<Message> {
        // CPU cores
        let cpus = self.system.cpus();
        let cpu_items: Vec<Element<Message>> = cpus
            .iter()
            .enumerate()
            .map(|(i, cpu)| {
                row![
                    text(format!("CPU {}", i)).size(12).width(Length::Fixed(60.0)),
                    progress_bar(0.0..=100.0, cpu.cpu_usage())
                        .height(Length::Fixed(12.0))
                        .width(Length::Fill),
                    text(format!("{:.0}%", cpu.cpu_usage()))
                        .size(12)
                        .width(Length::Fixed(50.0)),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center)
                .into()
            })
            .collect();

        column![
            text("CPU Cores").size(18),
            Space::with_height(Length::Fixed(8.0)),
            column(cpu_items).spacing(4),

            Space::with_height(Length::Fixed(24.0)),

            text("Disks").size(18),
            Space::with_height(Length::Fixed(8.0)),
            self.view_disks(),
        ]
        .spacing(4)
        .into()
    }

    fn view_disks(&self) -> Element<Message> {
        let disks = sysinfo::Disks::new_with_refreshed_list();
        let disk_items: Vec<Element<Message>> = disks
            .iter()
            .map(|disk| {
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total - available;
                let percent = used as f32 / total as f32 * 100.0;

                column![
                    row![
                        text(disk.name().to_string_lossy()).size(14),
                        Space::with_width(Length::Fill),
                        text(disk.mount_point().to_string_lossy()).size(12),
                    ],
                    progress_bar(0.0..=100.0, percent).height(Length::Fixed(8.0)),
                    text(format!(
                        "{:.1} GB / {:.1} GB ({:.0}%)",
                        used as f64 / 1024.0 / 1024.0 / 1024.0,
                        total as f64 / 1024.0 / 1024.0 / 1024.0,
                        percent
                    ))
                    .size(11),
                ]
                .spacing(4)
                .padding(8)
                .into()
            })
            .collect();

        column(disk_items).spacing(8).into()
    }
}

fn tab_button(label: &str, tab: Tab, current: Tab) -> Element<Message> {
    let style = if tab == current {
        iced::theme::Button::Primary
    } else {
        iced::theme::Button::Secondary
    };

    button(text(label))
        .style(style)
        .padding(8)
        .on_press(Message::SelectTab(tab))
        .into()
}

fn collect_processes(system: &System) -> Vec<ProcessInfo> {
    system
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu: process.cpu_usage(),
            memory: process.memory(),
            status: format!("{:?}", process.status()),
        })
        .collect()
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}
