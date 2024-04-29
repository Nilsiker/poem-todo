use poem::web::Path;
use poem::Result;
use poem::{error::InternalServerError, web::Data};
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Object)]
pub struct Todo {
    id: Uuid,
    description: String,
    done: bool,
}

pub struct TodosApi;
#[OpenApi]
impl TodosApi {
    #[oai(path = "/todos", method = "get")]
    pub async fn get_all(&self, Data(pool): Data<&PgPool>) -> poem::Result<Json<Vec<Todo>>> {
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
            .fetch_all(pool)
            .await
            .unwrap();
        Ok(Json(todos))
    }

    #[oai(path = "/todos/:id", method = "get")]
    pub async fn get_by_id(
        &self,
        Path(id): Path<Uuid>,
        Data(pool): Data<&PgPool>,
    ) -> poem::Result<Json<Vec<Todo>>> {
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1 LIMIT 1", id)
            .fetch_all(pool)
            .await
            .unwrap();
        Ok(Json(todos))
    }

    #[oai(path = "/todos", method = "post")]
    pub async fn create(
        &self,
        description: String,
        Data(pool): Data<&PgPool>,
    ) -> Result<Json<i64>> {
        let uuid = Uuid::new_v4();

        sqlx::query_as!(
            Todo,
            "INSERT INTO todos VALUES ($1, $2, false)",
            uuid,
            description,
        )
        .execute(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(0))
    }

    #[oai(path = "/todos/:id", method = "delete")]
    pub async fn delete_by_id(
        &self,
        Path(id): Path<Uuid>,
        Data(pool): Data<&PgPool>,
    ) -> poem::Result<Json<Todo>> {
        let deleted = sqlx::query_as!(Todo, "DELETE FROM todos WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await
            .map_err(InternalServerError)?;
        panic!("haha");
        Ok(Json(deleted))
    }

    #[oai(path = "/todos", method = "delete")]
    pub async fn delete_all(&self, Data(pool): Data<&PgPool>) -> poem::Result<Json<&'static str>> {
        sqlx::query_as!(Todo, "TRUNCATE TABLE todos")
            .execute(pool)
            .await
            .map_err(InternalServerError)?;

        Ok(Json("deleted all rows"))
    }
}
