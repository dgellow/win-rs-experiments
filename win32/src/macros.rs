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

#[macro_export]
macro_rules! impl_ops_for_all {
    ($($t:ty),+) => {
        $(impl std::ops::BitOr for $t {
			type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        })*
    }
}

// Go-like defer
//
// Source: https://stackoverflow.com/a/29963675/709884
//
// Usage:
// fn main() {
// 	let x = 42u8;
// 	defer!(println!("defer 1"));
// 	defer! {
// 		println!("defer 2");
// 		println!("inside defer {}", x)
// 	}
// 	println!("normal execution {}", x);
// }

pub struct ScopeCall<F: FnOnce()> {
	c: Option<F>,
}
impl<F: FnOnce()> Drop for ScopeCall<F> {
	fn drop(&mut self) {
		self.c.take().unwrap()()
	}
}

#[macro_export]
macro_rules! expr {
	($e: expr) => {
		$e
	};
} // tt hack

#[macro_export]
macro_rules! defer {
    ($($data: tt)*) => (
        let _scope_call = gui::macros::ScopeCall {
            c: Some(|| -> () { gui::expr!({ $($data)* }) })
        };
    )
}
