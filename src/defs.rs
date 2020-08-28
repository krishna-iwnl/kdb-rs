pub const MAX_ANDS: i8 = 20;
pub const MAX_ORS: i8 = 20;
pub const PAGE_SIZE: usize = 131072;

pub enum Target {
	Left,
	Right,
	Literal,
}

pub enum CompOperator {
	LessThan,
	GreaterThan,
	Equals,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
	Int,
	Float,
	String,
}

pub trait TypeTrait {
	fn print(&self);
}

pub struct IntType {
	pub value: i32,
}

pub struct FloatType {
	pub value: f64,
}

pub struct StringType {
	pub value: String,
}

impl TypeTrait for IntType {
	fn print(&self) {
		print!("{}", self.value);
	}
}
impl TypeTrait for FloatType {
	fn print(&self) {
		print!("{}", self.value);
	}
}
impl TypeTrait for StringType {
	fn print(&self) {
		print!("{}", self.value);
	}
}
