/// Name display
pub(super) trait NameDisplay {
    fn name_display(&self) -> &'static str;
}

impl<T: proc_easy::EasyArgument> NameDisplay for T {
    fn name_display(&self) -> &'static str {
        T::name_display()
    }
}
