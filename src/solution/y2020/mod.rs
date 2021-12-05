mod d1pt1;
pub use d1pt1::Solution as D1Pt1;
mod d1pt2;
pub use d1pt2::Solution as D1Pt2;
mod d2pt1;
pub use d2pt1::Solution as D2Pt1;
mod d2pt2;
pub use d2pt2::Solution as D2Pt2;
mod d3pt1;
pub use d3pt1::Solution as D3Pt1;
mod d3pt2;
pub use d3pt2::Solution as D3Pt2;
mod d4pt1;
pub use d4pt1::Solution as D4Pt1;
mod d4pt2;
pub use d4pt2::Solution as D4Pt2;
mod d5pt1;
pub use d5pt1::Solution as D5Pt1;
mod d5pt2;
pub use d5pt2::Solution as D5Pt2;
mod d6pt1;
pub use d6pt1::Solution as D6Pt1;
mod d6pt2;
pub use d6pt2::Solution as D6Pt2;
mod d7pt1;
pub use d7pt1::Solution as D7Pt1;
mod d7pt2;
pub use d7pt2::Solution as D7Pt2;
mod d8pt1;
pub use d8pt1::Solution as D8Pt1;
mod d8pt2;
pub use d8pt2::Solution as D8Pt2;
mod d9pt1;
pub use d9pt1::Solution as D9Pt1;
mod d9pt2;
pub use d9pt2::Solution as D9Pt2;
mod d10pt1;
pub use d10pt1::Solution as D10Pt1;
mod d10pt2;
pub use d10pt2::Solution as D10Pt2;
mod d11pt1;
pub use d11pt1::Solution as D11Pt1;
mod d11pt2;
pub use d11pt2::Solution as D11Pt2;
mod d12pt1;
pub use d12pt1::Solution as D12Pt1;
mod d12pt2;
pub use d12pt2::Solution as D12Pt2;
mod d13pt1;
pub use d13pt1::Solution as D13Pt1;
mod d13pt2;
pub use d13pt2::Solution as D13Pt2;
mod d14pt1;
pub use d14pt1::Solution as D14Pt1;
mod d14pt2;
pub use d14pt2::Solution as D14Pt2;
mod d15;
pub use d15::Solution as D15;
mod d16pt1;
pub use d16pt1::Solution as D16Pt1;
mod d16pt2;
pub use d16pt2::Solution as D16Pt2;
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
mod d24pt2;
pub use d24pt2::Solution as D24Pt2;
mod d25pt1;
pub use d25pt1::Solution as D25Pt1;

pub fn parsers<'a>() -> Vec<Box<dyn super::InputParser<'a>>>
{
	enum InputType<'a>
	{
		UrlInput(Box<dyn Fn(Option<String>) -> Box<dyn super::Solution + 'static>>),
		Parser(Box<dyn super::InputParser<'a>>),
	}

	let days: Vec<Option<InputType>> = vec!
	[
		Some(InputType::UrlInput(Box::new(|input| Box::new(D1Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D1Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D2Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D2Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D3Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D3Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D4Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D4Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D5Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D5Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D6Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D6Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D7Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D7Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D8Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D8Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D9Pt1::new(input.expect("empty input received"), 25))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D9Pt2::new(input.expect("empty input received"), 25))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D10Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D10Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D11Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D11Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D12Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D12Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D13Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D13Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D14Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D14Pt2::new(input.expect("empty input received")))))),
		Some(InputType::Parser(D15::parser_pt1())),
		Some(InputType::Parser(D15::parser_pt2())),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D16Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D16Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D17Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D17Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D18Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D18Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D19Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D19Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D20Pt1::new(input.expect("empty input received")))))),
		None,
		Some(InputType::UrlInput(Box::new(|input| Box::new(D21Pt1::new(input.expect("empty input received")))))),
		None,
		Some(InputType::UrlInput(Box::new(|input| Box::new(D22Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D22Pt2::new(input.expect("empty input received")))))),
		Some(InputType::Parser(D23Pt1::parser())),
		Some(InputType::Parser(D23Pt2::parser())),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D24Pt1::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D24Pt2::new(input.expect("empty input received")))))),
		Some(InputType::UrlInput(Box::new(|input| Box::new(D25Pt1::new(input.expect("empty input received")))))),
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

