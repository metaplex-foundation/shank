pub trait ByteSize {
    fn byte_size(&self) -> usize;
}

pub type OptionByteSize = Option<usize>;
pub trait MaybeByteSize {
    fn maybe_byte_size(&self) -> OptionByteSize;
}
