#[cfg(test)]
mod tests {
    use crate::analyzer::Analyzer;

    #[test]
    fn test_regexes() {
        let analyzer = Analyzer::new();
        let messages = vec![
            "+OK Hello World\r\n",
            "MON 12345\r\n",
            "RESPOND 12345 https://www.swilabus.com/ OK\r\n"
        ];

        for (i, message) in messages.iter().enumerate() {
            assert!(analyzer.regex_list[i].is_match(message), "Message {} does not match regex {}", message, i);
        }
    }
}