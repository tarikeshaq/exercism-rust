use std::ops::Add;
pub struct Triangle<T> {
    sides: [T; 3]
}

impl<T: Add<Output = T> + PartialOrd<T> + Clone + Copy + From<i32>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        match Triangle::can_build(sides) {
            true => Some(Triangle{sides: sides}),
            false => None
        }
    }

    fn can_build(sides: [T; 3]) -> bool {
        let (side1, side2, side3) = Triangle::get_sides(sides);
        let zero = T::from(0);
        if side1 <= zero || side2 <= zero || side3 <= zero {
            false
        } else {
            (side1 + side2) >= side3 &&
            (side1 + side3) >= side2 &&
            (side2 + side3) >= side1
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let side = self.sides[0];
        self.sides.iter().all(|&val| val == side)
    }

    pub fn is_scalene(&self) -> bool {
        let (side1, side2, side3) = Triangle::get_sides(self.sides);
        side1 != side2 && side2 != side3 && side1 != side3
    }

    fn get_sides(sides: [T; 3]) -> (T, T, T) {
        let side1 = sides[0];
        let side2 = sides[1];
        let side3 = sides[2];
        (side1, side2, side3)
    }

    pub fn is_isosceles(&self) -> bool {
        let (side1, side2, side3) = Triangle::get_sides(self.sides);
        (side1.eq(&side2) || side1.eq(&side3) || side2.eq(&side3)) && !self.is_equilateral()
    }
}
