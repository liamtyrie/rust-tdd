fn main() {}

fn parse_fields(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_fields() {
        let fields = parse_fields("");
        assert!(fields.is_empty());

        let fields = parse_fields("Tom");
        assert_eq!(fields.len(), 1);
        assert_eq!(fields.first().unwrap(), "Tom");

        let fields = parse_fields("Tom,Dick");
        assert_eq!(fields.len(), 2);
        assert_eq!(fields.first().unwrap(), "Tom");
        assert_eq!(fields.get(1).unwrap(), "Dick");

        let fields = parse_fields("Tom,,Dick");
        assert_eq!(fields.len(), 2);
        assert_eq!(fields.first().unwrap(), "Tom");
        assert_eq!(fields.get(1).unwrap(), "Dick");

        let fields = parse_fields("Tom,Dick,,Harry");
        assert_eq!(fields.len(), 3);
        assert_eq!(fields.first().unwrap(), "Tom");
        assert_eq!(fields.get(1).unwrap(), "Dick");
        assert_eq!(fields.get(2).unwrap(), "Harry");

        let fields = parse_fields(",Tom, Dick,, ,Harry,");
        assert_eq!(fields.len(), 3);
        assert_eq!(fields.first().unwrap(), "Tom");
        assert_eq!(fields.get(1).unwrap(), "Dick");
        assert_eq!(fields.get(2).unwrap(), "Harry");
    }

    #[test]
    fn can_add_values() {
        let result = add(2, 2);
        assert_eq!(result, 4)
    }
}
