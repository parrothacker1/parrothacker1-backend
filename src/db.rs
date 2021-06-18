#![allow(deprecated)]
use postgres::{Client, Error};
use std::env::var;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use rustc_serialize::json;

#[derive(RustcDecodable,RustcEncodable)]
pub struct Projectprelist {
    name:String,
    link:String,
}
#[derive(RustcDecodable,RustcEncodable)]
pub struct Projectlist {
    projects:Vec<String>,
}
#[derive(RustcDecodable,RustcEncodable)]
pub struct Langs {
    name:Vec<String>,
}
fn con() -> MakeTlsConnector {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    MakeTlsConnector::new(builder.build())
}

pub fn addproject(name:String,link:String) -> Result<bool, Error> {
    let mut client = Client::connect(&var("DATABASE_URL").unwrap(),con())?;
    client.execute("INSERT INTO projects (name, link) VALUES ($1, $2)",&[&name, &link],)?;
    let mut out:bool=false;
    for rows in client.query("SELECT name,link from projects",&[])? {
        let x:String=rows.get(0);
        if name==x {
            out=true;
            break;
        } else {
            continue;
        }
    }
    Ok(out)
}
pub fn delproject(name:String) -> Result<bool,Error> {
    let mut client=Client::connect(&var("DATABASE_URL").unwrap(),con())?;
    client.execute("DELETE FROM projects WHERE name=$1",&[&name],)?;
    let mut out:bool=true;
    for rows in client.query("SELECT name,link from projects",&[])? {
        let x:String=rows.get(0);
        if name==x {
            out=false;
            break;
        } else {
            continue;
        }
    }
    Ok(out)
}
pub fn listprojects() -> Result<String,Error> {
    let mut client=Client::connect(&var("DATABASE_URL").unwrap(),con())?;
    let mut vec:Vec<String>=Vec::new();
    for x in client.query("SELECT name,link from projects",&[])? {
        let x1:String=x.get("name");
        let x2:String=x.get("link");
        let out_pre=json::encode(&Projectprelist {
            name:x1,
            link:x2,
        }).unwrap();
        vec.push(out_pre);
    }
    let out=json::encode(&Projectlist {
        projects:vec,
    }).unwrap();
    Ok(out)
}
pub fn listlangs() -> Result<String,Error> {
    let mut client=Client::connect(&var("DATABASE_URL").unwrap(),con())?;
    let mut vec:Vec<String>=Vec::new();
    for x in client.query("SELECT name from langs",&[])? {
        let str:String=x.get(0);
        vec.push(str);
    }
    let out=json::encode(&Langs {
        name:vec,
    }).unwrap();
    Ok(out)
}
pub fn addlang(name:String,class:String) -> Result<bool,Error> {
    let mut out:bool=false;
    let mut client=Client::connect(&var("DATABASE_URL").unwrap(),con())?;
    client.execute("INSERT INTO langs (name,class) VALUES ($1,$2)",&[&name,&class],)?;
    for rows in client.query("SELECT name from langs",&[])? {
        let x:String=rows.get(0);
        if name==x {
            out=true;
            break;
        } else {
            continue;
        }
    }
    Ok(out)
}
pub fn dellang(name:String) -> Result<bool,Error> {
    let mut out:bool=true;
    let mut client=Client::connect(&var("DATABASE_URL").unwrap(),con())?;
    client.execute("DELETE from langs where name=$1",&[&name],)?;
    for rows in client.query("SELECT name from langs",&[])? {
        let x:String=rows.get(0);
        if name==x {
            out=false;
            break;
        } else {
            continue;
        }
    }
    Ok(out)
}
pub fn htmllang() -> Result<String,Error> {
    let mut client=Client::connect(&var("DATABASE_URL").unwrap(),con())?;
    let mut vec:Vec<String>=Vec::new();
    for x in client.query("SELECT class from langs",&[])? {
        let str:String=x.get(0);
        vec.push(str);
    }
    let out=json::encode(&Langs {
        name:vec,
    }).unwrap();
    Ok(out)
}
