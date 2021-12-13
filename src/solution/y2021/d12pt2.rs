use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d12pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "start-A\n\
///     start-b\n\
///     A-c\n\
///     A-b\n\
///     b-d\n\
///     A-end\n\
///     b-end";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "36");
/// let input = "dc-end\n\
///     HN-start\n\
///     start-kj\n\
///     dc-start\n\
///     dc-HN\n\
///     LN-dc\n\
///     HN-end\n\
///     kj-sa\n\
///     kj-HN\n\
///     kj-dc";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "103");
/// let input = "fs-end\n\
///     he-DX\n\
///     fs-he\n\
///     start-DX\n\
///     pj-DX\n\
///     end-zg\n\
///     zg-sl\n\
///     zg-pj\n\
///     pj-he\n\
///     RW-he\n\
///     fs-DX\n\
///     pj-RW\n\
///     zg-RW\n\
///     start-pj\n\
///     he-WI\n\
///     zg-he\n\
///     pj-fs\n\
///     start-RW";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "3509");
/// ```
pub struct Solution
{
	input: String,
}

impl Solution
{
	pub fn new(input: String) -> Self
	{
		Self { input, }
	}
}

type Cave = (usize, bool);
type Connection = (Cave, Cave);
type Path = std::sync::Arc<Vec<Cave>>;
type Calc = (bool, Path, Connection);

#[derive(Clone)]
struct Sender
{
	tx: std::sync::mpsc::Sender<(Self, Calc)>,
}

impl Sender
{
	fn send(&self, calc: Calc)
	{
		self.tx.send((self.clone(), calc)).unwrap();
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let connections: Vec<(&str, &str)> = self.input.lines()
			.map(|line|
			{
				let mut strs = line.split('-');
				Ok(
					( strs.next().ok_or(Error::AocParsing)?
					, strs.next().ok_or(Error::AocParsing)?
					)
				)
			})
			.collect::<Result<Vec<(&str, &str)>>>()?;

		let translation = connections.iter()
			.flat_map(|&(a, b)| [a, b])
			.collect::<std::collections::HashSet<_>>()
			.into_iter()
			.enumerate()
			.map(|(int, string)| (string, (int, string.chars().next().unwrap().is_ascii_uppercase())))
			.collect::<std::collections::HashMap<_,_>>();

		trace!("translation: {:?}", translation);

		let start = translation.get("start").copied().unwrap();
		let end = translation.get("end").copied().unwrap();

		let connections = connections.into_iter()
			.map(|(a, b)|
			{
				let a = *translation.get(a).unwrap();
				let b = *translation.get(b).unwrap();
				(a, b)
			})
			.collect::<Vec<_>>();

		let (tx, rx) = std::sync::mpsc::channel::<(Sender, Calc)>();
		let sender = Sender
		{
			tx: tx.clone(),
		};

		let iter = rx.into_iter()
			.filter(|(send, (repeat, path, (a, b))): &(Sender, Calc)|
			{
				if path.len() > 1 && (start.eq(a) || start.eq(b))
				{
					return false;
				}
				let current = path.last().copied().unwrap();
				let next = if current.eq(a)
				{
					b
				}
				else
				{
					if current.eq(b)
					{
						a
					}
					else
					{
						return false;
					}
				};
				if end.eq(next)
				{
					return true;
				}
				let repeat = if !next.1
				{
					let max_repeats = if *repeat { 1 } else { 2 };
					let current_repeats = path.iter().filter(|elem| next.eq(elem)).take(max_repeats).count();
					if current_repeats >= max_repeats
					{
						return false;
					}
					*repeat || current_repeats >= 1
				}
				else
				{
					*repeat
				};
				let mut path = path.as_ref().clone();
				path.push(*next);
				let path = std::sync::Arc::new(path.clone());
				for connection in connections.iter().copied()
				{
					send.send((repeat, path.clone(), connection));
				}
				false
			});

		for connection in connections.iter().copied()
		{
			tx.send((sender.clone(), (false, std::sync::Arc::new(vec![start]), connection))).unwrap();
		}
		drop(tx);
		drop(sender);

		Ok(format!("{}", iter.count()))
	}
}

