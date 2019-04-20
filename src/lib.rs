mod requesters;

use requesters::ClassIssueRequester;

#[macro_use]
extern crate serde_json;

#[derive(Debug)]
pub struct ClassIssue {
    pub title: String,
    pub body: String,
    pub number: u32,
    pub id: u32,
    pub open: bool,
}

//holds data for instructor and students
pub struct ClassIssues {
    requester: ClassIssueRequester,
}

fn parse_to_vector(deser: Vec<serde_json::Value>) -> Result<Vec<ClassIssue>, ()> {
    let mut v_ret: Vec<ClassIssue> = Vec::new();
    for x in deser {
        let issue_state: String = serde_json::from_value(x["state"].clone()).expect("err");
        let state = if &issue_state == "open" { true } else { false };
        dbg!(issue_state);
        let ci = ClassIssue {
            title: serde_json::from_value(x["title"].clone()).expect("err"),
            body: serde_json::from_value(x["body"].clone()).expect("err"),
            number: serde_json::from_value(x["number"].clone()).expect("err"),
            id: serde_json::from_value(x["id"].clone()).expect("err"),
            open: state,
        };
        v_ret.push(ci);
    }
    Ok(v_ret)
}

impl ClassIssues {
    pub fn new(class_repo_address: String, username: String, password: String) -> ClassIssues {
        ClassIssues {
            requester: ClassIssueRequester {
                class_repo_address,
                username,
                password,
            },
        }
    }

    //////////////////////////////////////
    //methods for use by the students
    //////////////////////////////////////
    pub fn register(&self, enc_repo: &str) -> Result<(), ()> {
        let body = self
            .requester
            .add_issue(&self.requester.username, enc_repo, "register")
            .expect("error closing");
        let deser: serde_json::Value = serde_json::from_str(&body).expect("error parsinge");
        if deser["state"] == "open" {
            return Err(());
        };
        Ok(())
    }

    pub fn request_grade(&self, enc_repo: &str) -> Result<(), ()> {
        let body = self
            .requester
            .add_issue(&self.requester.username, enc_repo, "grade_request")
            .expect("error closing");
        let deser: serde_json::Value = serde_json::from_str(&body).expect("error parsinge");
        if deser["state"] == "open" {
            return Err(());
        };
        Ok(())
    }

    pub fn view_grades(&self) -> Result<Vec<ClassIssue>, ()> {
        let body = self
            .requester
            .get_all_my_issues("grade_request".to_string())
            .expect("error closing");
        let deser: Vec<serde_json::Value> = serde_json::from_str(&body).expect("error parsinge");
        parse_to_vector(deser)
    }

    pub fn view_registrations(&self) -> Result<Vec<ClassIssue>, ()> {
        let body = self
            .requester
            .get_all_my_issues("register".to_string())
            .expect("error closing");
        let deser: Vec<serde_json::Value> = serde_json::from_str(&body).expect("error parsinge");
        parse_to_vector(deser)
    }

    // //////////////////////////////////////
    // //methods for use by the instructors
    // //////////////////////////////////////
    pub fn get_all_registrations(&self) -> Result<Vec<ClassIssue>, ()> {
        let body = self
            .requester
            .get_all_issues("register".to_string())
            .expect("error closing");
        let deser: Vec<serde_json::Value> = serde_json::from_str(&body).expect("error parsinge");
        parse_to_vector(deser)
    }

    pub fn get_open_registrations(&self) -> Result<Vec<ClassIssue>, ()> {
        let body = self
            .requester
            .get_open_issues("register".to_string())
            .expect("error closing");
        let deser: Vec<serde_json::Value> = serde_json::from_str(&body).expect("error parsinge");
        parse_to_vector(deser)
    }

    pub fn get_open_grade_requests(&self) -> Result<Vec<ClassIssue>, ()> {
        let body = self
            .requester
            .get_open_issues("grade_request".to_string())
            .expect("error closing");
        dbg!(&body);
        let deser: Vec<serde_json::Value> = serde_json::from_str(&body).expect("error parsinge");
        dbg!(&deser);
        parse_to_vector(deser)
    }

    pub fn confirm_register(&self, registration: &ClassIssue, enc_confirm: &str) -> Result<(), ()> {
        match self
            .requester
            .comment_on_issue(enc_confirm, registration.number)
        {
            Err(_) => return Err(()),
            _ => (),
        };

        match self.requester.close_issue(registration.number) {
            Err(_) => return Err(()),
            _ => (),
        };
        Ok(())
    }

    pub fn post_grade(&self, request: ClassIssue, enc_feedback: &str) -> Result<(), ()> {
        match self
            .requester
            .comment_on_issue(enc_feedback, request.number)
        {
            Err(_) => return Err(()),
            _ => (),
        };

        let body = self
            .requester
            .get_issue_comments(request.number)
            .expect("error closing");
        let deser: Vec<serde_json::Value> = serde_json::from_str(&body).expect("error parsinge");
        let first = &deser[0];
        let first_id:u32 =  serde_json::from_value(first["number"].clone()).expect("err");
        match self.requester.edit_comment(enc_feedback, request.id) {
             Err(_) => return Err(()),
             _ => Ok(()),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const USERNAME: &str = "hortinstein";
    const CLASS_REPO_ADDRESS: &str = "https://api.github.com/repos/replicatedu/issue_database";
    use std::env;

    #[test]
    fn register() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue = ClassIssues::new(CLASS_REPO_ADDRESS.to_string(),
                                     USERNAME.to_string(),
                                     password.to_string());
        issue.register("myrepo");
    }
    #[test]
    fn view_register() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue = ClassIssues::new(CLASS_REPO_ADDRESS.to_string(),
                                     USERNAME.to_string(),
                                     password.to_string());
        let regs = issue.view_registrations();
        dbg!(regs);
    }
    #[test]
    fn view_req() {
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue = ClassIssues::new(CLASS_REPO_ADDRESS.to_string(),
                                     USERNAME.to_string(),
                                     password.to_string());
        let regs = issue.view_registrations();
        dbg!(regs);
    }

    #[test]
    fn req_grade(){
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let issue = ClassIssues::new(CLASS_REPO_ADDRESS.to_string(),
                                     USERNAME.to_string(),
                                     password.to_string());
        issue.request_grade("asdfasdfdsa");
        let grade_reqs = issue.get_open_grade_requests().expect("error getting grades");
        for grade_req in grade_reqs{

            let regs = issue.post_grade(grade_req, "my grade_correct");
        }
        
    }

}
