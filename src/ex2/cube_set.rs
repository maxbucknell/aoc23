#[derive(PartialEq, Debug)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32
}

impl CubeSet {
    pub fn with_rgb(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn with_zero() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0
        }
    }

    pub fn contains(&self, set: &CubeSet) -> bool {
        self.red >= set.red && self.green >= set.green && self.blue >= set.blue
    }

    pub fn is_contained_by(&self, set: &CubeSet) -> bool {
        set.contains(self)
    }

    pub fn power(&self) -> u64 {
        u64::from(self.red) * u64::from(self.green) * u64::from(self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let zero = CubeSet::with_zero();
        let unit = CubeSet::with_rgb(1, 1, 1);

        // assert!(zero.is_contained_by(&zero));
        //
        // assert!(zero.is_contained_by(&unit));
        assert!(zero.contains(&zero));
        assert!(unit.contains(&zero));
        assert!(unit.contains(&unit));
    }

    #[test]
    fn test_is_contained_by() {
        let zero = CubeSet::with_zero();
        let unit = CubeSet::with_rgb(1, 1, 1);

        let red = CubeSet::with_rgb(1, 0, 0);
        let green = CubeSet::with_rgb(0, 1, 0);
        let blue = CubeSet::with_rgb(0, 0, 1);

        assert!(zero.is_contained_by(&zero));
        assert!(zero.is_contained_by(&unit));
        assert!(unit.is_contained_by(&unit));

        assert!(red.is_contained_by(&unit));
        assert!(green.is_contained_by(&unit));
        assert!(blue.is_contained_by(&unit));
        assert!(!red.is_contained_by(&zero));
        assert!(!green.is_contained_by(&zero));
        assert!(!blue.is_contained_by(&zero));
    }

    #[test]
    fn test_power() {
        let set = CubeSet::with_rgb(4, 2, 6);

        assert_eq!(set.power(), 48);

        let set = CubeSet::with_rgb(1, 3, 4);

        assert_eq!(set.power(), 12);

        let set = CubeSet::with_rgb(20, 13, 6);

        assert_eq!(set.power(), 1560);

        let set = CubeSet::with_rgb(14, 3, 15);

        assert_eq!(set.power(), 630);

        let set = CubeSet::with_rgb(6, 3, 2);

        assert_eq!(set.power(), 36);

    }
}
