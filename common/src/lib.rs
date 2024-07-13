pub mod event;
pub mod simple_rule;
pub mod compound_rule;
pub mod rule_result;

pub trait Advancing<T, U> {
    fn advance(&mut self, data: &T) -> AdvancingResult<U> {
        let adv_res = self.raw_advance(data);
        if let AdvancingResultType::Completed = adv_res.res_type {
            if self.needs_reset() {
                self.reset();
                AdvancingResult::restarted(adv_res.data)
            } else {
                adv_res
            }
        } else {
            adv_res
        }
    }
    fn raw_advance(&mut self, data: &T) -> AdvancingResult<U>;
    fn reset(&mut self);
    fn needs_reset(&self) -> bool;
}

pub struct AdvancingResult<T> {
    pub res_type: AdvancingResultType,
    pub data: Option<T>
}

impl<T> AdvancingResult<T> {
    fn completed(data: Option<T>) -> Self {
        Self::new(AdvancingResultType::Completed, data)
    }
    fn restarted(data: Option<T>) -> Self {
        Self::new(AdvancingResultType::Restarted, data)
    }
    fn hit(data: Option<T>) -> Self {
        Self::new(AdvancingResultType::Hit, data)
    }
    fn empty() -> Self {
        Self::new(AdvancingResultType::None, None)
    }
    fn new(res_type: AdvancingResultType, data: Option<T>) -> Self {
        Self { res_type, data }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum AdvancingResultType {
    None, Hit, Restarted, Completed
}