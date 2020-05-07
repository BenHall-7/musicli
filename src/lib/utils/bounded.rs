pub trait Bounded<T>
where
    T: Copy + PartialOrd,
{
    const MIN: T;
    const MAX: T;

    fn bounded(value: T) -> T {
        if value > Self::MAX {
            Self::MAX
        } else if value < Self::MIN {
            Self::MIN
        } else {
            value
        }
    }

    fn is_in_range(value: T) -> bool {
        value <= Self::MAX || value >= Self::MIN
    }
}
