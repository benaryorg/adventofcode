pub mod d1pt1;
pub mod d1pt2;
pub mod d2pt1;
pub mod d2pt2;
pub mod d3pt1;
pub mod d3pt2;
pub mod d4pt1;
pub mod d4pt2;
pub mod d5;
pub mod d6;
pub mod d7pt1;
pub mod d7pt2;
pub mod d8pt1;
pub mod d8pt2;
pub mod d9pt1;
pub mod d9pt2;
pub mod d10pt1;
pub mod d10pt2;
pub mod d11pt1;
pub mod d11pt2;
pub mod d12pt1;
pub mod d12pt2;
pub mod d13pt1;
pub mod d13pt2;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d17pt1;
pub mod d17pt2;
pub mod d18;
pub mod d19;
pub mod d20;
pub mod d21;
pub mod d22;
pub mod d23pt1;
pub mod d23pt2;
pub mod d24;
pub mod d25;

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
		Some(InputType::UrlInput(Box::new(|input| Box::new(d5::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d5::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d6::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d6::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d7pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d7pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d8pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d8pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d9pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d9pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d10pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d10pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d11pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d11pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d12pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d12pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d13pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d13pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d14::Solution::with_steps(10, input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d14::Solution::with_steps(40, input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d15::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d15::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d16::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d16::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d17pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d17pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d18::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d18::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d19::Solution::part1(input.expect("empty input received")))))),
		None,
		Some(InputType::UrlInput(Box::new(|input| Box::new(d20::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d20::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d21::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d21::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d22::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d22::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d23pt1::Solution::new(input.expect("empty input received")))))),
		None,
		Some(InputType::UrlInput(Box::new(|input| Box::new(d24::Solution::part1(input.expect("empty input received")))))),
		None,
		Some(InputType::UrlInput(Box::new(|input| Box::new(d25::Solution::part1(input.expect("empty input received")))))),
	];

	days.into_iter()
		.enumerate()
		.filter_map(|(idx,opt)| opt.map(|parser| (idx,parser)))
		.map(|(idx,parser)|
			match parser
			{
				InputType::UrlInput(parser) => Box::new((2021usize,idx/2+1,idx%2+1,parser)) as Box<dyn super::InputParser>,
				InputType::Parser(parser) => parser,
			}
		)
		.collect()
}

