use std::collections::LinkedList;
use actix_web::{get, HttpResponse};
use actix_web::dev::Response;
use actix_web::web::Path;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Follower {
    pub id: String,
    pub username: String,
    pub email: String,
    pub name: String
}

impl Follower {
    pub fn new(id: String, username: String, email: String, name: String) -> Self {
        Self {
            id,
            username,
            email,
            name
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: String,
    pub username: String,
    pub email: String,
    pub followers: Vec<Follower>,
    pub follows: Vec<Follower>,
    pub name: String
}

impl Person {
    pub fn new(
        id: String,
        username: String,
        email: String,
        followers: Vec<Follower>,
        follows: Vec<Follower>,
        name: String
    ) -> Self {
        Self {
            id,
            username,
            email,
            followers,
            follows,
            name
        }
    }
}

fn db_persons() -> Vec<Person> {
    let follower: Follower = Follower::new(
        "b55163d0-7dc6-4032-989d-e21788951986".to_owned(),
        "grr".to_owned(),
        "grr.omg@gmail.com".to_owned(),
        "omg".to_owned()
    );

    let follower2: Follower = Follower::new(
        "1296c0a9-42c5-4f7c-bdf3-1c889566f551".to_owned(),
        "woowwowo".to_owned(),
        "lol@gmail.com".to_owned(),
        "woow".to_owned()
    );

    vec![
        Person::new(
            "b55163d0-7dc6-4032-989d-e21788951986".to_owned(),
            "grr".to_owned(),
            "grr.omg@gmail.com".to_owned(),
            vec![follower],
            vec![follower2],
            "omg".to_owned()
        ),
        Person::new(
            "1296c0a9-42c5-4f7c-bdf3-1c889566f551".to_owned(),
            "woowwowo".to_owned(),
            "lol@gmail.com".to_owned(),
            vec![],
            vec![],
            "woow".to_owned()
        ),
    ]
}

#[get("/persons")]
pub async fn getAll() -> HttpResponse {
    let persons = db_persons();

    HttpResponse::Ok()
        //.content_type(APPLICATION_JSON)
        .json(persons)
}

#[get("/persons/{id}")]
pub async fn get(path: Path<(String, )>) -> HttpResponse {
    let persons = db_persons();

    let person_option = persons.iter()
        .find(|person| person.id == path.to_owned().0);

    match person_option {
        Some(person) => {
            HttpResponse::Ok()
                //.content_type(APPLICATION_JSON)
                .json(person)
        }
        None => {
            HttpResponse::NoContent()
                .await
                .unwrap()
        }
    }
}