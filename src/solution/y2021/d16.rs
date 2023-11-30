use crate::error::*;

use nom::
{
	bits::complete::*,
	combinator::*,
	sequence::*,
	multi::*,
	bits::*,
	IResult,
};

#[derive(Debug,PartialEq,Eq)]
enum Action
{
	SumVersion,
	Execute,
}

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d16::Solution, Solution as S };
/// # env_logger::init();
/// assert_eq!(Solution::part1("8A004A801A8002F478".to_string()).solve().unwrap(), "16");
/// assert_eq!(Solution::part1("620080001611562C8802118E34".to_string()).solve().unwrap(), "12");
/// assert_eq!(Solution::part1("C0015000016115A2E0802F182340".to_string()).solve().unwrap(), "23");
/// assert_eq!(Solution::part1("A0016C880162017C3686B18A3D4780".to_string()).solve().unwrap(), "31");
/// assert_eq!(Solution::part2("C200B40A82".to_string()).solve().unwrap(), "3");
/// assert_eq!(Solution::part2("04005AC33890".to_string()).solve().unwrap(), "54");
/// assert_eq!(Solution::part2("880086C3E88112".to_string()).solve().unwrap(), "7");
/// assert_eq!(Solution::part2("CE00C43D881120".to_string()).solve().unwrap(), "9");
/// assert_eq!(Solution::part2("D8005AC2A8F0".to_string()).solve().unwrap(), "1");
/// assert_eq!(Solution::part2("F600BC2D8F".to_string()).solve().unwrap(), "0");
/// assert_eq!(Solution::part2("9C005AC2F8F0".to_string()).solve().unwrap(), "0");
/// assert_eq!(Solution::part2("9C0141080250320F1802104A08".to_string()).solve().unwrap(), "1");
/// ```
pub struct Solution
{
	input: String,
	action: Action,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			input,
			action: Action::SumVersion,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			input,
			action: Action::Execute,
		}
	}
}

#[derive(Debug,PartialEq,Eq,Clone)]
enum Operator
{
	Sum,
	Product,
	Min,
	Max,
	GreaterThan,
	LessThan,
	Equal,
}

#[derive(Debug,PartialEq,Eq)]
enum Content
{
	Literal(u128),
	Operator(Operator, std::collections::VecDeque<Packet>),
}

#[derive(Debug,PartialEq,Eq)]
struct Packet
{
	version: u8,
	content: Content,
}

impl Packet
{
	fn calculate(&self) -> u128
	{
		match self.content
		{
			Content::Literal(lit) => lit,
			Content::Operator(Operator::Sum, ref vec) => vec.iter().map(Packet::calculate).sum(),
			Content::Operator(Operator::Product, ref vec) => vec.iter().map(Packet::calculate).product(),
			Content::Operator(Operator::Min, ref vec) => vec.iter().map(Packet::calculate).min().unwrap(),
			Content::Operator(Operator::Max, ref vec) => vec.iter().map(Packet::calculate).max().unwrap(),
			Content::Operator(Operator::GreaterThan, ref vec) => vec.iter().skip(1).map(Packet::calculate).all(|x| vec[0].calculate() > x) as u128,
			Content::Operator(Operator::LessThan, ref vec) => vec.iter().skip(1).map(Packet::calculate).all(|x| vec[0].calculate() < x) as u128,
			Content::Operator(Operator::Equal, ref vec) => vec.iter().skip(1).map(Packet::calculate).all(|x| vec[0].calculate() == x) as u128,
		}
	}
}

fn literal(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), Content>
{
	trace!("parsing literal");
	let mut num: u128 = 0;
	loop
	{
		trace!("parsing literal: another nibble");
		let (rest, (cont, nibble)): (_, (u8, u8)) = tuple((take(1_usize), take(4_usize)))(input)?;

		num <<= 4;
		num |= nibble as u128;
		input = rest;
		if cont != 1
		{
			break;
		}
	}

	Ok((input, Content::Literal(num)))
}

fn operator(type_: usize, input: (&[u8], usize)) -> IResult<(&[u8], usize), Content>
{
	trace!("parsing operator");
	let (input, len_type): (_, u8) = take(1_usize)(input)?;

	let (input, packets) = if len_type == 1
	{
		let (input, packet_count) = take(11_usize)(input)?;
		trace!("parsing operator: reading {} packets", packet_count);
		let (input, packets) = count(packet, packet_count)(input)?;
		(input, packets)
	}
	else
	{
		let (input, bit_length): (_, usize) = take(15_usize)(input)?;
		let (input, partial) = take(bit_length % 8)(input)?;
		let (input, full_bytes): (_, Vec<u8>) = count(take(8_usize), bit_length / 8)(input)?;
		let mut v = vec![partial];
		v.extend(full_bytes);
		trace!("parsing operator: split off {} bits: {:?}", bit_length, (&v, 8 - (bit_length % 8)));
		let (_, packets) = terminated(many0(packet), eof)((&v, 8 - (bit_length % 8))).map_err(|err| unimplemented!("{}", err))?;
		(input, packets)
	};

	let type_ = match type_
	{
		0 => Operator::Sum,
		1 => Operator::Product,
		2 => Operator::Min,
		3 => Operator::Max,
		5 => Operator::GreaterThan,
		6 => Operator::LessThan,
		7 => Operator::Equal,
		_ =>
		{
			let _: (_, usize) = fail(input)?;
			unreachable!();
		},
	};

	Ok((input, Content::Operator(type_, packets.into())))
}

fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet>
{
	trace!("parsing packet: {:?}", input);
	let (input, version) = take(3_usize)(input)?;
	trace!("parsing packet, version: {}", version);
	let (input, type_) = take(3_usize)(input)?;
	trace!("parsing packet, type: {}", type_);

	let (input, content) = match type_
	{
		4 => literal(input)?,
		_ => operator(type_, input)?,
	};

	trace!("parsing packet, content: {:?}", content);

	Ok((input, Packet
	{
		version,
		content,
	}))
}


impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let input = self.input.trim()
			.as_bytes()
			.chunks(2)
			.map(|v|
			{
				let a = v[0];
				let b = v[1];

				let a = match a
				{
					b'0'..=b'9' => a - b'0',
					b'A'..=b'F' => a - b'A' + 10,
					_ => Err(Error::AocParsing).context("non-hex-digit encountered")?,
				};
				let b = match b
				{
					b'0'..=b'9' => b - b'0',
					b'A'..=b'F' => b - b'A' + 10,
					_ => Err(Error::AocParsing).context("non-hex-digit encountered")?,
				};
				Ok(a << 4 | b)
			})
			.collect::<Result<Vec<_>>>()?;

		debug!("actual input: {:?}", input);

		let (_, packet) = bits::<_, _, _, nom::error::Error<_>, _>(terminated(packet, terminated(many0(tag(0, 1_usize)), eof)))(&input)
			.map_err(|err| anyhow!("{}", err))?;

		if self.action == Action::SumVersion
		{
			let mut vec = vec![packet];
			let version_sum = std::iter::from_fn(move ||
			{
				if vec.is_empty()
				{
					return None;
				}

				let packet = vec.pop().unwrap();

				{
					if let Content::Operator(_, packets) = packet.content
					{
						vec.extend(packets);
					}
					Some(packet.version as usize)
				}
			}).sum::<usize>();

			Ok(format!("{}", version_sum))
		}
		else
		{
			Ok(format!("{}", packet.calculate()))
		}
	}
}

