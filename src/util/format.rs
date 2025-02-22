use std::time;

pub fn file_size_to_string(file_size: u64) -> String {
    const FILE_UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "P"];
    const CONV_RATE: f64 = 1024.0;
    let mut file_size: f64 = file_size as f64;

    let mut index = 0;
    while file_size > CONV_RATE {
        file_size /= CONV_RATE;
        index += 1;
    }

    if file_size >= 100.0 {
        format!("{:>4.0} {}", file_size, FILE_UNITS[index])
    } else if file_size >= 10.0 {
        format!("{:>4.1} {}", file_size, FILE_UNITS[index])
    } else {
        format!("{:>4.2} {}", file_size, FILE_UNITS[index])
    }
}

pub fn mtime_to_string(mtime: time::SystemTime) -> String {
    const MTIME_FORMATTING: &str = "%Y-%m-%d %H:%M";

    let datetime: chrono::DateTime<chrono::offset::Local> = mtime.into();
    datetime.format(MTIME_FORMATTING).to_string()
}

pub fn format_tab_bar_title_string(
    max_len: usize,
    number: Option<usize>,
    title: impl Into<String>,
) -> String {
    let title: String = title.into();

    if let Some(number) = number {
        if title.len() > max_len {
            format!(
                "{}: {}…",
                number + 1,
                title.chars().take(max_len - 1).collect::<String>()
            )
        } else {
            format!("{}: {}", number + 1, title)
        }
    } else if title.len() > max_len {
        format!("{}…", title.chars().take(max_len - 1).collect::<String>())
    } else {
        title.to_string()
    }
}
