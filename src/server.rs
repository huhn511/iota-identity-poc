use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};

use identity_core::did::{DID};
use identity_vc::prelude::*;
use identity_common::Timestamp;
use identity_common::object;


use actix_web::{
    delete, get, head, middleware, options, patch, post, put, web, App, HttpRequest, HttpResponse,
    HttpServer,
};

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000);

    println!("Runnning HTTP Server on http://{}", addr);
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(handle_ping)
    })
    .bind(&addr)?
    .run()
    .await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to iota-identity-poc!")
}

#[get("/test")]
async fn handle_ping() -> HttpResponse {
    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["alice".into()],
        ..Default::default()
    }
    .init()
    .unwrap();
    println!("did: {}", did.to_string());

    // credential
    let issuance = Timestamp::now();

    // Question: should issuer from type DID?
    let credential = CredentialBuilder::new()
        .issuer(did.to_string())
        .context("https://www.w3.org/2018/credentials/examples/v1")
        .type_("PrescriptionCredential")
        .try_subject(object!(id: "did:iota:alice"))
        .unwrap()
        .issuance_date(issuance)
        .build()
        .unwrap();

    println!("credential: {:?}", credential);
    let verifiable = VerifiableCredential::new(credential, object!());
    println!("verifiable: {:?}", verifiable);

    let presentation = PresentationBuilder::new()
        .context("https://www.w3.org/2018/credentials/examples/v1")
        .id("did:example:id:123")
        .type_("PrescriptionCredential")
        .credential(verifiable.clone())
        .try_refresh_service(object!(id: "", type: "Refresh2020"))
        .unwrap()
        .try_terms_of_use(object!(type: "Policy2019"))
        .unwrap()
        .try_terms_of_use(object!(type: "Policy2020"))
        .unwrap()
        .build()
        .unwrap();

    println!("presentation: {:?}", presentation);


    match presentation.validate() {
        Ok(some) => {
            println!("presentation.validate some: {:?}", some);
        },
        Err(error) => {
            println!("presentation.validate Error: {}", error);
        }
    }

    HttpResponse::Ok().body(did.to_string())
}
