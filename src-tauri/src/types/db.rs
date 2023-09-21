use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use tauri::AppHandle;

use crate::{
    config::get_or_create_db_path,
    error::DBError,
    types::recipe::RecipeDetails,
    utils::db::{string_to_vec, vec_to_string},
};

pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn new(app: AppHandle) -> Result<Self, DBError> {
        let db_path = get_or_create_db_path(app)?;
        let manager = SqliteConnectionManager::file(db_path);
        let pool = Pool::new(manager).map_err(|_| DBError::ConnectionPool)?;
        let db = Self { pool };
        db.init_table()?;
        Ok(db)
    }

    fn init_table(&self) -> Result<(), DBError> {
        let conn = self.pool.get().map_err(|_| DBError::ConnectionPool)?;
        conn.execute(
            "
CREATE TABLE IF NOT EXISTS RecipeDetails (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  image         TEXT NOT NULL,
  url           TEXT NOT NULL,
  servings      INTEGER NOT NULL,
  time          INTEGER NOT NULL,
  calories      INTEGER NOT NULL,
  source        TEXT,
  title         TEXT NOT NULL,
  ingredients   TEXT,
  instructions  TEXT,
  date_added    DATETIME DEFAULT CURRENT_TIMESTAMP
);
        ",
            (),
        )?;
        Ok(())
    }

    pub fn add_recipe(
        &self,
        recipe: &RecipeDetails,
        uploaded_image: Option<String>,
    ) -> Result<(), DBError> {
        let ingredients_str = vec_to_string(&recipe.ingredients)?;
        let instructions_str = vec_to_string(&recipe.instructions)?;

        let conn = self.pool.get().map_err(|_| DBError::ConnectionPool)?;

        let recipe_image_to_upload = match &uploaded_image {
            Some(image_path) => image_path.as_str(),
            None => &recipe.image,
        };

        conn.execute(
        "INSERT INTO RecipeDetails (image, url, servings, time, calories, source, title, ingredients, instructions) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![recipe_image_to_upload, &recipe.url, recipe.servings, recipe.time, recipe.calories, &recipe.source, &recipe.title, ingredients_str, instructions_str],
    )?;

        Ok(())
    }

    pub fn get_recipe_by_id(&self, id: &i32) -> Result<Option<RecipeDetails>, DBError> {
        let conn = self.pool.get().map_err(|_| DBError::ConnectionPool)?;

        let mut stmt = conn.prepare("SELECT * FROM RecipeDetails WHERE id = ?1")?;
        let mut rows = stmt.query_map(params![id], |row| {
            let ingredients: Option<String> = row.get(8)?;
            let ingredients =
                string_to_vec(&ingredients).map_err(|_| rusqlite::Error::InvalidQuery)?;
            let instructions: Option<String> = row.get(9)?;
            let instructions =
                string_to_vec(&instructions).map_err(|_| rusqlite::Error::InvalidQuery)?;
            Ok(RecipeDetails {
                id: row.get(0)?,
                image: row.get(1)?,
                url: row.get(2)?,
                servings: row.get(3)?,
                time: row.get(4)?,
                calories: row.get(5)?,
                source: row.get(6)?,
                title: row.get(7)?,
                ingredients,
                instructions,
                date_added: row.get(10)?,
            })
        })?;

        match rows.next() {
            Some(Ok(recipe)) => Ok(Some(recipe)),
            Some(Err(e)) => Err(DBError::Connection(e)),
            None => Ok(None),
        }
    }

    pub fn get_all_recipes(&self) -> Result<Vec<RecipeDetails>, DBError> {
        let conn = self.pool.get().map_err(|_| DBError::ConnectionPool)?;

        let mut stmt = conn.prepare("SELECT * FROM RecipeDetails")?;
        let recipes_iter = stmt.query_map([], |row| {
            let ingredients: Option<String> = row.get(8)?;
            let ingredients =
                string_to_vec(&ingredients).map_err(|_| rusqlite::Error::InvalidQuery)?;
            let instructions: Option<String> = row.get(9)?;
            let instructions =
                string_to_vec(&instructions).map_err(|_| rusqlite::Error::InvalidQuery)?;
            Ok(RecipeDetails {
                id: row.get(0)?,
                image: row.get(1)?,
                url: row.get(2)?,
                servings: row.get(3)?,
                time: row.get(4)?,
                calories: row.get(5)?,
                source: row.get(6)?,
                title: row.get(7)?,
                ingredients,
                instructions,
                date_added: row.get(10)?,
            })
        })?;

        let mut recipes = Vec::new();

        for recipe in recipes_iter {
            let recipe = recipe.map_err(DBError::Connection)?;
            recipes.push(recipe);
        }

        Ok(recipes)
    }

    pub fn update_recipe(&self, recipe: &RecipeDetails) -> Result<(), DBError> {
        let conn = self.pool.get().map_err(|_| DBError::ConnectionPool)?;

        let id = match &recipe.id {
            Some(id) => id,
            None => return Err(DBError::MissingID),
        };

        let recipe_exists = self.does_recipe_exist_by_id(id)?;

        match recipe_exists {
            true => {
                let ingredients_str = vec_to_string(&recipe.ingredients)?;
                let instructions_str = vec_to_string(&recipe.instructions)?;

                conn.execute(
        "UPDATE RecipeDetails SET image = ?1, url = ?2, servings = ?3, time = ?4, calories = ?5, source = ?6, title = ?7, ingredients = ?8, instructions = ?9, date_added = ?10 WHERE id = ?11",
        params![&recipe.image, &recipe.url, &recipe.servings, &recipe.time, &recipe.calories, &recipe.source, &recipe.title, &ingredients_str, &instructions_str, &recipe.date_added, &id],
    )?;

                Ok(())
            }
            false => Err(DBError::NoRecord),
        }
    }

    pub fn delete_recipe_by_id(&self, id: &i32) -> Result<(), DBError> {
        let conn = self.pool.get().map_err(|_| DBError::ConnectionPool)?;

        // Check if the recipe exists
        let recipe_exists = self.does_recipe_exist_by_id(id)?;
        // If it does, delete it
        match recipe_exists {
            true => {
                conn.execute("DELETE FROM RecipeDetails WHERE id = ?1", params![&id])?;

                Ok(())
            }
            false => Err(DBError::NoRecord),
        }
    }
    pub fn does_recipe_exist_by_url(&self, url: &str) -> Result<bool, DBError> {
        let conn = self.pool.get().map_err(|_| DBError::ConnectionPool)?;
        let recipe_exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM RecipeDetails WHERE url = ?1)",
            params![&url],
            |row| row.get(0),
        )?;
        Ok(recipe_exists)
    }

    fn does_recipe_exist_by_id(&self, id: &i32) -> Result<bool, DBError> {
        let conn = self.pool.get().map_err(|_| DBError::ConnectionPool)?;
        let recipe_exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM RecipeDetails WHERE id = ?1)",
            params![&id],
            |row| row.get(0),
        )?;
        Ok(recipe_exists)
    }
}
