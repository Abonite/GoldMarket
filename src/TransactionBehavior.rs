// TODO: 这些行为是否足够描述一个市场？

pub enum Behavior {
    Buy (u64),
    Sell(u64),
    Noop
}