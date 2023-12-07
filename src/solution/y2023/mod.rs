pub mod d1pt1;
pub mod d1pt2;
pub mod d2pt1;
pub mod d2pt2;
pub mod d3pt1;
pub mod d3pt2;
pub mod d4pt1;
pub mod d4pt2;
pub mod d5pt1;
pub mod d5pt2;
pub mod d6pt1;
pub mod d6pt2;
pub mod d7pt1;

pub fn parsers<'a>() -> Vec<Box<dyn super::InputParser<'a>>>
{
	#[allow(unused)]
	enum InputType<'a>
	{
		UrlInput(Box<dyn Fn(Option<String>) -> Box<dyn super::Solution + 'static>>),
		Parser(Box<dyn super::InputParser<'a>>),
	}

	let days: Vec<Option<InputType>> = vec!
	[
		Some(InputType::UrlInput(Box::new(|input| Box::new(d1pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d1pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d2pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d2pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d3pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d3pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d4pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d4pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d5pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d5pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d6pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d6pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d7pt1::Solution::new(input.expect("empty input received")))))),
	];

	days.into_iter()
		.enumerate()
		.filter_map(|(idx,opt)| opt.map(|parser| (idx,parser)))
		.map(|(idx,parser)|
			match parser
			{
				InputType::UrlInput(parser) => Box::new((2023usize,idx/2+1,idx%2+1,parser)) as Box<dyn super::InputParser>,
				InputType::Parser(parser) => parser,
			}
		)
		.collect()
}

