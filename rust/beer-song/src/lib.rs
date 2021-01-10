pub fn verse(n: u32) -> String {
    format!(
        "{} of beer on the wall, {} of beer.\n{} of beer on the wall.\n",
        match n {
            0 => "No more bottles".into(),
            1 => "1 bottle".into(),
            _ => format!("{} bottles", n),
        },
        match n {
            0 => "no more bottles".into(),
            1 => "1 bottle".into(),
            _ => format!("{} bottles", n),
        },
        match n {
            0 => "Go to the store and buy some more, 99 bottles".into(),
            1 => "Take it down and pass it around, no more bottles".into(),
            2 => "Take one down and pass it around, 1 bottle".into(),
            _ => format!("Take one down and pass it around, {} bottles", n - 1),
        },
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<_>>().join("\n")
}
