use zellij_tile::prelude::*;

// chronoクレートから時刻を扱うためのものをインポートします
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    // the state of the plugin
    timezone: Tz,
    current_time: DateTime<Utc>,
    tabs: Vec<TabInfo>,
    mode: ModeInfo,
}

register_plugin!(State);

// NOTE: you can start a development environment inside Zellij by running `zellij -l zellij.kdl` in
// this plugin's folder
//
// More info on plugins: https://zellij.dev/documentation/plugins

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.timezone = "Asia/Tokyo".parse().unwrap();

        set_timeout(1.0);
        request_permission(&[
            PermissionType::ReadApplicationState,
        ]);
        subscribe(&[
            EventType::Timer,
            EventType::TabUpdate,
            EventType::ModeUpdate,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        // react to `Event`s that have been subscribed to (and the plugin has permissions for)
        // return true if this plugin's `render` function should be called for the plugin to render
        // itself
        match event {
            // `Timer`イベントを受け取った場合の処理
            Event::Timer(_payload) => {
                // 現在時刻を取得し、"時:分:秒"の形式にフォーマットします
                let time = Utc::now();
                // 状態を更新します
                self.current_time = time;
                // 画面を再描画するように指示します
                should_render = true;
                // 再度1秒後に`Timer`イベントを発生させるように依頼し、ループさせます
                set_timeout(1.0);
            }
            Event::TabUpdate(tabs) => {
                self.tabs = tabs;
                should_render = true;
            }
            Event::ModeUpdate(mode) => {
                self.mode = mode;
                should_render = true;
            }
            // その他のイベントは無視します
            _ => (),
        };
        should_render
    }

    fn pipe (&mut self, _pipe_message: PipeMessage) -> bool {
        let should_render = false;
        // react to data piped to this plugin from the CLI, a keybinding or another plugin
        // read more about pipes: https://zellij.dev/documentation/plugin-pipes
        // return true if this plugin's `render` function should be called for the plugin to render
        // itself
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        // println!("Hi there! I have {rows} rows and {cols} columns.current_time {}", self.current_time);
        // print_text_with_coordinates(text, 0, 0, None, None);

        let width = 10;
        
        let mode_text = Text::new(input_mode_to_text(&self.mode))
        .color_range(2, 0..);

        print_text_with_coordinates(mode_text, 0, 0, Some(10), None);
        for (i, j) in self.tabs.iter().enumerate()
        {
            let mut text = Text::new(format!("{}", j.name));
            if j.active {
                text = text.selected();
            }
            print_ribbon_with_coordinates(text,10 + i * width, 0, Some(width), None);
        }

        let width = 15;
        let text = Text::new(format!("{}", self.current_time
                .with_timezone(&self.timezone)
                .format("%H:%M:%S").to_string())
        );
        print_ribbon_with_coordinates(text,cols - width, 0, Some(width), None);

        // Debug MSG
        print!("rows {} : cols {}", rows, cols);
    }
}


fn input_mode_to_text(mode: &ModeInfo) -> String {
    match mode.mode{
        InputMode::Normal => "NORMAL".to_string(),
        InputMode::Locked => "LOCKED".to_string(),
        InputMode::Resize => "RESIZE".to_string(),
        InputMode::Pane => "PANE".to_string(),
        InputMode::Tab => "TAB".to_string(),
        InputMode::Scroll => "SCROLL".to_string(),
        InputMode::EnterSearch => "ENTERSEARCH".to_string(),
        InputMode::Session => "SESSION".to_string(),
        InputMode::Search => "SEARCH".to_string(),
        InputMode::RenameTab => "RENAMETAB".to_string(),
        InputMode::Move => "MOVE".to_string(),
        InputMode::RenamePane => "RENAMEPANE".to_string(),
        InputMode::Prompt => "PROMPT".to_string(),
        InputMode::Tmux => "TMUX".to_string(),
    }
}
