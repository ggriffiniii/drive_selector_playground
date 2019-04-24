pub use drive_selector_derive::DriveSelector;

pub trait DriveSelector {
    fn selector() -> String {
        let mut selector = String::new();
        Self::selector_with_ident("", &mut selector);
        selector
    }

    fn selector_with_ident(ident: &str, selector: &mut String);
}

impl DriveSelector for String {
    fn selector_with_ident(ident: &str, selector: &mut String) {
        match selector.chars().last() {
            Some(',') | None => {}
            _ => selector.push_str(","),
        }
        selector.push_str(ident);
    }
}

impl DriveSelector for bool {
    fn selector_with_ident(ident: &str, selector: &mut String) {
        match selector.chars().last() {
            Some(',') | None => {}
            _ => selector.push_str(","),
        }
        selector.push_str(ident);
    }
}

impl<T> DriveSelector for Vec<T>
where
    T: DriveSelector,
{
    fn selector_with_ident(ident: &str, selector: &mut String) {
        match selector.chars().last() {
            Some(',') | None => {}
            _ => selector.push_str(","),
        }
        selector.push_str(ident);
        let mut inner_selector = String::new();
        T::selector_with_ident("", &mut inner_selector);
        if !inner_selector.is_empty() {
            selector.push_str("(");
            selector.push_str(&inner_selector);
            selector.push_str(")");
        }
    }
}

impl<T> DriveSelector for Option<T>
where
    T: DriveSelector,
{
    fn selector_with_ident(ident: &str, selector: &mut String) {
        T::selector_with_ident(ident, selector)
    }
}
