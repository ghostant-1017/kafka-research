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

use rand::RngCore;
use crate::SOLUTION_TOPIC_PREFIX;

pub trait Request {
    fn to_topic(&self) -> String;
    fn to_data(&self) -> String;
}

pub struct MockRequest {
    epoch: u32,
    data: String,
}

impl MockRequest {
    pub fn random(epoch: u32) -> Self {
        let mut rng = rand::thread_rng();
        let data: String = (0..10)
            .map(|_| rng.next_u32().to_string())
            .collect::<Vec<String>>()
            .join(",");
        Self { epoch, data }
    }
}

impl Request for MockRequest {
    fn to_topic(&self) -> String {
        format!("{}-{}",SOLUTION_TOPIC_PREFIX, self.epoch)
    }

    fn to_data(&self) -> String {
        self.data.clone()
    }
}
