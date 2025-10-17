use std::num::Saturating;

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        let mut r_sat = Saturating(result);
        let i_sat = Saturating(i);
        r_sat = r_sat * i_sat;

        result = r_sat.0
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
