use serde::Deserialize;

#[derive(Deserialize)]
struct Story { //i suppose this is a struct
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
}

const COLORS: [&str; 8] = [
    "\x1b[31m", // red
    "\x1b[32m", // green
    "\x1b[33m", // yellow
    "\x1b[34m", // blue
    "\x1b[35m", // magenta
    "\x1b[36m", // cyan
    "\x1b[91m", // bright red
    "\x1b[94m", // bright blue
];

fn print_colored(line: &str, line_index: &mut usize) {
    let color = COLORS[*line_index % COLORS.len()];
    println!("{color}{line}\x1b[0m");
    *line_index += 1;
}

fn main() {
    let mut line_index = 0usize;

    print_colored("Top 10 Hacker News Stories", &mut line_index);
    print_colored("", &mut line_index);

    let client = reqwest::blocking::Client::new();

    let top_ids: Vec<u64> = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .expect("Failed to fetch top stories")
        .json()
        .expect("Failed to parse story IDs");

    for (i, id) in top_ids.iter().take(10).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");

        let story: Story = client
            .get(&url)
            .send()
            .expect("Failed to fetch story")
            .json()
            .expect("Failed to parse story");

        let link = story.url.as_deref().unwrap_or("(no URL)");
        let story_line = format!("{}. {} ({} points by {})", i + 1, story.title, story.score, story.by);
        let link_line = format!("  {}", link);

        print_colored(&story_line, &mut line_index);
        print_colored(&link_line, &mut line_index);
        print_colored("", &mut line_index);
    }
}