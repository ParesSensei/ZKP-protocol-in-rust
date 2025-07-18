use std::{collections::HashMap};
use tonic::{transport::Server, Code, Request, Response, Status, codegen::http::request};
use num_bigint::BigUint;
use std::sync::Mutex;

use zkp_chaum_pedersen::ZKP;

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{auth_server::{Auth, AuthServer}, RegisterRequest, RegisterResponse, AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse};

#[derive(Debug,Default)]
pub struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_id_to_user: Mutex<HashMap<String, String>>,
}

#[derive(Debug,Default)]
pub struct UserInfo {
    // Registration
    pub user_name: String,
    pub y1: BigUint,
    pub y2: BigUint,
    // Authorization
    pub r1: BigUint,
    pub r2: BigUint,
    // Verification
    pub c: BigUint,
    pub s: BigUint,
    pub session_id: String,
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        println!("Processing Register {:?}", request);
        
        let request = request.into_inner();

        let user_name = request.user;

        let mut user_info = UserInfo::default();

        user_info.user_name = user_name.clone();
        user_info.y1 = BigUint::from_bytes_be(&request.y1);
        user_info.y2 = BigUint::from_bytes_be(&request.y2);

        let mut user_info_hashmap = &mut self.user_info.lock().unwrap();
        user_info_hashmap.insert(user_name, user_info);

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challange(&self, request: Request<AuthenticationChallengeRequest>) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Processing Challange Request {:?}", request);
        
        let request = request.into_inner();

        let user_name = request.user;

        let mut user_info_hashmap = &mut self.user_info.lock().unwrap();

        if let Some(user_info) = user_info_hashmap.get_mut(&user_name) {
            let (_, _, _, q) = ZKP::get_constants();
            let c = ZKP::generate_random_number_below(&q);
            let auth_id = ZKP::generate_random_string(12);

            let mut auth_id_to_user = &mut self.auth_id_to_user.lock().unwrap();
            auth_id_to_user.insert(auth_id.clone(), user_name);

            Ok(Response::new(AuthenticationChallengeResponse {auth_id, c: c.to_bytes_be()}))
        } else {
            Err(Status::new(Code::NotFound, format!("User: {} not found in database", user_name)))
        }
    }

    async fn verify_authentication(&self, request: Request<AuthenticationAnswerRequest>) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        println!("Processing Verification Request{:?}", request);
        
        let request = request.into_inner();

        let auth_id = request.auth_id;

        let mut auth_id_to_user_hashmap = &mut self.auth_id_to_user.lock().unwrap();

        if let Some(user_name) = auth_id_to_user_hashmap.get(&auth_id) {
            let mut user_infp_hashmap = &mut self.user_info.lock().unwrap();
            let user_info = user_infp_hashmap.get_mut(user_name).expect("Auth id not found in hashmap");

            let s = BigUint::from_bytes_be(&request.s);

            let (alpha, beta, p, q) = ZKP::get_constants();
            let zkp = ZKP {alpha, beta, p, q};

            let verificaton = zkp.verify(&user_info.r1, &user_info.r2, &user_info.y1, &user_info.y2, &user_info.c, &s);

            if verificaton {
                let session_id = ZKP::generate_random_string(12);
    
                Ok(Response::new(AuthenticationAnswerResponse { session_id}))
            } else {
                Err(Status::new(Code::PermissionDenied, format!("AuthId: {} bad solution to the challange", auth_id)))
            }
        } else {
            Err(Status::new(Code::NotFound, format!("AuthId: {} not found in database", auth_id)))
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:50051".to_string();

    let auth_impl = AuthImpl::default();

    println!{"✅ Running the server in {}", addr};

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr.parse().expect("could not convert address"))
        .await
        .unwrap();
}