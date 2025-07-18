use std::io::stdin;
use num_bigint::BigUint;

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{auth_client::AuthClient, RegisterRequest};
use zkp_chaum_pedersen::ZKP;

use crate::zkp_auth::{AuthenticationAnswerRequest, AuthenticationChallengeRequest};

#[tokio::main]
async fn main() {
    let (alpha, beta, p, q) = ZKP::get_constants();
    let zkp = ZKP {alpha: alpha.clone(), beta: beta.clone(), p: p.clone(), q: q.clone()};
    let mut buf = String::new();

    let mut client = AuthClient::connect("http://127.0.0.1:50051").await.expect("Could not connet to the server");
    println!{"✅ Connected to the server"};

    println!("Please provide the username");
    stdin().read_line(&mut buf).expect("could not get the username from stdin");
    let username = buf.trim().to_string();
    buf.clear();
    
    println!("Please provide the password");
    stdin().read_line(&mut buf).expect("could not get the username from stdin");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    let y1 = ZKP::exponentiate(&alpha, &password, &p);
    let y2 = ZKP::exponentiate(&beta, &password, &p);

    let request = RegisterRequest{
        user: username.clone(),
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };

    let _response = client.register(request).await.expect("Could not register in server");
    print!("{:?}", _response );

    println!("Please provide the password (to login)");
    stdin().read_line(&mut buf).expect("could not get the username from stdin");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    let k = ZKP::generate_random_number_below(&q);
    let r1 = ZKP::exponentiate(&alpha, &k, &p);
    let r2 = ZKP::exponentiate(&beta, &k, &p);

    let request = AuthenticationChallengeRequest{
        user: username,
        r1: r1.to_bytes_be(),
        r2: r2.to_bytes_be(),
    };

    let response = client.create_authentication_challange(request).await.expect("Could not request challenge to server").into_inner();
    print!("{:?}", response );

    let auth_id = response.auth_id;
    let c = BigUint::from_bytes_be(&response.c);

    let s = zkp.solve(&k, &c, &password);

    let request = AuthenticationAnswerRequest{
        auth_id,
        s: s.to_bytes_be(),
    };
    
    let response = client.verify_authentication(request).await.expect("Could not verify authentivtiom in server").into_inner();

    println!("You logged in !! session_id: {}", response.session_id);
}