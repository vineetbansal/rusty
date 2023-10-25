extern crate rusty;

#[cfg(test)]
mod tests {
    use rusty::bed::reader::{build_bed_reader, BedReader};

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
        let bed_file = "/path/to/bed/file".to_string();
        let bed_reader: BedReader = build_bed_reader(bed_file);
        assert!(format!("{:?}", bed_reader)=="BedReader @ /path/to/bed/file");
    }

}



