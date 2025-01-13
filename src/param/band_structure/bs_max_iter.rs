use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};
/// This keyword controls the maximum number of iterations to perform when
/// calculating band structure.
/// # Note
/// It might be necessary to increase this value if a low BS_MAX_CG_STEPS is used.
/// # Default
/// 60
/// # Example
/// `BS_MAX_ITER : 50`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, KeywordDisplay,
)]
#[keyword_display(field="BS_MAX_ITER", from=u32,value=u32, default_value=60)]
pub struct BSMaxIter(u32);

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::BSMaxIter;

    #[test]
    fn bs_max_iter() {
        let m = BSMaxIter::default();
        assert_eq!(60, m.value());
        let m = BSMaxIter::from(120);
        assert_eq!("BS_MAX_ITER : 120", m.output());
    }
}
