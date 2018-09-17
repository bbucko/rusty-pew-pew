use game::Rect;

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect { x, y, width, height }
    }

    pub fn has_intersection(&self, other: &Rect) -> bool {
        /* Horizontal intersection */
        let mut a_min = self.x;
        let mut a_max = a_min + self.width as i32;
        let mut b_min = other.x;
        let mut b_max = b_min + other.width as i32;

        if b_min > a_min {
            a_min = b_min;
        }
        if b_max < a_max {
            a_max = b_max;
        }

        if a_max <= a_min {
            return false;
        }

        /* Vertical intersection */
        a_min = self.y;
        a_max = a_min + self.height as i32;
        b_min = other.y;
        b_max = b_min + other.height as i32;
        if b_min > a_min {
            a_min = b_min;
        }
        if b_max < a_max {
            a_max = b_max;
        }

        if a_max <= a_min {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use game::Rect;

    #[test]
    fn test_intersection() {
        //given
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(5, 5, 10, 10);

        //when
        let intersecting = a.has_intersection(&b);

        //then
        assert!(intersecting);
    }

    #[test]
    fn test_no_intersection() {
        //given
        let a = Rect::new(0, 0, 1, 1);
        let b = Rect::new(5, 5, 1, 1);

        //when
        let intersecting = a.has_intersection(&b);

        //then
        assert!(!intersecting);
    }

    #[test]
    fn test_border_intersection() {
        //given
        let a = Rect::new(0, 0, 5, 5);
        let b = Rect::new(5, 5, 5, 5);

        //when
        let intersecting = a.has_intersection(&b);

        //then
        assert!(!intersecting);
    }
}
