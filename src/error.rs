use std::fmt;

pub struct SRLError(pub String /* problem-src-procedure */ , pub String /* errordescription */);

impl fmt::Debug for SRLError {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl fmt::Display for SRLError {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl SRLError {
	fn to_string(&self) -> String {
		return self.0.to_string() + ": " + &self.1;
	}
}
