
struct Rectangle {
    length: u32,
    breadth: u32
}

impl Rectangle {
    fn can_hold(&self, target: &Rectangle) -> bool {
        self.length > target.length && self.breadth > target.breadth
    }
}


fn add(v1: usize, v2: usize) -> usize {
    v1 + v2
}

fn add_2(v1: usize, v2: usize) -> Result<usize, String> {
    match v1 == v2 {
        true => Ok(v1 + v2),
        false => Err("Invalid values".to_string())
    }
}

fn validate_text(text: &str) -> bool {
    text.len() == 10
}

fn terribly_wrong(flag: bool) {
    match flag {
        true => panic!("things went right!"),
        false => panic!("things went left")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_fn(){
        let result = add(22, 22);
        assert_eq!(result, 44);
    }
    
    #[test]
    #[ignore]
    fn test_add_fn_2() -> Result<(), String> {
        let result = add_2(33, 33)?;
        
        match result == 66 {
            true => Ok(()),
            false => Err("wrong value".to_string())
        }
    }
    
    #[test]
    fn test_validate_text(){
        assert!(validate_text("orangutans"))
    }
    
    #[test]
    fn test_rect_can_hold(){
        let big_rect = Rectangle { length: 50, breadth: 20 };
        let small_rect = Rectangle { length: 20, breadth: 12 };
        
        assert!(big_rect.can_hold(&small_rect), "rectangle cannot hold target");
    }
    
    #[test]
    #[should_panic(expected = "right")]
    fn test_terribly_wrong(){
        terribly_wrong(true);
    }
}