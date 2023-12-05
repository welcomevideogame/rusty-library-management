pub mod structs {

    use super::super::utils;
    use crate::types::enums::{MediaType, PermissionLevel};
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use std::collections::HashMap;

    // Struct Definitions
    // ---------------------------------------------------------------

    #[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
    pub struct Employee {
        id: u16,
        name: String,
        department: String,
        boss_id: u16,
        project: String,
        subject: String,
        alloc_budget: u16,
        perm_level: PermissionLevel,
        password: String,
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
    pub struct Media {
        id: u16,
        media_type: MediaType,
        name: String,
        borrowable: bool,
        vendor: String,
        renter: String,
    }

    // Trait Implementation
    // ---------------------------------------------------------------

    pub trait DisplayInfo: Serialize + for<'de> Deserialize<'de> {
        fn get_id(&self) -> u16;
        fn get_name(&self) -> &str;
        fn get_table_name() -> &'static str;
        fn additional_setup(&mut self);
    }

    impl DisplayInfo for Employee {
        fn get_id(&self) -> u16 {
            self.id
        }
        fn get_name(&self) -> &str {
            self.name.as_str()
        }
        fn get_table_name() -> &'static str {
            "Employee"
        }
        fn additional_setup(&mut self) {
            match utils::security::hash_str(self.password.as_str()) {
                Ok(hash) => self.password = hash,
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    impl fmt::Display for Employee {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Employee Information:\n\
                 ID: {}\n\
                 Name: {}\n\
                 Department: {}\n\
                 Boss ID: {}\n\
                 Project: {}\n\
                 Subject: {}\n\
                 Allocated Budget: {}\n\
                 Permission Level: {}\n\
                 Password: {}",
                self.id,
                self.name,
                self.department,
                self.boss_id,
                self.project,
                self.subject,
                self.alloc_budget,
                self.perm_level,
                self.password
            )
        }
    }

    impl DisplayInfo for Media {
        fn get_id(&self) -> u16 {
            self.id
        }
        fn get_name(&self) -> &str {
            self.name.as_str()
        }
        fn get_table_name() -> &'static str {
            "Media"
        }
        fn additional_setup(&mut self) {
            // TODO - Nothing yet
        }
    }

    impl fmt::Display for Media {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Media Information:\n\
                 ID: {}\n\
                 Media Type: {}\n\
                 Name: {}\n\
                 Borrowable: {}\n\
                 Vendor: {}\n\
                 Renter: {}",
                self.id,
                self.media_type,
                self.name,
                if self.borrowable { "Yes" } else { "No" },
                self.vendor,
                self.renter
            )
        }
    }

    // Constructors and Getters/Setters
    // ---------------------------------------------------------------
    #[allow(clippy::too_many_arguments)]
    impl Employee {
        pub fn new(
            id: u16,
            name: String,
            department: String,
            boss_id: u16,
            project: String,
            subject: String,
            alloc_budget: u16,
            perm_level: PermissionLevel,
            password: String,
        ) -> Result<Self, &'static str> {
            if password.len() < 8 {
                return Err("Password should be at least 8 characters long.");
            }
            match utils::security::hash_str(password.as_str()) {
                Ok(hash) => Ok(Employee {
                    id,
                    name,
                    department,
                    boss_id,
                    project,
                    subject,
                    alloc_budget,
                    perm_level,
                    password: hash,
                }),
                Err(_) => Err("Error occurred while hashing password"),
            }
        }
        pub fn department(&self) -> &str {
            &self.department
        }
        pub fn boss_id(&self) -> u16 {
            self.boss_id
        }
        pub fn project(&self) -> &str {
            &self.project
        }
        pub fn subject(&self) -> &str {
            &self.subject
        }
        pub fn alloc_budget(&self) -> u16 {
            self.alloc_budget
        }
        pub fn perm_level(&self) -> &PermissionLevel {
            &self.perm_level
        }
        pub fn password(&self) -> &str {
            &self.password
        }
        pub fn set_id(&mut self, id: u16) {
            self.id = id;
        }
        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }
        pub fn set_department(&mut self, department: String) {
            self.department = department;
        }
        pub fn set_boss_id(&mut self, boss_id: u16) {
            self.boss_id = boss_id;
        }
        pub fn set_project(&mut self, project: String) {
            self.project = project;
        }
        pub fn set_subject(&mut self, subject: String) {
            self.subject = subject;
        }
        pub fn set_alloc_budget(&mut self, alloc_budget: u16) {
            self.alloc_budget = alloc_budget;
        }
        pub fn set_perm_level(&mut self, perm_level: PermissionLevel) {
            self.perm_level = perm_level;
        }
        pub fn set_password(&mut self, password: String) {
            match utils::security::hash_str(password.as_str()) {
                Ok(hash) => self.password = hash,
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    impl Media {
        pub fn new(
            id: u16,
            media_type: MediaType,
            name: String,
            borrowable: bool,
            vendor: String,
            renter: String,
        ) -> Media {
            Media {
                id,
                media_type,
                name,
                borrowable,
                vendor,
                renter,
            }
        }
        pub fn media_type(&self) -> &MediaType {
            &self.media_type
        }
        pub fn borrowable(&self) -> bool {
            self.borrowable
        }
        pub fn vendor(&self) -> &str {
            &self.vendor
        }
        pub fn renter(&self) -> &str {
            &self.renter
        }
        pub fn set_id(&mut self, id: u16) {
            self.id = id;
        }
        pub fn set_media_type(&mut self, media_type: MediaType) {
            self.media_type = media_type;
        }
        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }
        pub fn set_borrowable(&mut self, borrowable: bool) {
            self.borrowable = borrowable;
        }
        pub fn set_vendor(&mut self, vendor: String) {
            self.vendor = vendor;
        }
        pub fn set_renter(&mut self, renter: String) {
            self.renter = renter;
        }
    }


    #[derive(Default)]
    struct TreeNode {
        word: bool,
        children: HashMap<char, TreeNode>,
    }
    
    impl TreeNode {
        pub fn new() -> Self {
            TreeNode {
                word: false,
                children: HashMap::new(),
            }
        }
    }
    
    pub struct Trie {
        root: TreeNode,
    }
    
    impl Trie {
        pub fn new() -> Self {
            Trie { root: TreeNode::new() }
        }
    
        pub fn insert(&mut self, word: String) {
            let mut cur = &mut self.root;
            for c in word.chars() {
                cur = cur.children.entry(c).or_insert(TreeNode::new());
            }
            cur.word = true;
        }
    
        pub fn search(&self, word: String) -> Option<Vec<String>> {
            let mut result = Vec::new();
            if let Some(node) = self.traverse(&word) {
                if node.word {
                    result.push(word.clone());
                }
                let mut current_word = word.clone();
                self.collect_words(node, &mut current_word, &mut result);
                Some(result)
            } else {
                None
            }
        }
    
        pub fn starts_with(&self, prefix: String) -> Option<Vec<String>> {
            let mut result = Vec::new();
            if let Some(node) = self.traverse(&prefix) {
                let mut current_word = prefix.clone();
                self.collect_words(node, &mut current_word, &mut result);
                Some(result)
            } else {
                None
            }
        }
    
        fn traverse(&self, prefix: &String) -> Option<&TreeNode> {
            let mut cur = &self.root;
            for c in prefix.chars() {
                match cur.children.get(&c) {
                    Some(child) => {
                        cur = child;
                    }
                    None => {
                        return None;
                    }
                }
            }
            Some(cur)
        }
    
        fn collect_words(&self, node: &TreeNode, current_word: &mut String, result: &mut Vec<String>) {
            if node.word {
                result.push(current_word.clone());
            }
            for (char, child_node) in &node.children {
                current_word.push(*char);
                self.collect_words(child_node, current_word, result);
                current_word.pop();
            }
        }
    }
    
}

