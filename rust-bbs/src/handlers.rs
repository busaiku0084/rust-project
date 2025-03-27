use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect},
};
use askama::Template;
use chrono::Local;
use serde::Deserialize;

use crate::{models::Post, storage};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "edit.html")]
struct EditTemplate {
    post: Post,
}

#[derive(Deserialize)]
pub struct PostForm {
    name: String,
    message: String,
}

// トップページ表示（投稿一覧＋フォーム）
pub async fn show_form() -> impl IntoResponse {
    let mut posts = storage::load_posts();
    posts.sort_by(|a, b| b.id.cmp(&a.id)); // 新しい順
    Html(IndexTemplate { posts }.render().unwrap())
}

// 投稿処理
pub async fn submit_post(Form(form): Form<PostForm>) -> impl IntoResponse {
    let new_id = storage::load_posts().iter().map(|p| p.id).max().unwrap_or(0) + 1;
    let new_post = Post {
        id: new_id,
        name: form.name,
        message: form.message,
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    storage::save_post(new_post);
    Redirect::to("/")
}

// 編集フォーム表示
pub async fn edit_form(Path(id): Path<usize>) -> impl IntoResponse {
    let posts = storage::load_posts();
    if let Some(post) = posts.into_iter().find(|p| p.id == id) {
        Html(EditTemplate { post }.render().unwrap()).into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

// 編集送信処理
pub async fn edit_submit(Path(id): Path<usize>, Form(form): Form<PostForm>) -> impl IntoResponse {
    storage::update_post(id, form.name, form.message);
    Redirect::to("/")
}

// 削除処理
pub async fn delete_post(Path(id): Path<usize>) -> impl IntoResponse {
    storage::delete_post(id);
    Redirect::to("/")
}
