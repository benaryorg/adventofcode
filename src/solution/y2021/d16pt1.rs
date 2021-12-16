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

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d16pt1::Solution, Solution as S };
/// # env_logger::init();
/// assert_eq!(Solution::new("8A004A801A8002F478".to_string()).solve().unwrap(), "16");
/// assert_eq!(Solution::new("620080001611562C8802118E34".to_string()).solve().unwrap(), "12");
/// assert_eq!(Solution::new("C0015000016115A2E0802F182340".to_string()).solve().unwrap(), "23");
/// assert_eq!(Solution::new("A0016C880162017C3686B18A3D4780".to_string()).solve().unwrap(), "31");
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

#[derive(Debug)]
enum Content
{
	Literal(usize),
	Operator(u8, Vec<Packet>),
}

#[derive(Debug)]
struct Packet
{
	version: u8,
	content: Content,
}

fn literal(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), Content>
{
	trace!("parsing literal");
	let mut num = 0;
	while let Ok((rest, _)) = tag::<_, _, _, ()>(1, 1_usize)(input)
	{
		trace!("parsing literal: another nibble");
		num <<= 4;
		let (rest, bits): (_, usize) = take(4_usize)(rest)?;
		num |= bits;
		input = rest;
	}

	let (input, _) = tag(0, 1_usize)(input)?;

	trace!("parsing literal: last nibble");
	num <<= 4;
	let (input, bits): (_, usize) = take(4_usize)(input)?;
	num |= bits;

	Ok((input, Content::Literal(num)))
}

fn operator(type_: u8, input: (&[u8], usize)) -> IResult<(&[u8], usize), Content>
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

	Ok((input, Content::Operator(type_, packets)))
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
				Ok((a << 4 | b) as u8)
			})
			.collect::<Result<Vec<_>>>()?;

		debug!("actual input: {:?}", input);

		let (_, packet) = bits::<_, _, _, nom::error::Error<_>, _>(terminated(packet, terminated(many0(tag(0, 1_usize)), eof)))(&input)
			.map_err(|err| anyhow!("{}", err))?;

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
}

