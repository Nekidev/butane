use std::env::args;

use butane::prelude::*;
use butane::query;
use getting_started::models::Post;
use getting_started::*;

fn main() {
    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{target}%");

    let conn = establish_connection();
    let cnt = query!(Post, title.like({ pattern }))
        .delete(&conn)
        .expect("error deleting posts");
    println!("Deleted {cnt} posts");
}
