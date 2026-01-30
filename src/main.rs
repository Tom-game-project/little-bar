mod clock;

use clock::clock_emoji;
use zellij_tile::prelude::*;
// chronoクレートから時刻を扱うためのものをインポートします
use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    // the state of the plugin
    timezone: Tz,
    current_time: DateTime<Utc>,
    tabs: Vec<TabInfo>,
    mode: ModeInfo,
    session: Vec<SessionInfo>,
    battery: String,
    battery_status: String,
}

register_plugin!(State);

// nerd font const settings 
// https://www.nerdfonts.com/cheat-sheet
// - battery
const NF_MD_BATTERY_CHARGING_0 : &'static str = "\u{f008e}";
const NF_MD_BATTERY_CHARGING_10: &'static str = "\u{f007a}";
const NF_MD_BATTERY_CHARGING_20: &'static str = "\u{f007b}";
const NF_MD_BATTERY_CHARGING_30: &'static str = "\u{f007c}";
const NF_MD_BATTERY_CHARGING_40: &'static str = "\u{f007d}";
const NF_MD_BATTERY_CHARGING_50: &'static str = "\u{f007e}";
const NF_MD_BATTERY_CHARGING_60: &'static str = "\u{f007f}";
const NF_MD_BATTERY_CHARGING_70: &'static str = "\u{f0080}";
const NF_MD_BATTERY_CHARGING_80: &'static str = "\u{f0081}";
const NF_MD_BATTERY_CHARGING_90: &'static str = "\u{f0082}";
const NF_MD_BATTERY            : &'static str = "\u{f0079}";
// - charge mode 
const NF_MD_POWER_PLUG         : &'static str = "\u{f06a5}";
const NF_FA_CHECK              : &'static str = "\u{f00c}" ;
const NF_MD_LIGHTNING_BOLT     : &'static str = "\u{f140b}";


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
            PermissionType::RunCommands,
        ]);
        subscribe(&[
            EventType::Timer,
            EventType::TabUpdate,
            EventType::ModeUpdate,
            EventType::SessionUpdate,
            EventType::PermissionRequestResult,
            EventType::RunCommandResult,
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
                
                run_command(
                    &["cat", "/sys/class/power_supply/BAT0/capacity"],
                    BTreeMap::from([("kind".to_string(), "capacity".to_string())]),
                );

                run_command(
                    &["cat", "/sys/class/power_supply/BAT0/status"],
                    BTreeMap::from([("kind".to_string(), "status".to_string())]));

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
            Event::SessionUpdate(session, _) => {
                self.session = session;
                should_render = true;
            }
            Event::PermissionRequestResult(result) => {
                set_selectable(false);
            }
            Event::RunCommandResult(exit_code, stdout, stderr, context) => {
                let stdout_string = String::from_utf8_lossy(&stdout);
                let stdout_string = stdout_string.to_string().trim().to_string();

                match context.get("kind").map(|x| x.as_str()) {
                    Some("capacity") =>{
                        self.battery = stdout_string;
                    }
                    Some("status") =>
                        self.battery_status = stdout_string,
                    _ => {}
                }
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

        let session_name = format!("({})", current_session_name(&self.session));
        let session_text = Text::new(session_name);
        let mode_text = input_mode_to_text(&self.mode); //.color_range(2, 0..);

        print_text_with_coordinates(session_text, 0, 0, Some(10), None);
        print_text_with_coordinates(mode_text, 10, 0, Some(10), None);
        for (i, j) in self.tabs.iter().enumerate()
        {
            let mut text = Text::new(format!("{}", j.name));
            if j.active {
                text = text.selected();
            }
            print_text_with_coordinates(text,(cols / 2) + i * width - (self.tabs.len() * width / 2), 0, Some(width), None);
        }

        // バッテリー表示の追加
        if !self.battery.is_empty() {
            let battery_width = 8;
            if let Ok(battery_num) = self.battery.parse::<i32>() {
                let icon = match battery_num {
                    0       => NF_MD_BATTERY_CHARGING_0 ,
                    1 ..10  => NF_MD_BATTERY_CHARGING_10,
                    10..20  => NF_MD_BATTERY_CHARGING_20,
                    20..30  => NF_MD_BATTERY_CHARGING_30,
                    30..40  => NF_MD_BATTERY_CHARGING_40,
                    40..50  => NF_MD_BATTERY_CHARGING_50,
                    50..60  => NF_MD_BATTERY_CHARGING_60,
                    60..70  => NF_MD_BATTERY_CHARGING_70,
                    70..80  => NF_MD_BATTERY_CHARGING_80,
                    80..90  => NF_MD_BATTERY_CHARGING_90,
                    90..100 => NF_MD_BATTERY            , 
                    _ => NF_MD_BATTERY_CHARGING_0,
                };

                let battery_text = 
                    match self.battery_status.as_str() {
                        "Charging" => Text::new(NF_MD_POWER_PLUG).color_range(2, 0..),
                        "Discharging" => Text::new(NF_MD_LIGHTNING_BOLT).color_range(2, 0..),
                        "Full" => Text::new(NF_FA_CHECK).color_range(2, 0..),
                        "Not charging" => Text::new(NF_MD_POWER_PLUG).color_range(0, 0..),
                        _ => {
                            Text::new("E").color_range(0, 0..)
                        }
                    } ;
                // 時計の左隣（cols - 15 - 8 = cols - 23）付近に配置
                print_text_with_coordinates(battery_text, cols - 23, 0, Some(battery_width), None);
                print_text_with_coordinates(Text::new(format!("{}{}%", icon, battery_num)), cols - 22, 0, Some(battery_width), None);
            } else {
                // failed to parse 
            }
        }

        let width = 15;
        let text = Text::new(format!("{} {}", &self.current_time
                .with_timezone(&self.timezone)
                .format("%H:%M")
                .to_string(),
                clock_emoji(&self.current_time.with_timezone(&self.timezone)
                )
            )
        );
        print_ribbon_with_coordinates(text,cols - width, 0, Some(width), None);
    }
}

fn current_session_name(session_list: &[SessionInfo]) -> String {
    if let Some(a) = session_list.iter().find(|a| a.is_current_session) {
        a.name.clone()
    }
    else {
        "".to_string()
    }
}

fn input_mode_to_text(mode: &ModeInfo) -> Text {
    match mode.mode{
        InputMode::Normal => Text::new("NORMAL".to_string()).color_range(1, 0..),
        InputMode::Locked => Text::new("LOCKED".to_string()).color_range(2, 0..),
        InputMode::Resize => Text::new("RESIZE".to_string()).color_range(2, 0..),
        InputMode::Pane => Text::new("PANE".to_string()).color_range(2, 0..),
        InputMode::Tab => Text::new("TAB".to_string()).color_range(2, 0..),
        InputMode::Scroll => Text::new("SCROLL".to_string()).color_range(2, 0..),
        InputMode::EnterSearch => Text::new("ENTERSEARCH".to_string()).color_range(2, 0..),
        InputMode::Session => Text::new("SESSION".to_string()).color_range(2, 0..),
        InputMode::Search => Text::new("SEARCH".to_string()).color_range(2, 0..),
        InputMode::RenameTab => Text::new("RENAMETAB".to_string()).color_range(2, 0..),
        InputMode::Move => Text::new("MOVE".to_string()).color_range(2, 0..),
        InputMode::RenamePane => Text::new("RENAMEPANE".to_string()).color_range(2, 0..),
        InputMode::Prompt => Text::new("PROMPT".to_string()).color_range(2, 0..),
        InputMode::Tmux => Text::new("TMUX".to_string()).color_range(2, 0..),
    }
}
