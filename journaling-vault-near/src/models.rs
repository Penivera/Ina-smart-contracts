struct JournalEntry {
    user: String,
    content: String,
    tags: Vec<String>,
    is_private: bool,
}

impl JournalEntry {
    pub fn new(user: String, content: String, tags: Vec<String>, is_private: bool) -> Self {
        JournalEntry {
            user,
            content,
            tags,
            is_private,
        }
    }
}