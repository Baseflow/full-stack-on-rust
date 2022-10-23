pub trait Repository<T>: Send + Sync {
    /// Returns all availble instances of `<T>`
    fn get_all(&self) -> Vec<T>;

    /// Returns a single instance of `<T>` based on the given id
    ///
    ///  # Arguments
    ///  
    ///  * `id` - The identifier of the item to find in the data store.
    fn get_by_id(&self, id: uuid::Uuid) -> Option<T>;

    /// Inserts a single instance of `<T>` in the data store
    ///
    ///  # Arguments
    ///  
    ///  * `entity` - The entity to insert.
    fn insert(&self, entity: T) -> Result<T, String>;

    /// Updates a single instance of `<T>` in the data store with the given `id`
    ///
    ///  # Arguments
    ///  
    ///  * `id` - The unique identifier of the entity to update
    ///  * `entity` - An updated version of the entity with the latest values.
    fn update(&self, id: uuid::Uuid, entity: T) -> Result<T, String>;

    /// Deletes a single instance of `<T>` from the data store with the given `id`
    ///
    ///  # Arguments
    ///  
    ///  * `id` - The identifier of the item to delete from the data store.
    fn delete(&self, id: uuid::Uuid) -> Result<bool, String>;
}
