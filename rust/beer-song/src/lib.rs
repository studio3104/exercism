pub fn verse(n: u32) -> String {
    let t1: String;
    let t2: String;
    let t3: String;

    format!(
        "{} of beer on the wall, {} of beer.\n{} of beer on the wall.\n",
        match n {
            0 => "No more bottles",
            1 => "1 bottle",
            _ => {
                t1 = format!("{} bottles", n);
                &t1
            }
        },
        match n {
            0 => "no more bottles",
            1 => "1 bottle",
            _ => {
                t2 = format!("{} bottles", n);
                &t2
            }
        },
        match n {
            0 => "Go to the store and buy some more, 99 bottles",
            1 => "Take it down and pass it around, no more bottles",
            2 => "Take one down and pass it around, 1 bottle",
            _ => {
                t3 = format!("Take one down and pass it around, {} bottles", (n - 1).to_string());
                &t3
            }
        },
    )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();

    for n in (end..=start).rev() {
        song = format!("{}{}\n", song, verse(n));
    }

    song.trim().to_string() + "\n"
}
