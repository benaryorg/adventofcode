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
pub mod d7pt2;
pub mod d8pt1;
pub mod d8pt2;
pub mod d9pt1;
pub mod d9pt2;
pub mod d10pt1;
pub mod d10pt2;
pub mod d11;
pub mod d12;
pub mod d13pt1;
pub mod d13pt2;
pub mod d14pt1;
pub mod d14pt2;
pub mod d15;
pub mod d16;
pub mod d17;
pub mod d18;
pub mod d19;
pub mod d20;
pub mod d21;
pub mod d22;
pub mod d23;
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
		Some(InputType::UrlInput(Box::new(|input| Box::new(d5pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d5pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d6pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d6pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d7pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d7pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d8pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d8pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d9pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d9pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d10pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d10pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d11::Solution::with_empty(2, input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d11::Solution::with_empty(1000000, input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d12::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d12::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d13pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d13pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d14pt1::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d14pt2::Solution::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d15::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d15::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d16::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d16::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d17::Solution::with_min_max(1, 3, input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d17::Solution::with_min_max(4, 10, input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d18::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d18::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d19::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d19::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d20::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d20::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d21::Solution::with_steps(64, input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d21::Solution::with_steps(26501365, input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d22::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d22::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d23::Solution::part1(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d23::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d24::Solution::part1(200000000000000..=400000000000000, input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d24::Solution::part2(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(d25::Solution::new(input.expect("empty input received")))))),
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

