extern crate reqwest;

use std::collections::HashMap;

//holds data for instructor and students
pub struct ClassIssueRequester {
    pub class_repo_address: String,
    pub username: String,
    pub password: String,
}

impl ClassIssueRequester {
    pub fn new(
        class_repo_address: String,
        username: String,
        password: String,
    ) -> ClassIssueRequester {
        ClassIssueRequester {
            class_repo_address,
            username,
            password,
        }
    }

    pub fn add_issue(
        &self,
        title: &str,
        body: &str,
        label: &str,
    ) -> Result<String, Box<std::error::Error>> {
        let label = [label];

        let mut url_str = String::new();
        url_str.push_str(&format!("{}/issues", self.class_repo_address));

        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");

        let client = reqwest::Client::new();
        let mut res = client
            .post(url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&json!({
                "title": title,
                "body": body,
                "labels": label
            }))
            .send()?;
        let body = res.text().expect("error parsing");
        Ok(body)
    }
    pub fn comment_on_issue(
        &self,
        body: &str,
        issue_num: u32,
    ) -> Result<String, Box<std::error::Error>> {
        let mut url_str = String::new();
        url_str.push_str(&format!(
            "{}/issues/{}/comments",
            self.class_repo_address, issue_num
        ));
        let client = reqwest::Client::new();
        //dbg!(&url_str);
        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");

        let mut res = client
            .post(url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&json!({ "body": body }))
            .send()?;
        let body = res.text().expect("error parsing");
        //dbg!(&body);
        Ok(body)
    }
    pub fn get_issue(&self, issue_num: u32) -> Result<String, Box<std::error::Error>> {
        let mut url_str = String::new();
        url_str.push_str(&format!("{}/issues/{}", self.class_repo_address, issue_num));
        //dbg!(&url_str);
        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");

        let client = reqwest::Client::new();
        let mut res = client.get(url).send()?;
        dbg!(&res);
        let body = res.text().expect("error parsing");
        Ok(body)
    }
    //GET /repos/:owner/:repo/issues/:issue_number/comments
    pub fn get_issue_comments(&self, issue_num: u32) -> Result<String, Box<std::error::Error>> {
        let mut url_str = String::new();
        url_str.push_str(&format!(
            "{}/issues/{}/comments",
            self.class_repo_address, issue_num
        ));
        //dbg!(&url_str);
        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");

        let client = reqwest::Client::new();
        let mut res = client.get(url).send()?;
        let body = res.text().expect("error parsing");
        Ok(body)
    }
    pub fn get_open_issues(&self, label: String) -> Result<String, Box<std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("state", "open");
        map.insert("labels", &label);

        let mut url_str = String::new();
        url_str.push_str(&format!("{}/issues", self.class_repo_address));
        //dbg!(&url_str);
        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");

