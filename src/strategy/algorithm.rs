use crate::database::Column;

pub trait Momentum {
    fn get_momentum(&self, skip: usize, time: usize) -> Option<f64>;
}

impl Momentum for &[Column] {
    fn get_momentum(&self, skip: usize, time: usize) -> Option<f64> {
        if self.len() <= time {
            return None
        }

        let ps = f64::from(self[skip].close_price);
        let pst = f64::from(self[time].close_price);

        Some(ps / pst - 1.0)
    }
}
