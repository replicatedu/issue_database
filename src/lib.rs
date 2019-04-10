extern crate reqwest;

use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

//holds data for instructor and students
pub struct ClassIssue {
	class_repo_address: String,
    username: String,
    password: String
}

impl ClassIssue {
    pub fn new( 
                class_repo_address: String, 
                username: String, 
                password: String) -> ClassIssue {
        ClassIssue {
            class_repo_address,
            username,
            password
        }
    }

    pub fn write_issue( &self)-> Result<(), Box<std::error::Error>> {        
        // This will POST a body of `{"lang":"rust","body":"json"}`
        let mut map = HashMap::new();
        map.insert("lang", "rust");
        map.insert("body", "json");

        let client = reqwest::Client::new();
        
        let url = reqwest::Url::parse(&self.class_repo_address).expect("invalid issue writing url");
        let res = client.post(url)
            .basic_auth(&self.username,Some(&self.password))
            .json(&map)
            .send()?;

        dbg!(res);
        Ok(())
    }

    pub fn get_issue( &self, issue_num:i32){
    }
    pub fn close_issue( &self, issue_num:i32){
    }
    pub fn comment_on_issue( &self, issue_num:i32){
    }
    pub fn return_open_issue_numbers( &self ){
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_issue() {
        let username = "hortinstein";
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let class_repo_address = "https://www.github.com/replicatedu/issue_database";
        let issue_db = ClassIssue::new(class_repo_address.to_string(),username.to_string(),password.to_string());
        println!("asdasd");
        issue_db.write_issue();
    }
}