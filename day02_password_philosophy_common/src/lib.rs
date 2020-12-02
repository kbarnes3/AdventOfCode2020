pub struct PasswordEntry {
    pub min_count: u8,
    pub max_count: u8,
    pub required_char: char,
    pub password: &'static str
}

// Transform with:
// '<,'>s/\(\d\+\)-\(\d\+\) \(\w\): \(\w\+\)/    PasswordEntry { min_count: \1, max_count: \2, required_char: '\3', password: "\4" },
pub const SAMPLE_DATA: [PasswordEntry; 3] = [
    PasswordEntry { min_count: 1, max_count: 3, required_char: 'a', password: "abcde" },
    PasswordEntry { min_count: 1, max_count: 3, required_char: 'b', password: "cdefg" },
    PasswordEntry { min_count: 2, max_count: 9, required_char: 'c', password: "ccccccccc" },
];
