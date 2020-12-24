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
mod d21pt1;
pub use d21pt1::Solution as D21Pt1;
mod d21pt2;
pub use d21pt2::Solution as D21Pt2;
mod d22pt1;
pub use d22pt1::Solution as D22Pt1;
mod d22pt2;
pub use d22pt2::Solution as D22Pt2;
mod d23pt1;
pub use d23pt1::Solution as D23Pt1;
mod d23pt2;
pub use d23pt2::Solution as D23Pt2;
mod d24pt1;
pub use d24pt1::Solution as D24Pt1;

pub fn parsers<'a>() -> Vec<Box<dyn super::InputParser<'a>>>
{
	enum InputType<'a>
	{
		UrlInput(Box<dyn Fn(Option<String>) -> Box<dyn super::Solution + 'static>>),
		Parser(Box<dyn super::InputParser<'a>>),
	};

	let days: Vec<Option<InputType>> = vec!
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
		Some(InputType::UrlInput(Box::new(|input| Box::new(D17Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D17Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D18Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D18Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D19Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D19Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D20Pt1::new(input.expect("empty input received")))))),
		None,
		Some(InputType::UrlInput(Box::new(|input| Box::new(D21Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D21Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D22Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D22Pt2::new(input.expect("empty input received")))))),
		Some(InputType::Parser(D23Pt1::parser())),
		Some(InputType::Parser(D23Pt2::parser())),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D24Pt1::new(input.expect("empty input received")))))),
	];
	days.into_iter()
		.enumerate()
		.filter_map(|(idx,opt)| opt.map(|parser| (idx,parser)))
		.map(|(idx,parser)|
			match parser
			{
				InputType::UrlInput(parser) => Box::new((2020usize,idx/2+1,idx%2+1,parser)) as Box<dyn super::InputParser>,
				InputType::Parser(parser) => parser,
			}
		)
		.collect()
}

