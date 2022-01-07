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
