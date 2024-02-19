pub struct LogWriter<T> {
    pub _type: String,
    pub fs_manager: T,
    pub path_to_file: String,
}

impl<T> LogWriter<T> {
    pub fn init(_type: String, fs_manager: T, path_to_file: String) -> Self {
        let fs_manager_i = fs_manager::init(path_to_file);
        Self { _type, fs_manager: fs_manager_i, path_to_file}
    }

    pub fn log(&self, data: String) {
        self.fs_manager.write_log(data)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
