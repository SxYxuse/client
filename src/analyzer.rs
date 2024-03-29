use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Analyzer {
    pub(crate) regex_list: Vec<Regex>,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            regex_list: vec![
                NEWMON_RESP_POSITIVE_REGEX.clone(),
                NEWMON_RESP_NEGATIVE_REGEX.clone(),
                MON_REGEX.clone(),
                RESPOND_REGEX.clone(),
            ],
        }
    }

    pub fn get_groups(&self, received_message: &str) -> HashMap<String, String> {
        let mut groups = HashMap::new();
        for regex in &self.regex_list {
            if let Some(captures) = regex.captures(received_message) {
                for name in regex.capture_names().flatten() {
                    if let Some(match_) = captures.name(&name) {
                        groups.insert(name.to_string(), match_.as_str().to_string());
                    }
                }
            }
        }
        groups
    }
}

lazy_static! {
    static ref LETTER_REGEX: Regex = Regex::new(r"[A-Za-z]").unwrap();
    static ref DIGIT_REGEX: Regex = Regex::new(r"[0-9]").unwrap();
    static ref LETTER_DIGIT_REGEX: Regex = Regex::new(&format!("({}|{})", LETTER_REGEX.as_str(), DIGIT_REGEX.as_str())).unwrap();
    static ref CRLF_REGEX: Regex = Regex::new(r"\r\n").unwrap();
    static ref PORT_REGEX: Regex = Regex::new(&format!("{}{{1,5}}", DIGIT_REGEX.as_str())).unwrap();
    static ref CHARACTER_REGEX: Regex = Regex::new(r"[\x20-\xFF]").unwrap();
    static ref CHARACTER_SPEC_REGEX: Regex = Regex::new(r"[-_=./+*$Â°()\[\]{}^]").unwrap();
    static ref CHARACTER_PASS_REGEX: Regex = Regex::new(&format!("({}|{})", LETTER_DIGIT_REGEX.as_str(), CHARACTER_SPEC_REGEX.as_str())).unwrap();
    static ref SP_REGEX: Regex = Regex::new(r"\x20").unwrap();
    static ref ID_REGEX: Regex = Regex::new(&format!("{}{{5,10}}", LETTER_DIGIT_REGEX.as_str())).unwrap();
    static ref PROTOCOL_REGEX: Regex = Regex::new(&format!("{}{{3,15}}", LETTER_DIGIT_REGEX.as_str())).unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new(&format!("{}{{3,50}}", LETTER_DIGIT_REGEX.as_str())).unwrap();
    static ref PASSWORD_REGEX: Regex = Regex::new(&format!("{}{{3,50}}", CHARACTER_PASS_REGEX.as_str())).unwrap();
    static ref AUTHENTICATION_REGEX: Regex = Regex::new(&format!("{}{{3,50}}", CHARACTER_PASS_REGEX.as_str())).unwrap();
    static ref PASSWORD_AUTH_REGEX: Regex = Regex::new(&format!(r"(?P<Password>{})((?P<Auth>{}))?", PASSWORD_REGEX.as_str(), AUTHENTICATION_REGEX.as_str())).unwrap();
    static ref HOST_REGEX: Regex = Regex::new(&format!(r"({}|\.|_|-){{3,50}}", LETTER_DIGIT_REGEX.as_str())).unwrap();
    static ref PATH_REGEX: Regex = Regex::new(&format!(r"/({}|[.\-_]){{0,100}}", LETTER_DIGIT_REGEX.as_str())).unwrap();
    static ref URL_REGEX: Regex = Regex::new(&format!(r"{}://({}((:{}))??@)?{}({})?{}", PROTOCOL_REGEX.as_str(), USERNAME_REGEX.as_str(), PASSWORD_AUTH_REGEX.as_str(), HOST_REGEX.as_str(), PORT_REGEX.as_str(), PATH_REGEX.as_str())).unwrap();
    static ref STATE_REGEX: Regex = Regex::new(r"OK|ALARM|DOWN|UNKNOWN").unwrap();
    static ref MESSAGE_REGEX: Regex = Regex::new(&format!("{}{{1,200}}", CHARACTER_REGEX.as_str())).unwrap();

    pub static ref NEWMON_RESP_POSITIVE_REGEX: Regex = Regex::new(&format!(r"(?P<StatePositive>\+OK){}(?P<MessageForPositive>{})({})?", SP_REGEX.as_str(), MESSAGE_REGEX.as_str(), CRLF_REGEX.as_str())).unwrap();
    pub static ref NEWMON_RESP_NEGATIVE_REGEX: Regex = Regex::new(&format!(r"(?P<StateNegative>\-ERR){}(?P<MessageForNegative>{})({})?", SP_REGEX.as_str(), MESSAGE_REGEX.as_str(), CRLF_REGEX.as_str())).unwrap();
    pub static ref MON_REGEX: Regex = Regex::new(&format!(r"(?P<Type>MON)((?P<Id>({}{}){{0,100}}))({})?", SP_REGEX.as_str(), ID_REGEX.as_str(), CRLF_REGEX.as_str())).unwrap();
    pub static ref RESPOND_REGEX: Regex = Regex::new(&format!("(?P<Type>RESPOND){}(?P<Id>{}){}(?P<Url>{}){}(?P<State>{})({})?", SP_REGEX.as_str(), ID_REGEX.as_str(), SP_REGEX.as_str(), URL_REGEX.as_str(), SP_REGEX.as_str(), STATE_REGEX.as_str(), CRLF_REGEX.as_str())).unwrap();
}