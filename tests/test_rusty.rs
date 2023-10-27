extern crate rusty;

#[cfg(test)]
mod tests {
    use rusty::bed::reader::{BedReader};

    #[test]
    fn test_true() {
        assert!(true);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(rusty::myrustmodule::factorial(5), 120);
    }

    #[test]
    fn test_bed_reader_valid() {
        let bed_reader: BedReader = BedReader::new(Some("tests/data/sample.bed".to_string()));
        assert!(format!("{:?}", bed_reader)=="BedReader @ tests/data/sample.bed");
        assert_eq!(bed_reader.length(), 9);
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn test_bed_reader_invalid() {
        let bed_reader: BedReader = BedReader::new(Some("/no/such/file".to_string()));
    }

    #[test]
    fn test_bed_reader_default() {
        let bed_reader: BedReader = BedReader::new(None);
        assert!(format!("{:?}", bed_reader)=="BedReader @ tests/data/sample.bed");
        assert_eq!(bed_reader.length(), 9);
    }

}