pub mod enums {
    use std::cmp::Ordering;
    use std::fmt;

    #[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
    pub enum PermissionLevel {
        Basic,
        User,
        Manager,
        Admin,
        Dev,
        #[default]
        None,
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
    pub enum MediaType {
        Book,
        VideoGame,
        Movie,
        Music,
        #[default]
        None,
    }

    impl PartialEq for PermissionLevel {
        fn eq(&self, other: &Self) -> bool {
            self.to_ordinal() == other.to_ordinal()
        }
    }

    impl PartialOrd for PermissionLevel {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.to_ordinal().cmp(&other.to_ordinal()))
        }
    }

    impl PermissionLevel {
        fn to_ordinal(&self) -> u8 {
            match self {
                PermissionLevel::None => 0,
                PermissionLevel::Basic => 1,
                PermissionLevel::User => 2,
                PermissionLevel::Manager => 3,
                PermissionLevel::Admin => 4,
                PermissionLevel::Dev => 5,
            }
        }
    }

    impl fmt::Display for PermissionLevel {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                PermissionLevel::Basic => write!(f, "Basic"),
                PermissionLevel::User => write!(f, "User"),
                PermissionLevel::Manager => write!(f, "Manager"),
                PermissionLevel::Admin => write!(f, "Admin"),
                PermissionLevel::Dev => write!(f, "Dev"),
                PermissionLevel::None => write!(f, "None"),
            }
        }
    }

    impl fmt::Display for MediaType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MediaType::Book => write!(f, "Book"),
                MediaType::VideoGame => write!(f, "Video Game"),
                MediaType::Movie => write!(f, "Movie"),
                MediaType::Music => write!(f, "Music"),
                MediaType::None => write!(f, "None"),
            }
        }
    }
}

// Tests
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::PermissionLevel;
    use crate::types::structs::Trie;

    #[test]
    fn create_employee() {
        let test_employee = structs::Employee::new(
            10,
            String::from("John Doe"),
            String::from("IT"),
            1,
            String::from("Ambitious Project"),
            String::from("Computer Science"),
            1_000,
            PermissionLevel::Manager,
            String::from("password"),
        );
        assert!(test_employee.is_ok())
    }

    #[test]
    fn create_bad_employee() {
        let test_employee = structs::Employee::new(
            10,
            String::from("John Doe"),
            String::from("IT"),
            1,
            String::from("Ambitious Project"),
            String::from("Computer Science"),
            1_000,
            PermissionLevel::Manager,
            String::from("pass"),
        );
        assert!(test_employee.is_err())
    }

    #[test]
    fn test_starts_with() {
        let mut trie = Trie::new();
        trie.insert("hello".to_string());
        trie.insert("world".to_string());

        assert_eq!(trie.starts_with("he".to_string()), Some(vec!["hello".to_string()]));
        assert_eq!(trie.starts_with("wo".to_string()), Some(vec!["world".to_string()]));
        assert_eq!(trie.starts_with("foo".to_string()), None);
    }

    #[test]
    fn test_empty_trie() {
        let trie = Trie::new();
        assert_eq!(trie.search("hello".to_string()), None);
        assert_eq!(trie.starts_with("he".to_string()), None);
    }
}