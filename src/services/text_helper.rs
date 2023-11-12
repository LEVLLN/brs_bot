pub fn tokenized_text(text: Option<&str>) -> Option<Vec<Vec<&str>>> {
        Some(
            text?
                .lines()
                .map(|line| line.split_whitespace().collect())
                .collect()
        )
    }
