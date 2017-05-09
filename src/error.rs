use std;
use std::fmt::Debug;

pub enum SRLResult<T> {
	Ok(T),
	Err(SRLError)
}

pub struct SRLError {
	filename : &'static str,
	line : u32,
	msg : String,
	outer : Option<Box<SRLError>>
}

impl<T> SRLResult<T> {
	pub fn unwrap(self) -> T {
		match self {
			SRLResult::Ok(t) => t,
			SRLResult::Err(err) => panic!("{:?}", err)
		}
	}

	pub fn is_err(&self) -> bool {
		match self {
			&SRLResult::Ok(_) => false,
			&SRLResult::Err(_) => true
		}
	}

	pub fn is_ok(&self) -> bool {
		match self {
			&SRLResult::Ok(_) => true,
			&SRLResult::Err(_) => false 
		}
	}
}

impl SRLError {
	pub fn new(filename : &'static str, line : u32, msg : String) -> SRLError {
		SRLError { filename, line, msg, outer : None }
	}

	pub fn add_outer(&mut self, outer : SRLError) {
		match &mut self.outer {
			&mut Some(ref mut x) => { x.add_outer(outer); return; },
			&mut None => {}
		};
		self.outer = Some(Box::new(outer));
	}
}

impl Debug for SRLError {
	fn fmt(&self, formatter : &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		match self.msg.is_empty() {
			true => write!(formatter, "\nERR({}:{})", self.filename, self.line)?,
			false => write!(formatter, "\nERR({}:{} \"{}\")", self.filename, self.line, self.msg)?
		}
		match &self.outer {
			&Some(ref x) => {
				x.fmt(formatter)?;
			}, &None => { write!(formatter, "\n")?; }
		}
		Ok(())
	}
}

#[macro_export]
macro_rules! ok {
	($thing:expr) => { SRLResult::Ok($thing) }
}

#[macro_export]
macro_rules! err {
	() =>				{ SRLResult::Err(SRLError::new(file!(), line!(), String::new())) };
	($msg:expr $(, $more:expr)*) =>	{ SRLResult::Err(SRLError::new(file!(), line!(), format!($msg, $($more),*))) }
}

// extract
#[macro_export]
macro_rules! x {
	($a:expr) => { x!($a, "") };
	($a:expr $(, $msg:expr)+) => {
		match $a {
			SRLResult::Ok(x) => { x }
			SRLResult::Err(mut x) => {
				x.add_outer(SRLError::new(file!(), line!(), format!($($msg),*)));
				return SRLResult::Err(x);
			}
		}
	}
}
