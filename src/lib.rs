use std::collections::HashMap;

/// Parses a string containing key - value pairs into a HashMap.
pub fn parse_string_to_hashmap(string: &str, delimiter: char) -> Option<HashMap<String, String>> {
    let mut map = HashMap::new();
    for line in string.lines() {
        let split = line.split(delimiter).collect::<Vec<_>>();
        let key = split.get(0)?.to_string();
        let value = split.get(1)?.to_string();
        map.insert(key, value);
    }
    Some(map)
}
