mod d17pt1;
pub use d17pt1::Solution as D17Pt1;
mod d17pt2;
pub use d17pt2::Solution as D17Pt2;
mod d18pt1;
pub use d18pt1::Solution as D18Pt1;
mod d18pt2;
pub use d18pt2::Solution as D18Pt2;
mod d19pt1;
pub use d19pt1::Solution as D19Pt1;
mod d19pt2;
pub use d19pt2::Solution as D19Pt2;
mod d20pt1;
pub use d20pt1::Solution as D20Pt1;
mod d20pt2;
pub use d20pt2::Solution as D20Pt2;
mod d22pt1;
pub use d22pt1::Solution as D22Pt1;

pub fn parsers<'a>() -> Vec<Box<dyn super::InputParser<'a>>>
{
	let days: Vec<Option<Box<dyn Fn(Option<String>) -> Box<dyn super::Solution + 'static>>>> = vec!
	[
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		Some(Box::new(|input: Option<String>| { Box::new(D17Pt1::new(input.expect("empty input received"))) })),
		Some(Box::new(|input: Option<String>| { Box::new(D17Pt2::new(input.expect("empty input received"))) })),
		Some(Box::new(|input: Option<String>| { Box::new(D18Pt1::new(input.expect("empty input received"))) })),
		Some(Box::new(|input: Option<String>| { Box::new(D18Pt2::new(input.expect("empty input received"))) })),
		Some(Box::new(|input: Option<String>| { Box::new(D19Pt1::new(input.expect("empty input received"))) })),
		Some(Box::new(|input: Option<String>| { Box::new(D19Pt2::new(input.expect("empty input received"))) })),
		Some(Box::new(|input: Option<String>| { Box::new(D20Pt1::new(input.expect("empty input received"))) })),
		None,
		None,
		None,
		Some(Box::new(|input: Option<String>| { Box::new(D22Pt1::new(input.expect("empty input received"))) })),
	];
	days.into_iter()
		.enumerate()
		.filter_map(|(idx,opt)| opt.map(|parser| (idx,parser)))
		.map(|(idx,parser)| Box::new((2020usize,idx/2+1,idx%2+1,parser)) as Box<dyn super::InputParser>)
		.collect()
}

