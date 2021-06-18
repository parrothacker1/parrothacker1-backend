#![allow(unused_must_use)]
use actix_web::{web,App,middleware::Logger,Responder,HttpServer,HttpResponse,get};
use env_logger;
use std::env::var;
mod db;

#[get("/projects")]
async fn pro_l() -> impl Responder {
    env_logger::init();
    let out=db::listprojects();
    match out {
        Ok(out) => HttpResponse::Ok().body(out),
        Err(e) => {
            log::error!("Failed to fetch list {}",e);
            HttpResponse::Ok().body("false")
        },
    }
}
#[get("/addproject/{key}/{name}/{link}")]
async fn add_pro(web::Path((key,name,link)): web::Path<(String,String,String)>) -> impl Responder {
    if key==var("TOKEN").unwrap().to_string() {
        env_logger::init();
        let out=db::addproject(name,link);
        match out {
            Ok(out)  => {
                if out {
                    log::info!("Added a new project");
                    HttpResponse::Ok().body("true".to_string())
                } else {
                    log::error!("Failed to add a new project");
                    HttpResponse::Ok().body("false".to_string())
                }
            }
            Err(e) => {
                log::error!("Failed to add project : {}",e);
                HttpResponse::Ok().body("false".to_string())
            },
        }
    } else {
        log::error!("Wrong token");
        HttpResponse::Ok().body("wrong_token".to_string())
    }

}
#[get("/delproject/{key}/{name}")]
async fn del_pro(web::Path((key,name)): web::Path<(String,String)>) -> impl Responder {
    env_logger::init();
    if key==var("TOKEN").unwrap().to_string() {
        let out=db::delproject(name);
        match out {
            Ok(out) => {
                if out {
                    log::info!("Deleted a existing project");
                    HttpResponse::Ok().body("true".to_string())
                } else {
                    log::error!("Failed to delete project");
                    HttpResponse::Ok().body("false".to_string())
                }
            }
            Err(e) => {
                log::error!("Failed to delete project: {}",e);
                HttpResponse::Ok().body("false".to_string())
            }
        }
    } else {
        log::error!("Wrong token");
        HttpResponse::Ok().body("wrong_token".to_string())
    }
}
#[get("/langs/{outp}")]
async fn lang_l(web::Path(outp): web::Path<String>) -> impl Responder {
    env_logger::init();
    if outp != "html".to_string() {
        let out=db::listlangs();
        match out {
            Ok(out) => HttpResponse::Ok().body(out),
            Err(e) => {
                log::error!("Failed to access languages : {}",e);
                HttpResponse::Ok().body("false".to_string())
            },
        }
    } else {
        let out=db::htmllang();
        match out {                                                                                         Ok(out) => HttpResponse::Ok().body(out),
            Err(e) => {                                                                                         log::error!("Failed to access languages : {}",e);
                HttpResponse::Ok().body("false".to_string())                                                },
        }
    }
}
#[get("/addlang/{key}/{name}/{class}")]
async fn add_lan(web::Path((key,name,class)): web::Path<(String,String,String)>) -> impl Responder {
    if key==var("TOKEN").unwrap().to_string() {
        let out=db::addlang(name,class);
        match out {
            Ok(out) => {
                if out {
                    log::info!("Added a language");
                    HttpResponse::Ok().body("true".to_string())
                } else {
                    log::error!("Faiked to add a new language");
                    HttpResponse::Ok().body("false".to_string())
                }
            },
            Err(e) => {
                log::error!("Failed to add a new language: {}",e);
                HttpResponse::Ok().body("false".to_string())
            },
        }
    } else {
        log::error!("Wrong token");
        HttpResponse::Ok().body("wrong_token".to_string())
    }
}
#[get("/")]
async fn indx() -> impl Responder {
    HttpResponse::Ok().body("A backend server for ph1 website and bot".to_string())
}
#[get("/dellang/{key}/{name}")]
async fn del_lan(web::Path((key,name)): web::Path<(String,String)>) -> impl Responder {
    if key==var("TOKEN").unwrap().to_string() {
        let out=db::dellang(name);
        match out {
            Ok(out) => {
                if out {
                    log::info!("Deleted a exisiting language");
                    HttpResponse::Ok().body("true".to_string())
                } else {
                    log::error!("Failed to delete language");
                    HttpResponse::Ok().body("false".to_string())
                }
            },
            Err(e) => {
                log::error!("Failed to delete language: {}",e);
                HttpResponse::Ok().body("false".to_string())
            },
        }
    } else {
        HttpResponse::Ok().body("wrong_token".to_string())
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host:String=var("HOST").unwrap().to_string();
    let port:String=var("PORT").unwrap().to_string();
    let add:String=format!("{}:{}",host,port);
    env_logger::init();
    log::info!("Started server at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(indx)
            .service(pro_l)
            .service(del_pro)
            .service(add_pro)
            .service(lang_l)
            .service(add_lan)
            .service(del_lan)
            .wrap(Logger::new("%a   %r Status Code:%s"))
    })
    .bind(add)?
    .run()
    .await
}
