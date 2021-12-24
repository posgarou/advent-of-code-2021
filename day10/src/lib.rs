pub enum ParseResult {
    Ok,
    Corrupted(char),
    Incomplete(String),
}

pub fn sum_corrupted_points(lines: Vec<&str>) -> i32 {
    lines
        .into_iter()
        .map(parse_line)
        .map(|result| match result {
            ParseResult::Corrupted(')') => 3,
            ParseResult::Corrupted(']') => 57,
            ParseResult::Corrupted('}') => 1197,
            ParseResult::Corrupted('>') => 25137,
            _ => 0,
        })
        .sum()
}

pub fn get_median_completion_score(lines: Vec<&str>) -> i64 {
    let mut missing_scores: Vec<i64> = lines
        .into_iter()
        .map(parse_line)
        .filter_map(|line| match line {
            ParseResult::Incomplete(missing) => Some(get_completion_score(&missing)),
            _ => None,
        })
        .collect();

    missing_scores.sort();

    missing_scores[missing_scores.len() / 2]
}

fn get_completion_score(missing_characters: &str) -> i64 {
    let letter_values: Vec<i32> = missing_characters
        .chars()
        .map(|delimiter| match delimiter {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        })
        .collect();

    letter_values
        .iter()
        .fold(0, |previous, current| previous * 5 + (*current as i64))
}

fn parse_line(input: &str) -> ParseResult {
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
                        return ParseResult::Corrupted(character);
                    }
                }
                None => return ParseResult::Corrupted(character),
            }
        }
    }

    if open_chunks.len() == 0 {
        return ParseResult::Ok;
    }

    let missing_characters = open_chunks
        .into_iter()
        .rev()
        .filter_map(get_closing_delimiter_for)
        .collect();

    ParseResult::Incomplete(missing_characters)
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
    match get_closing_delimiter_for(open_chunk) {
        Some(closing_delimiter) => input == closing_delimiter,
        None => false,
    }
}

fn get_closing_delimiter_for(input: char) -> Option<char> {
    match input {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
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
    use crate::{get_completion_score, get_median_completion_score, sum_corrupted_points};

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

    #[test]
    fn it_calculates_completion_score() {
        assert_eq!(get_completion_score("}}]])})]"), 288957);
        assert_eq!(get_completion_score(")}>]})"), 5566);
    }

    #[test]
    fn it_calculates_median_completion_score() {
        let lines = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "(((({<>}<{<{<>}{[]{[]{}",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        assert_eq!(get_median_completion_score(lines), 288957);
    }
}
