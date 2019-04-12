mod requesters;

use requesters::ClassIssueRequester;

#[macro_use]
extern crate serde_json;

pub struct ClassIssue {
    title: String,
    username: String,
    body: String,
    label: String,
    number: u32,
    open: bool
}

//holds data for instructor and students
pub struct ClassIssues {
    requester: ClassIssueRequester,
}

impl ClassIssues {
    pub fn new(class_repo_address: String, username: String, password: String) -> ClassIssues {
        ClassIssues{
            requester: ClassIssueRequester {
                class_repo_address,
                username,
                password,
            }
        }
    }
    //////////////////////////////////////
    //methods for use by the students 
    //////////////////////////////////////
    pub fn register(&self,enc_repo: &str) -> Result<(),()>{
        let body = self.requester
            .add_issue(&self.requester.username,enc_repo,"register")
            .expect("error closing");
        let deser: serde_json::Value = serde_json::from_str(&body).expect("error parsinge");
        dbg!(&deser);
        if deser["state"] == "open"{
            return Err(());
        };
        Ok(())
    }

    pub fn request_grade(&self,enc_repo: &str) -> Result<(),()>{
        let body = self.requester
            .add_issue(&self.requester.username,enc_repo,"grade_request")
            .expect("error closing");
        let deser: serde_json::Value = serde_json::from_str(&body).expect("error parsinge");
        dbg!(&deser);
        if deser["state"] == "open"{
            return Err(());
        };
        Ok(())
    }

    // pub fn view_grades()  -> Result<Vec<ClassIssue>,()>{
    //     Ok(())
    // }

    // //////////////////////////////////////
    // //methods for use by the instructors
    // //////////////////////////////////////
    // pub fn get_all_registrations(&self) -> Result<Vec<ClassIssue>,()>{
   
    //     Ok(())
    // }


    // pub fn get_all_grade_requests(&self) -> Result<Vec<ClassIssue>,()>{
      
    //     Ok(())
    // }


    pub fn confirm_register(&self, registration: &ClassIssue) -> Result<(),()>{
        match self.requester.close_issue(registration.number){
            Err(_) => return Err(()),
            _ => (),
        };
        Ok(())
    }


    pub fn post_grade(&self,request:ClassIssue,enc_feedback:&str) -> Result<(),()>{
        match self.requester.comment_on_issue(enc_feedback, request.number){
            Err(_) => return Err(()),
            _ => (),
        };
        match self.requester.edit_first_comment(enc_feedback){
            Err(_) => return Err(()),
            _ => ()
        };
        match self.requester.close_issue(request.number){
            Err(_) => return Err(()),
            _ => ()
        };
        Ok(())
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    const USERNAME: &str = "hortinstein";
    const CLASS_REPO_ADDRESS: &str = "https://api.github.com/repos/replicatedu/issue_database";

    #[test]
    fn close_issue() {
        
    }
}
