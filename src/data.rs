//
// pub struct RequestBuilder {
//     user_context: Option<UserContext>,
//     solution: Option<Solution>,
//     timestamp: u64,
// }
//
// impl Default for RequestBuilder {
//     fn default() -> Self {
//         Self {
//             user_context: None,
//             solution: None,
//             timestamp: get_current_timestamp()
//         }
//     }
// }
//
// impl RequestBuilder {
//     pub fn solution(&mut self, solution: Solution) {
//         self.solution = Some(solution)
//     }
//
//     pub fn user_context(&mut self, user_context: UserContext) {
//         self.user_context = Some(user_context)
//     }
//
//     pub fn build(self) -> Request {
//         Request {
//             user_context: self.user_context.unwrap(),
//             solution: self.solution.unwrap(),
//             timestamp: self.timestamp as i64,
//         }
//     }
// }
//
// pub fn get_current_timestamp() -> u64 {
//     std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_secs()
// }
//
//
// pub struct Request {
//     user_context: UserContext,
//     solution: Solution,
//     timestamp: i64,
// }
//
// pub struct Solution {}
// pub struct UserContext {
//     user_id: i64,
//     miner_account: String,
//     worker_name: String,
// }
//

pub trait Request {
    fn to_topic(&self) -> String;
    fn to_data(&self) -> String;
}

pub struct MockRequest {}

impl Request for MockRequest {
    fn to_topic(&self) -> String {
        "test-topic".to_string()
    }

    fn to_data(&self) -> String {
        "test-data".to_string()
    }
}
