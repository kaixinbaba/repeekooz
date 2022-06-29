use crate::config::Config;
use crate::events::{self, warn, Message};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use anyhow::Result;
use std::fs::File;
use std::path::Path;
use std::{
    io::{self, Stdout},
    sync::{Arc, Mutex},
};

use tui::{backend::CrosstermBackend, Terminal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchMode {
    /// 普通文本搜索
    Normal,

    /// 搜期数
    Volume,

    /// 搜类别
    Category,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    /// 搜索模式
    Search,

    /// 浏览模式
    View,

    /// 弹窗提示
    Popup,

    /// 项目明细
    Detail,
}

pub struct App {
    /// 终端
    pub terminal: Terminal<CrosstermBackend<Stdout>>,

    /// 是否要显示帮助
    pub show_help: bool,
}

impl App {
    fn new(config: &Config) -> Result<App> {
        let show_help = Self::init_config(config.config_path.clone())? || config.show_help;

        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        // init Global static

        Ok(App {
            terminal,
            show_help,
        })
    }

    /// 初始化配置文件
    fn init_config(config_path: String) -> Result<bool> {
        let path = Path::new(&config_path).join(".hgtui.toml");
        if path.exists() {
            return Ok(false);
        }
        if File::create(&path).is_ok() {
            return Ok(true);
        }
        Ok(false)
    }
}

impl App {}

impl Drop for App {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

pub(crate) fn start(config: &Config) -> Result<()> {
    if config.show_themes {
        return Ok(());
    }

    let app = Arc::new(Mutex::new(App::new(config)?));

    let moved_app = app.clone();
    events::handle_key_event(moved_app);

    events::handle_notify(app);

    Ok(())
}
