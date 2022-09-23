pub trait Repository<T> {
    fn get_all(&self) -> Vec<T>;
    fn get_by_id(&self, id: i32) -> Option<T>;
    fn insert(&self, entity: T) -> Result<T, String>;
    fn update(&self, id: i32, entity: T) -> Result<T, String>;
    fn delete(&self, id: i32) -> Result<bool, String>;
}
