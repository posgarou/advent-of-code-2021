pub struct CorruptedDelimiter(pub char);

pub fn sum_corrupted_points(lines: Vec<&str>) -> i32 {
    lines
        .into_iter()
        .filter_map(is_corrupted)
        .map(
            |CorruptedDelimiter(delimiter)| match delimiter.to_string().as_str() {
                ")" => 3,
                "]" => 57,
                "}" => 1197,
                ">" => 25137,
                _ => 0,
            },
        )
        .sum()
}

pub fn is_corrupted(input: &str) -> Option<CorruptedDelimiter> {
    let mut open_chunks = vec![];

    for character in input.chars() {
        if is_opening_delimiter(character) {
            open_chunks.push(character);
        } else if is_closing_delimiter(character) {
            let current_chunk = open_chunks.last();

            match current_chunk {
                Some(delimiter) => {
                    if is_closing_delimiter_for(*delimiter, character) {
                        open_chunks.pop();
                    } else {
                        return Some(CorruptedDelimiter(character));
                    }
                }
                None => return Some(CorruptedDelimiter(character)),
            }
        }
    }

    None
}

fn is_opening_delimiter(input: char) -> bool {
    match input.to_string().as_str() {
        "(" => true,
        "[" => true,
        "{" => true,
        "<" => true,
        _ => false,
    }
}

fn is_closing_delimiter_for(open_chunk: char, input: char) -> bool {
    let input_str = input.to_string();

    match open_chunk.to_string().as_str() {
        "(" => input_str == ")",
        "[" => input_str == "]",
        "{" => input_str == "}",
        "<" => input_str == ">",
        _ => false,
    }
}

fn is_closing_delimiter(input: char) -> bool {
    match input.to_string().as_str() {
        ")" => true,
        "]" => true,
        "}" => true,
        ">" => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use crate::{is_corrupted, sum_corrupted_points};

    #[test]
    fn it_recognizes_corrupted_lines() {
        assert_eq!(
            is_corrupted("{([(<{}[<>[]}>{[]{[(<()>")
                .unwrap()
                .0
                .to_string(),
            "}"
        );

        assert_eq!(
            is_corrupted("<{([([[(<>()){}]>(<<{{")
                .unwrap()
                .0
                .to_string(),
            ">"
        )
    }

    #[test]
    fn it_sums_corrupted_points() {
        let lines = vec![
            "{([(<{}[<>[]}>{[]{[(<()>",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
        ];

        assert_eq!(sum_corrupted_points(lines), 26397);
    }
}
