pub struct FileManager;

impl FileManager {
    pub fn init() -> Self {
        Self {};
    }

    pub fn create_folder(&self, path_to_folder: String) {
        todo!();
    }

    pub fn create_file(&self, path_to_file: String) {
        todo!();
    }

    pub fn write_into_file(&self, path_to_file: String) {
        todo!();
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
