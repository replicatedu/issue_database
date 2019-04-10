extern crate reqwest;

use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

extern crate serde_json;



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
        // let mut map = HashMap::new();
        // map.insert("name", "asdasd");
        // map.insert("private", "false");
       
        let mut url_str = String::new();
        url_str.push_str(&format!("{}/issues/",self.class_repo_address));

        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");
        
        let client = reqwest::Client::new();
        let res = client.post(url)
            .basic_auth(&self.username,Some(&self.password))
            //.json(&map)
            .send()?;

        dbg!(res);
        Ok(())
    }

    pub fn get_issue( &self, issue_num:i32) -> Result<String, reqwest::Error>{
        let mut url_str = String::new();
        url_str.push_str(&format!("{}/issues/{}",self.class_repo_address,issue_num));
        dbg!(&  url_str);
        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");
        
        let client = reqwest::Client::new();
        let mut res = client.get(url)
            .send()?;
        let body = res.text().expect("error parsing");

        let deser:serde_json::Value = serde_json::from_str(&body).expect("error parsinge");
        dbg!(deser);
        Ok(body)
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
        let class_repo_address = "https://api.github.com/repos/replicatedu/issue_database";
        let issue_db = ClassIssue::new(class_repo_address.to_string(),username.to_string(),password.to_string());
        dbg!(issue_db.get_issue(1));
    }
}