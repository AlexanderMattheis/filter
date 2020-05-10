
#[derive(Copy, Clone)]
pub enum BorderHandlingType {
    ConstantValue,
    Unprocessed,
    PaddingConstantValue,
    PaddingExtend,
    PaddingMirror,
    PaddingPeriodically
}