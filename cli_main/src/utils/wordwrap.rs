use terminal_size::terminal_size;

pub fn term_width() -> Option<u16> {
    let (w, _) = terminal_size()?;
    Some(w.0)
}

// percent: 0.0 - 100.0; bar: [====>-----]  40.00%
pub fn process_bar(width: u16, percent: f64) -> (String, String, String, String, String, String) {
    let tool_head = "[".to_string();
    let tool_tail = "]".to_string();
    let num_s = format!("{:7.2}%", percent);
    let tool_arrow;
    let mut bar_ed = String::new();
    let mut bar_to = String::new();
    let total: u16 = unsafe { ((width - 10) as f64 * percent / 100.0).to_int_unchecked() };
    if total == width - 10 {
        // No need to display the `>`
        tool_arrow = String::new();
        bar_to = String::new();
        for _ in 0..total {
            bar_ed.push('=');
        }
    } else {
        tool_arrow = ">".to_string();
        for _ in 0..total {
            bar_ed.push('=');
        }
        for _ in total..width - 11 {
            bar_to.push('-');
        }
    }
    (tool_head, bar_ed, tool_arrow, bar_to, tool_tail, num_s)
}

pub fn time_info(
    width: u16,
    over: bool,
    ddl: String,
    importance: String,
    rest: String,
) -> (String, String, String, String, String, String) {
    let over = (if over { "[Over!] " } else { "" }).to_string();
    let mut importance = importance;
    let rest = rest + "\n";
    let mut blank_0 = "".to_string();
    let mut blank_1 = "".to_string();

    if width < 72 {
        importance += "\n"
    } else {
        let blank_sze = width - importance.len() as u16;
        let left = blank_sze >> 1;
        for _ in (over.len() + ddl.len()) as u16..left {
            blank_0.push(' ');
        }
        for _ in (left + importance.len() as u16)..=(width - rest.len() as u16) {
            blank_1.push(' ');
        }
    }

    (over, ddl, blank_0, importance, blank_1, rest)
}

pub fn entry_title(width: u16, title: String) -> (String, String, String) {
    if width < (title.len() + 8) as u16 {
        ("\n{-- ".to_string(), title, " --}\n".to_string())
    } else {
        let left = (width - title.len() as u16 - 8) >> 1;
        let mut blank_0 = "\n".to_string();
        for _ in 0..left {
            blank_0.push(' ');
        }
        (blank_0 + "{-- ", title, " --}\n".to_string())
    }
}
