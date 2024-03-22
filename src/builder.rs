/*pub struct Builder;

impl Builder {
    const NEWMON: &'static str = "NEWMON <augmented_url)\n\n";
    const LISTMON: &'static str = "LISTMON\n\n";
    const REQUEST: &'static str = "REQUEST <id>\n\n";

    fn replace_placeholder(template: &str, placeholder: &str, value: &str) -> String {
        template.replace(&format!("<{}>", placeholder), value)
    }

    pub fn build_newmon_message(augmented_url: &str) -> String {
        Self::replace_placeholder(Self::NEWMON, "augmented_url", augmented_url)
    }

    pub fn build_listmon_message() -> String {
        Self::LISTMON.to_string()
    }

    pub fn build_request_message(id: &str) -> String {
        Self::replace_placeholder(Self::REQUEST, "id", id)
    }
}*/

pub struct Builder;

impl Builder {
    const NEWMON: &'static str = "NEWMON <augmented_url>\r\n";
    const LISTMON: &'static str = "LISTMON\r\n";
    const REQUEST: &'static str = "REQUEST <id>\r\n";

    fn replace_placeholder(&self, template: &str, placeholder: &str, value: &str) -> String {
        template.replace(&format!("<{}>", placeholder), value)
    }

    pub fn build_newmon_message(&self, augmented_url: &str) -> String {
        self.replace_placeholder(Self::NEWMON, "augmented_url", augmented_url)
    }

    pub fn build_listmon_message(&self) -> String {
        Self::LISTMON.to_string()
    }

    pub fn build_request_message(&self, id: &str) -> String {
        self.replace_placeholder(Self::REQUEST, "id", id)
    }
}