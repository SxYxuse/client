#[cfg(test)]
mod tests {
    use crate::analyzer::Analyzer;
    
    #[test]
    fn test_get_groups_for_respond_message() {
        let analyzer = Analyzer::new();
        let message = "RESPOND 12345 https://www.swilabus.com/ OK\r\n";
        let groups = analyzer.get_groups(message);

        assert_eq!(groups.get("Type").unwrap(), "RESPOND");
        assert_eq!(groups.get("Id").unwrap(), "12345");
        assert_eq!(groups.get("Url").unwrap(), "https://www.swilabus.com/");
        assert_eq!(groups.get("State").unwrap(), "OK");
    }

    #[test]
    fn test_get_groups_for_mon_message() {
        let analyzer = Analyzer::new();
        let message = "MON http1 http2\r\n";
        let groups = analyzer.get_groups(message);

        assert_eq!(groups.get("Type").unwrap(), "MON");
        assert_eq!(groups.get("Id").unwrap(), " http1 http2");
    }

    #[test]
    fn test_get_groups_for_newmon_message_negative() {
        let analyzer = Analyzer::new();
        let message = "-ERR Hello\r\n";
        let groups = analyzer.get_groups(message);

        assert_eq!(groups.get("StateNegative").unwrap(), "-ERR");
        assert_eq!(groups.get("MessageForNegative").unwrap(), "Hello");
    }

    #[test]
    fn test_get_groups_for_newmon_message_positive() {
        let analyzer = Analyzer::new();
        let message = "+OK Hello\r\n";
        let groups = analyzer.get_groups(message);

        assert_eq!(groups.get("StatePositive").unwrap(), "+OK");
        assert_eq!(groups.get("MessageForPositive").unwrap(), "Hello");
    }
}