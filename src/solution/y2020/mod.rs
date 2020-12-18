mod d17pt2;

pub fn parsers<'a>() -> Vec<Box<dyn super::InputParser<'a>>>
{
	vec!
	[
		super::helper::coerce_parser_input((2020,17,2),|input| { d17pt2::Solution::new(input.expect("empty input received")) } ),
	]
}

