#[allow(dead_code)]
// instead of this
// fn parse_csv<R: std::io::BufRead>(contents: R> -> std::io::Result<Vec<Vec<String>>> {
// }
// use this (read as `content is of type that implements the trait io::BufRead`)
fn parse_csv(contents: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    contents
        .lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}

fn main() {}
