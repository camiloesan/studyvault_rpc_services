use crate::dal::data_access;
use mysql::{params, prelude::Queryable};
use uuid::Uuid;

pub struct Post {
    pub post_id: u32,
    pub channel_id: u32,
    pub file_id: String,
    pub file_name: String,
    pub title: String,
    pub description: String,
    pub publish_date: String,
}

pub async fn create_post(
    channel_id: u32,
    file_name: String,
    title: String,
    description: String,
) -> bool {
    let mut conn = data_access::get_connection();

    let uuid = Uuid::new_v4().to_string();

    let first_query = "INSERT INTO files (file_id, name) VALUES (:file_id, :file_name)";
    let f_result = conn
        .exec_iter(
            first_query,
            params! {
                "file_id" => &uuid,
                "file_name" => file_name,
            },
        )
        .expect("Failed to insert file")
        .affected_rows();

    let second_query = "INSERT INTO posts (channel_id, file_id, title, description, publish_date)
        VALUES (:channel_id, :file_id, :title, :description, NOW())";
    let s_result = conn
        .exec_iter(
            second_query,
            params! {
                "channel_id" => channel_id,
                "file_id" => uuid,
                "title" => title,
                "description" => description,
            },
        )
        .expect("Failed to create post")
        .affected_rows();

    f_result == 1 && s_result == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_post() {
        let channel_id = 1;
        let file_name = "test.pdf".to_string();
        let title = "Test Post".to_string();
        let description = "This is a test post".to_string();

        let result = create_post(channel_id, file_name, title, description).await;

        assert_eq!(result, true);
    }
}
