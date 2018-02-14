
// -------------
// error-chain
// -------------

/// Helper for formatting Errors that wrap strings
macro_rules! format_err {
    ($error:expr, $str:expr) => {
        $error(format!($str))
    };
    ($error:expr, $str:expr, $($arg:expr),*) => {
        $error(format!($str, $($arg),*))
    }
}


/// Helper for formatting strings with error-chain's `bail!` macro
macro_rules! bail_fmt {
    ($error:expr, $str:expr) => {
        bail!(format_err!($error, $str))
    };
    ($error:expr, $str:expr, $($arg:expr),*) => {
        bail!(format_err!($error, $str, $($arg),*))
    }
}


// -------------
// rusqlite
// -------------

/// Attempts to execute an `insert`
///
/// Returns a `Result` containing the given model
///
/// # macro syntax
///
/// ```rust,ignore
/// try_insert_to_model!(
///     query-expr-to-execute ;
/// )
/// ```
///
/// # Example
///
/// ```rust,ignore
/// impl NewPaste {
///     fn insert(self, conn: &Connection) -> Result<()> {
///         let stmt = "insert into pastes (key, content, content_type, date_created, date_viewed) values (?, ?, ?, ?, ?)";
///         let now = Dt::now();
///         try_insert!(conn, stmt, &[&self.key, &self.content, &self.content_type, &now, &now])
///     }
/// }
/// ```
macro_rules! try_insert {
    ($conn:expr, $stmt:ident, $params:expr) => {
        {
            let mut stmt = $conn.prepare($stmt)?;
            let row_id = stmt.insert($params)?;
            row_id
        }
    }
}


