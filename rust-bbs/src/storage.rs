use std::fs::OpenOptions;
use std::io::Read;
use crate::models::Post;

const FILE_PATH: &str = "posts.json";

/// 全投稿を読み込む
pub fn load_posts() -> Vec<Post> {
    let mut file = match OpenOptions::new().read(true).open(FILE_PATH) {
        Ok(f) => f,
        Err(_) => return vec![], // ファイルがなければ空配列を返す
    };
    let mut content = String::new();
    if file.read_to_string(&mut content).is_ok() {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        vec![]
    }
}

/// 全投稿を書き出す
pub fn save_all(posts: &[Post]) {
    if let Ok(json) = serde_json::to_string_pretty(posts) {
        let _ = std::fs::write(FILE_PATH, json);
    }
}

/// 投稿を新規追加
pub fn save_post(post: Post) {
    let mut posts = load_posts();
    posts.push(post);
    save_all(&posts);
}

/// 投稿を編集（idで探して更新）
pub fn update_post(id: usize, name: String, message: String) {
    let mut posts = load_posts();
    if let Some(p) = posts.iter_mut().find(|p| p.id == id) {
        p.name = name;
        p.message = message;
    }
    save_all(&posts);
}

/// 投稿を削除（idで除外）
pub fn delete_post(id: usize) {
    let mut posts = load_posts();
    posts.retain(|p| p.id != id);
    save_all(&posts);
}
