use std::{error::Error, net::Ipv4Addr};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};

mod my_api {
    use actix_web::{get, post, HttpResponse, Responder};
    use utoipa::ToSchema;

    #[utoipa::path(
        get,
        path = "/",
        responses(
            (status = 200, description = "Hello world!"),
            (status = NOT_FOUND, description = "not found!")
        )
    )]
    #[get("/")]
    pub async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[utoipa::path(
        post,
        path = "/echo",
        responses(
            (status = 200, description = "request body received."),
            (status = "5XX", description = "server error")
        ),
        request_body(content = String, description = "json as string request", content_type = "application/json")
    )]
    #[post("/echo")]
    pub async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    pub async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(paths(my_api::hello, my_api::echo))]
    struct ApiDoc;

    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    HttpServer::new(|| {
        App::new()
            .service(my_api::hello)
            .service(my_api::echo)
            .route("/hey", web::get().to(my_api::manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


mod pet_api {
    use utoipa::ToSchema;

    #[derive(ToSchema)]
    pub struct Pet {
        id: u64,
        name: String,
        age: Option<i32>,
    }

    /// Get pet by id
    ///
    /// Get pet from database by pet id
    #[utoipa::path(
    get,
    path = "/pets/{id}",
    responses(
    (status = 200, description = "Pet found succesfully", body = Pet),
    (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(
    ("id" = u64, Path, description = "Pet database id to get Pet for"),
    )
    )]
    async fn get_pet_by_id(pet_id: u64) -> Pet {
        Pet {
            id: pet_id,
            age: None,
            name: "lightning".to_string(),
        }
    }
}
