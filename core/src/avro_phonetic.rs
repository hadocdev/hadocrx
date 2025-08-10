use std::{cell::RefCell, collections::HashMap, ffi::{c_char, CStr, CString}, sync::{Arc, Mutex, OnceLock}};

use ffi_convert::CReprOf;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum MatchType {
    Prefix,
    Suffix,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Scope {
    Consonant = 1,
    Vowel = 2,
    Punctuation = 4,
    Exact = 8,
    NotConsonant = 16,
    NotVowel = 32,
    NotPunctuation = 64,
    NotExact = 128,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct MatchRule {
    match_type: MatchType,
    scope: u8,
    exact_value: Option<char>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ConditionalRule {
    conditions: Vec<MatchRule>,
    replacement: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Pattern {
    find: String,
    find_len: usize,
    default_replace: String,
    conditional_rules: Vec<ConditionalRule>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AvroPhonetic {
    patterns_by_length: HashMap<usize, Vec<Pattern>>,
    max_pattern_length: usize,
    vowel_set: Vec<char>, 
    consonant_set: Vec<char>,
    case_sensitive_set: Vec<char>,
}

#[allow(dead_code)]
impl AvroPhonetic {
    fn new() -> Self {
        let mut phonetic = AvroPhonetic {
            patterns_by_length: HashMap::new(),
            max_pattern_length: 0,
            vowel_set: Vec::new(),
            consonant_set: Vec::new(),
            case_sensitive_set: Vec::new()
        }; 

        phonetic.load_patterns_from_json(); 
        phonetic
    }

    fn is_vowel(&self, c: &char) -> bool { self.vowel_set.contains(&c.to_ascii_lowercase()) }
    fn is_consonant(&self, c: &char) -> bool { self.consonant_set.contains(&c.to_ascii_lowercase()) }
    fn is_punctuation(&self, c: &char) -> bool { !self.is_vowel(c) && !self.is_consonant(c) }
    fn is_case_sensitive(&self, c: &char) -> bool { self.case_sensitive_set.contains(&c.to_ascii_lowercase()) }
    fn fix_string(&self, input: &str) -> String {
        input.chars().map( |c| {
            if self.is_case_sensitive(&c) { c }
            else { c.to_ascii_lowercase() }
        }).collect()
    }

    fn check_condition(&self, condition: &MatchRule, text: &str, start: usize, end: usize) -> bool {
        let chars: Vec<char> = text.chars().collect();
        let chars_len = chars.len();
        let check_char = match condition.match_type {
            MatchType::Prefix => { 
                if start < 1 { ' ' }
                else { chars[start - 1] }
            },
            MatchType::Suffix => {
                if end + 1 >= chars_len { ' ' }
                else { chars[end + 1] }
            }
        };
        // Check exact match first
        if condition.scope & (Scope::Exact as u8) != 0 {
            if let Some(exact) = condition.exact_value {
                return check_char == exact;
            }
        }
        
        if condition.scope & (Scope::NotExact as u8) != 0 {
            if let Some(exact) = condition.exact_value {
                return check_char != exact;
            }
        }
        
        // Check character type conditions
        let mut result = true;
        
        if condition.scope & (Scope::Consonant as u8) != 0 {
            result &= self.is_consonant(&check_char);
        }
        
        if condition.scope & (Scope::NotConsonant as u8) != 0 {
            result &= !self.is_consonant(&check_char);
        }
        
        if condition.scope & (Scope::Vowel as u8) != 0 {
            result &= self.is_vowel(&check_char);
        }
        
        if condition.scope & (Scope::NotVowel as u8) != 0 {
            result &= !self.is_vowel(&check_char);
        }
        
        if condition.scope & (Scope::Punctuation as u8) != 0 {
            result &= self.is_punctuation(&check_char);
        }
        
        if condition.scope & (Scope::NotPunctuation as u8) != 0 {
            result &= !self.is_punctuation(&check_char);
        }
        
        result
    }

    fn convert(&self, input: &str) -> String {
        let fixed_string = self.fix_string(input);
        let mut result = String::with_capacity(fixed_string.len()*2);
        let mut i = 0;
        let chars: Vec<char> = fixed_string.chars().collect();
        let chars_len = chars.len();
        while i < chars_len {
            let mut matched = false;
            for length in (1..=self.max_pattern_length.min(chars_len - i)).rev() {
                if let Some(patterns) = self.patterns_by_length.get(&length) {
                    let start = i;
                    let end = i + length - 1;
                    let substring: String = chars[start..=end].iter().collect();
                    for pattern in patterns {
                        if pattern.find == substring {
                            let rules = &pattern.conditional_rules;
                            let replacement = if !rules.is_empty() {
                                let mut matched_rule = false;
                                let mut rule_replacement = String::new();
                                for rule in rules {
                                    if rule.conditions.iter().all(|cond| self.check_condition(cond, &fixed_string, start, end)) {
                                        rule_replacement = rule.replacement.clone();
                                        matched_rule = true;
                                        break;
                                    }
                                }
                                if matched_rule {
                                    rule_replacement
                                } else {
                                    pattern.default_replace.clone()
                                }
                            } else {
                                pattern.default_replace.clone()
                            };
                            result.push_str(&replacement);
                            i += length;
                            matched = true;
                            break;
                        }
                    }
                    if matched {
                        break;
                    }
                }
            }
            if !matched {
                result.push(chars[i]);
                i += 1;
            }
        }
        result
    }

    fn load_patterns_from_json(&mut self) {
        let json_string = include_str!("./replacements.json");
        let json_value = json::parse(json_string).unwrap();
        self.vowel_set = json_value["vowel"].to_string().chars().collect();
        self.consonant_set = json_value["consonant"].to_string().chars().collect();
        self.case_sensitive_set = json_value["casesensitive"].to_string().chars().collect();
        let patterns = &json_value["patterns"];
        for i in 0..patterns.len() {
            let pattern_json = &patterns[i];
            let find = pattern_json["find"].to_string();
            let replace = pattern_json["replace"].to_string();
            let find_len = find.len();
            self.max_pattern_length = self.max_pattern_length.max(find_len);
        
            let mut parsed_rules: Vec<ConditionalRule> = Vec::new();
            if pattern_json["rules"] != json::Null {
                let rules = &pattern_json["rules"];
                for i in 0..rules.len() {
                    let rule = &rules[i];
                    if rule["matches"] != json::Null {
                        let matches = &rule["matches"];
                        let mut conditions: Vec<MatchRule> = Vec::new();
                        for i in 0..matches.len() {
                            let match_rule = &matches[i];
                            let match_type = match match_rule["type"].as_str().unwrap() {
                                "prefix" => MatchType::Prefix,
                                "suffix" => MatchType::Suffix,
                                t => panic!("Inavalid match_type: {}", t),
                            };
                            let scope_str = match_rule["scope"].as_str().unwrap();
                            let mut scope_flags = 0u8;
                            
                            // Parse scope efficiently
                            if scope_str.starts_with('!') {
                                let actual_scope = &scope_str[1..];
                                match actual_scope {
                                    "consonant" => scope_flags |= Scope::NotConsonant as u8,
                                    "vowel" => scope_flags |= Scope::NotVowel as u8,
                                    "punctuation" => scope_flags |= Scope::NotPunctuation as u8,
                                    "exact" => scope_flags |= Scope::NotExact as u8,
                                    _ => {}
                                }
                            } else {
                                match scope_str {
                                    "consonant" => scope_flags |= Scope::Consonant as u8,
                                    "vowel" => scope_flags |= Scope::Vowel as u8,
                                    "punctuation" => scope_flags |= Scope::Punctuation as u8,
                                    "exact" => scope_flags |= Scope::Exact as u8,
                                    _ => {}
                                }
                            }
                            let exact_value = match_rule["value"].as_str().and_then(|s| s.chars().next());
                            conditions.push(MatchRule {
                                match_type,
                                scope: scope_flags,
                                exact_value
                            });
                        }
                        parsed_rules.push(ConditionalRule { conditions, replacement: rule["replace"].to_string() });
                    }
                }
            }
            let pattern = Pattern {
                find_len,
                find,
                default_replace: replace,
                conditional_rules: parsed_rules
            };
            self.patterns_by_length.entry(find_len).or_insert_with(Vec::new).push(pattern);
        }
        for patterns in self.patterns_by_length.values_mut() {
            patterns.sort_by(|a, b| b.find_len.cmp(&a.find_len));
        }
    }
}

static AVRO_PHONETIC: OnceLock<Arc<Mutex<AvroPhonetic>>> = OnceLock::new();
thread_local! {
    static LAST_CONVERTED_TEXT: RefCell<Option<CString>> = RefCell::new(None);
}

#[allow(dead_code)]
fn get_avro_phonetic() -> &'static Arc<Mutex<AvroPhonetic>> { 
    AVRO_PHONETIC.get_or_init(|| Arc::new(Mutex::new(AvroPhonetic::new())))
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn avro_phonetic_convert(text: *const c_char) -> *const c_char {
    let text_str = unsafe { CStr::from_ptr(text).to_str().unwrap() };
    let avro_phonetic_arc_mutex = get_avro_phonetic();
    let avro_phonetic_guard = avro_phonetic_arc_mutex.lock().unwrap();
    let converted_text = avro_phonetic_guard.convert(text_str);
    let cstring = CString::c_repr_of(converted_text).unwrap();
    let ptr = cstring.as_ptr();
    LAST_CONVERTED_TEXT.with(|last| {
        *last.borrow_mut() = Some(cstring);
    });
    ptr
}
