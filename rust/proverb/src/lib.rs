pub fn build_proverb(list: &[&str]) -> String {
    let mut saying = String::new();
    if list.len() == 0 {
        return saying
    }

    for i in 0..(list.len() - 1) {
        saying = format!("{}For want of a {} the {} was lost.\n", saying, list[i], list[i + 1]);
    }

    format!("{}And all for the want of a {}.", saying, list[0])
}
