pub type Range<'r> = (&'r str, &'r str);

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

impl<'r> IsValid for Range<'r> {
    fn is_valid(&self) -> bool {
        let (start, end) = &self;
        !start.is_empty() && !end.is_empty()

    }
}
