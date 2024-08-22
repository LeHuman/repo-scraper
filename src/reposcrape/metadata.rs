use regex::Regex;
use std::collections::HashMap;

pub struct Metadata;

impl Metadata {
    pub fn extract(text: &str) -> HashMap<String, String> {
        let re: Regex = Regex::new(r"(?i)^\s*<!--\s*((?P<key>\w*?):\s*(?P<val>.*?)|(?P<start>\w+\s*START)|(?P<end>\w+\s*END))\s*-\s*-\s*>").unwrap();
        let re_section: Regex = Regex::new(r"(?i)^(?P<name>.+?)\s*?(START|END)").unwrap();
        let mut map: HashMap<String, String> = HashMap::new();

        let mut section_accumulator = String::new();
        let mut section_name = String::new();
        let mut section_enable = false;

        fn extract_metadata_section_name(re: &Regex, captured_line: &str) -> String {
            let full_line = captured_line.to_uppercase();
            let captures = re.captures(&full_line);

            if captures.is_some() {
                let result = captures.expect("Failed to capture line regex");
                if result.name("name").is_some() {
                    let name = result
                        .name("name")
                        .expect("Failed to get section name")
                        .as_str()
                        .to_uppercase();
                    return name;
                }
            }

            String::new()
        }

        for line in text.lines() {
            let captures = re.captures(line);
            if captures.is_some() {
                let result = captures.expect("Failed to capture line regex");
                if result.name("start").is_some() {
                    if section_enable {
                        // TODO: warn on malformed section
                        // continue;
                    }
                    let name = result
                        .name("start")
                        .expect("Failed to get start name")
                        .as_str();

                    section_name = extract_metadata_section_name(&re_section, name);
                    section_accumulator = String::new();
                    section_enable = true;
                } else if result.name("end").is_some() {
                    let name = result.name("end").expect("Failed to get end name").as_str();
                    if (!section_enable)
                        || (extract_metadata_section_name(&re_section, name) != section_name)
                    {
                        // TODO: warn on malformed section
                        continue;
                    }
                    section_accumulator.pop(); // Remove last ' '
                    map.insert(section_name.clone(), section_accumulator.clone());
                    section_enable = false;
                } else if result.name("key").is_some() && result.name("val").is_some() {
                    let key = result
                        .name("key")
                        .expect("Failed to get key")
                        .as_str()
                        .to_uppercase();
                    let val = result
                        .name("val")
                        .expect("Failed to get val")
                        .as_str()
                        .to_string(); // FIXME: Does this need to be uppercase?
                    map.insert(key, val);
                }
                // TODO: warn that capture detected but unable to parse
            } else if section_enable {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    section_accumulator.push_str(trimmed);
                    section_accumulator.push(' ');
                }
            }
        }

        map
    }
}
