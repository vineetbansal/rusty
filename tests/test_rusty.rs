extern crate rusty;

#[cfg(test)]
mod tests {
    use rusty::music::library::{build_library, Library};

    #[test]
    fn test_true() {
        assert!(true);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(rusty::myrustmodule::factorial(5), 120);
    }

    #[test]
    fn test_music_library() {
        let root_folder = "/path/to/folder".to_string();
        let library: Library = build_library(root_folder);
        assert!(format!("{:?}", library)=="Library @ /path/to/folder");
    }

}



