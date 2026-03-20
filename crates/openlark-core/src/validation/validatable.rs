/// 可验证特质
pub trait Validatable {
    /// 检查是否为空或空白
    fn is_empty_trimmed(&self) -> bool;
}

impl Validatable for &str {
    fn is_empty_trimmed(&self) -> bool {
        self.trim().is_empty()
    }
}

impl Validatable for String {
    fn is_empty_trimmed(&self) -> bool {
        self.trim().is_empty()
    }
}

impl<T: Validatable> Validatable for &T {
    fn is_empty_trimmed(&self) -> bool {
        (*self).is_empty_trimmed()
    }
}

impl<T> Validatable for Vec<T> {
    fn is_empty_trimmed(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Validatable for &[T] {
    fn is_empty_trimmed(&self) -> bool {
        self.is_empty()
    }
}
