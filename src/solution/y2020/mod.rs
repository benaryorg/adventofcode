mod d17pt1;
pub use d17pt1::Solution as D17Pt1;
mod d17pt2;
pub use d17pt2::Solution as D17Pt2;
mod d18pt1;
pub use d18pt1::Solution as D18Pt1;

pub fn parsers<'a>() -> Vec<Box<dyn super::InputParser<'a>>>
{
	vec!
	[
		super::helper::coerce_parser_input((2020,17,1),|input| { D17Pt1::new(input.expect("empty input received")) } ),
		super::helper::coerce_parser_input((2020,17,2),|input| { D17Pt2::new(input.expect("empty input received")) } ),
		super::helper::coerce_parser_input((2020,18,1),|input| { D18Pt1::new(input.expect("empty input received")) } ),
	]
}

