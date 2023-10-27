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
    fn test_bed_reader() {
        let bed_file = "tests/sample.bed".to_string();
        let bed_reader: BedReader = BedReader::new(bed_file);
        assert!(format!("{:?}", bed_reader)=="BedReader @ tests/sample.bed");
        assert_eq!(bed_reader.length(), 9);
    }
}
