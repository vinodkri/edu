
use std::{collections::HashMap};
#[derive(Debug)]
pub struct GetSimilarTitles {
    similar_titles_db: HashMap<String, Vec<String>>,
}

impl GetSimilarTitles {
    pub fn new() -> Self {
        Self {
            similar_titles_db: HashMap::new()
        }
    }

    fn generate_key(title: String) -> String {
        let mut count = vec![0; 26];
        for c in title.chars() {
            let index = (c as u32 - 'a' as u32) as usize;
            count[index] += 1;
        }

        let mut key = String::from("");

        for k in 0..26 {
            key = key + "#";
            key = key + &count[k].to_string();
        }
        key
    }

    pub fn insert_title(&mut self, title: String) {
        let key = GetSimilarTitles::generate_key(title.clone());
        //println!("inserting: {} : {}", key, title);
        self.similar_titles_db.entry(key).or_insert(Vec::new()).push(title);

        // for (key, value) in &self.similar_titles_db {
        //     println!("{}: {:?}", key, value);
        // }
    }

    pub fn retrieve_similar_titles(&self, title: String) -> &Vec<String> {
        let key = GetSimilarTitles::generate_key(title);
        self.similar_titles_db.get(&key).unwrap()
    }
}
        
#[cfg(test)]
mod tests {
    use std::vec;

    use super::GetSimilarTitles;

    #[test]
    fn test_1() {
        let mut collections = GetSimilarTitles::new();
        let titles = vec!["duel", "dule", "speed", "spede", "deul", "cars" ];
        for title in titles.into_iter() {
            collections.insert_title(String::from(title));
        }

        let res = collections.retrieve_similar_titles(String::from("duel"));
        for title in vec!["duel", "dule"] {
            assert_eq!(res.contains(&title.to_string()), true);
        }
    }
}