        let client = reqwest::Client::new();
        let mut res = client.get(url).json(&map).send()?;
        let body = res.text().expect("error parsing");
        Ok(body)
    }

    pub fn get_all_issues(&self, label: String) -> Result<String, Box<std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("state", "all");
        map.insert("labels", &label);

        let mut url_str = String::new();
        url_str.push_str(&format!("{}/issues", self.class_repo_address));
        //dbg!(&url_str);
        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");

        let client = reqwest::Client::new();
        let mut res = client.get(url).query(&map).send()?;
        let body = res.text().expect("error parsing");
        Ok(body)
    }

    pub fn get_all_my_issues(&self, label: String) -> Result<String, Box<std::error::Error>> {
        let mut map = HashMap::new();
        let username = &self.username;
        let state = &"all".to_string();
        map.insert("creator", username);
        map.insert("state", state);
        map.insert("labels", &label);
        let mut url_str = String::new();
        url_str.push_str(&format!("{}/issues", self.class_repo_address));
        //dbg!(&url_str);
        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");

        let client = reqwest::Client::new();
        let mut res = client.get(url).query(&map).send()?;
        let body = res.text().expect("error parsing");
        Ok(body)
    }

    pub fn close_issue(&self, issue_num: u32) -> Result<String, Box<std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("state", "closed");

        let mut url_str = String::new();
        url_str.push_str(&format!("{}/issues/{}", self.class_repo_address, issue_num));
        //dbg!(&url_str);
        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");

        let client = reqwest::Client::new();
        let mut res = client
            .patch(url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&map)
            .send()?;
        let body = res.text().expect("error parsing");
        Ok(body)
    }
    pub fn edit_comment(&self, body: &str, id: u32) -> Result<String, Box<std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("body", body);

        let mut url_str = String::new();
        url_str.push_str(&format!(
            "{}/issues/comments/{}",
            self.class_repo_address, id
        ));
        dbg!(&url_str);
        let url = reqwest::Url::parse(&url_str).expect("invalid issue writing url");

        let client = reqwest::Client::new();
        let mut res = client
            .patch(url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&map)
            .send()?;
        let body = res.text().expect("error parsing");
        dbg!(&body);
        Ok(body)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    const USERNAME: &str = "hortinstein";
    const CLASS_REPO_ADDRESS: &str = "https://api.github.com/repos/replicatedu/issue_database";

    #[test]
    fn close_issue() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue_db = ClassIssueRequester::new(
            CLASS_REPO_ADDRESS.to_string(),
            USERNAME.to_string(),
            password.to_string(),
        );
        let body = issue_db.close_issue(1).expect("error closing");
        let deser: serde_json::Value = serde_json::from_str(&body).expect("error parsinge");
        dbg!(&deser);
        assert!(deser["state"] == "closed");
    }
    #[test]
    fn comment_issue() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue_db = ClassIssueRequester::new(
            CLASS_REPO_ADDRESS.to_string(),
            USERNAME.to_string(),
            password.to_string(),
        );
        let body = issue_db
            .comment_on_issue("test comment", 1)
            .expect("error closing");
        let deser: serde_json::Value = serde_json::from_str(&body).expect("error parsinge");
        dbg!(&deser);
    }
    #[test]
    fn add_issue() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue_db = ClassIssueRequester::new(
            CLASS_REPO_ADDRESS.to_string(),
            USERNAME.to_string(),
            password.to_string(),
        );
        let body = issue_db
            .add_issue("this is a unit test", "testing add issue", "test")
            .expect("error closing");
        let deser: serde_json::Value = serde_json::from_str(&body).expect("error parsinge");
        dbg!(&deser);
        assert!(deser["state"] == "open");
    }
    #[test]
    fn get_issue() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue_db = ClassIssueRequester::new(
            CLASS_REPO_ADDRESS.to_string(),
            USERNAME.to_string(),
            password.to_string(),
        );
        let body = issue_db.get_issue(8).expect("error closing");
        let deser: serde_json::Value = serde_json::from_str(&body).expect("error parsinge");
        dbg!(&deser);
        assert!(&deser["title"] == "this is a unit test");
        assert!(&deser["body"] == "testing add issue");
        let array_val = deser["labels"].clone();
        dbg!(&array_val);
        //let array: Vec<String> = serde_json::from_value(array_val).expect("error");
        let a = array_val[0]["name"].clone();
        let testlabel: String = serde_json::from_value(a).expect("not good");
        assert!(testlabel == "test");
    }
    #[test]
    fn get_open_issues() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue_db = ClassIssueRequester::new(
            CLASS_REPO_ADDRESS.to_string(),
            USERNAME.to_string(),
            password.to_string(),
        );
        let body = issue_db
            .get_open_issues("test".to_string())
            .expect("error closing");
        let deser: Vec<serde_json::Value> = serde_json::from_str(&body).expect("error parsinge");
        for x in deser {
            let title: String = serde_json::from_value(x["title"].clone()).expect("err");
            let number: u32 = serde_json::from_value(x["number"].clone()).expect("err");
            let issue_body: String = serde_json::from_value(x["body"].clone()).expect("err");
            dbg!(title);
            dbg!(number);
            //dbg!(issue_body);
            issue_db.close_issue(number);
        }
    }

    #[test]
    fn get_all_issues() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue_db = ClassIssueRequester::new(
            CLASS_REPO_ADDRESS.to_string(),
            USERNAME.to_string(),
            password.to_string(),
        );
        let body = issue_db
            .get_all_issues("test".to_string())
            .expect("error closing");
        let deser: Vec<serde_json::Value> = serde_json::from_str(&body).expect("error parsinge");
        for x in deser {
            //dbg!(&x);
            let title: String = serde_json::from_value(x["title"].clone()).expect("err");
            let number: u32 = serde_json::from_value(x["number"].clone()).expect("err");
            let issue_body: String = serde_json::from_value(x["body"].clone()).expect("err");
            dbg!(title);
            dbg!(number);
            //dbg!(issue_body);
            issue_db.close_issue(number);
        }
    }
    #[test]
    fn get_all_my_issues() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue_db = ClassIssueRequester::new(
            CLASS_REPO_ADDRESS.to_string(),
            USERNAME.to_string(),
            password.to_string(),
        );
        let body = issue_db
            .get_all_my_issues("test".to_string())
            .expect("error closing");
        let deser: Vec<serde_json::Value> = serde_json::from_str(&body).expect("error parsinge");
        for x in deser {
            //dbg!(&x);
            let title: String = serde_json::from_value(x["title"].clone()).expect("err");
            let number: u32 = serde_json::from_value(x["number"].clone()).expect("err");
            let issue_body: String = serde_json::from_value(x["body"].clone()).expect("err");
            dbg!(title);
            dbg!(number);
            //dbg!(issue_body);
            issue_db.close_issue(number);
        }
    }

}
