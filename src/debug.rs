#[macro_export]
macro_rules! display {
	( $($t:tt)* ) => {
		{
			use std::io::Write;
            let mut out = std::io::stdout();
            write!(out, $($t)* ).unwrap();
			write!(out, "\n").unwrap();
            out.flush().unwrap();
        }
    }
}

#[macro_export]
macro_rules! err_display {
	( $($t:tt)* ) => {
		{
			use std::io::Write;
            let mut err = std::io::stderr();
            write!(err, $($t)* ).unwrap();
			write!(err, "\n").unwrap();
            err.flush().unwrap();
        }
    }
}
