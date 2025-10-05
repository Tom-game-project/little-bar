use chrono::{DateTime, Timelike};
use chrono_tz::Tz;

pub fn clock_emoji(date: &DateTime<Tz>) -> String {
    let (_, h) = date.hour12();
    let h = h % 12;
    let m = date.minute();

    if h == 1 && m < 30 { "🕐".to_string() }
    else if h == 1 && 30 <= m && m < 60 { "🕜".to_string() }
    else if h == 2 &&  m < 30 { "🕑".to_string() }
    else if h == 2 && 30 <= m && m < 60 { "🕝".to_string() }
    else if h == 3 && m < 30 { "🕒".to_string() }
    else if h == 3 && 30 <= m && m < 60 { "🕞".to_string() }
    else if h == 4 &&  m < 30 { "🕓".to_string() }
    else if h == 4 && 30 <= m && m < 60 { "🕟".to_string() }
    else if h == 5 && m < 30 { "🕔".to_string() }
    else if h == 5 && 30 <= m && m < 60 { "🕠".to_string() }
    else if h == 6 &&  m < 30 { "🕕".to_string() }
    else if h == 6 && 30 <= m && m < 60 { "🕡".to_string() }
    else if h == 7 &&  m < 30 { "🕖".to_string() }
    else if h == 7 && 30 <= m && m < 60 { "🕢".to_string() }
    else if h == 8 &&  m < 30 { "🕗".to_string() }
    else if h == 8 && 30 <= m && m < 60 { "🕣".to_string() }
    else if h == 9 &&  m < 30 { "🕘".to_string() }
    else if h == 9 && 30 <= m && m < 60 { "🕤".to_string() }
    else if h == 10 && m < 30 { "🕙".to_string() }
    else if h == 10 && 30 <= m && m < 60 { "🕥".to_string() }
    else if h == 11 && m < 30 { "🕚".to_string() }
    else if h == 11 && 30 <= m && m < 60 { "🕦".to_string() }
    else if h == 0 && m < 30 { "🕛".to_string() }
    else if h == 0 && 30 <= m && m < 60 { "🕧".to_string() }
    else {unreachable!()}
}
