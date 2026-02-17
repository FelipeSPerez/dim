#[macro_export]
macro_rules! opt_update {
    ($conn:ident, $query:expr => ($self:expr, $constraint:expr)) => {
        {
            if let Some(x) = $self.as_ref() {
                ::sqlx::query!($query, x, $constraint)
                    .execute(&mut *$conn)
                    .await?;
            }
        }
    };
    ($conn:ident, $query:expr => ($self:expr, $constraint:expr), $($tail:tt)+) => {
        {
            crate::opt_update!($conn, $query => ($self, $constraint));
            crate::opt_update!($conn, $($tail)*);
        }
    }
}
