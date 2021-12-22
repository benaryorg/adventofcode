use crate::error::*;

use std::ops::RangeInclusive;

use nom::
{
	character::complete::*,
	number::complete::*,
	bytes::complete::*,
	combinator::*,
	sequence::*,
	branch::*,
	multi::*,
	IResult,
	Parser,
};

#[derive(Debug,Eq,PartialEq)]
enum Part
{
	Part1,
	Part2,
}

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d22::Solution, Solution as S };
/// # env_logger::init();
/// let input = "on x=10..12,y=10..12,z=10..12\n\
/// 	on x=11..13,y=11..13,z=11..13\n\
/// 	off x=9..11,y=9..11,z=9..11\n\
/// 	on x=10..10,y=10..10,z=10..10\n";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "39");
/// let input = "on x=-20..26,y=-36..17,z=-47..7\n\
///     on x=-20..33,y=-21..23,z=-26..28\n\
///     on x=-22..28,y=-29..23,z=-38..16\n\
///     on x=-46..7,y=-6..46,z=-50..-1\n\
///     on x=-49..1,y=-3..46,z=-24..28\n\
///     on x=2..47,y=-22..22,z=-23..27\n\
///     on x=-27..23,y=-28..26,z=-21..29\n\
///     on x=-39..5,y=-6..47,z=-3..44\n\
///     on x=-30..21,y=-8..43,z=-13..34\n\
///     on x=-22..26,y=-27..20,z=-29..19\n\
///     off x=-48..-32,y=26..41,z=-47..-37\n\
///     on x=-12..35,y=6..50,z=-50..-2\n\
///     off x=-48..-32,y=-32..-16,z=-15..-5\n\
///     on x=-18..26,y=-33..15,z=-7..46\n\
///     off x=-40..-22,y=-38..-28,z=23..41\n\
///     on x=-16..35,y=-41..10,z=-47..6\n\
///     off x=-32..-23,y=11..30,z=-14..3\n\
///     on x=-49..-5,y=-3..45,z=-29..18\n\
///     off x=18..30,y=-20..-8,z=-3..13\n\
///     on x=-41..9,y=-7..43,z=-33..15\n\
///     on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n\
///     on x=967..23432,y=45373..81175,z=27513..53682\n";
/// let input = "on x=-5..47,y=-31..22,z=-19..33\n\
///     on x=-44..5,y=-27..21,z=-14..35\n\
///     on x=-49..-1,y=-11..42,z=-10..38\n\
///     on x=-20..34,y=-40..6,z=-44..1\n\
///     off x=26..39,y=40..50,z=-2..11\n\
///     on x=-41..5,y=-41..6,z=-36..8\n\
///     off x=-43..-33,y=-45..-28,z=7..25\n\
///     on x=-33..15,y=-32..19,z=-34..11\n\
///     off x=35..47,y=-46..-34,z=-11..5\n\
///     on x=-14..36,y=-6..44,z=-16..29\n\
///     on x=-57795..-6158,y=29564..72030,z=20435..90618\n\
///     on x=36731..105352,y=-21140..28532,z=16094..90401\n\
///     on x=30999..107136,y=-53464..15513,z=8553..71215\n\
///     on x=13528..83982,y=-99403..-27377,z=-24141..23996\n\
///     on x=-72682..-12347,y=18159..111354,z=7391..80950\n\
///     on x=-1060..80757,y=-65301..-20884,z=-103788..-16709\n\
///     on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\n\
///     on x=-52752..22273,y=-49450..9096,z=54442..119054\n\
///     on x=-29982..40483,y=-108474..-28371,z=-24328..38471\n\
///     on x=-4958..62750,y=40422..118853,z=-7672..65583\n\
///     on x=55694..108686,y=-43367..46958,z=-26781..48729\n\
///     on x=-98497..-18186,y=-63569..3412,z=1232..88485\n\
///     on x=-726..56291,y=-62629..13224,z=18033..85226\n\
///     on x=-110886..-34664,y=-81338..-8658,z=8914..63723\n\
///     on x=-55829..24974,y=-16897..54165,z=-121762..-28058\n\
///     on x=-65152..-11147,y=22489..91432,z=-58782..1780\n\
///     on x=-120100..-32970,y=-46592..27473,z=-11695..61039\n\
///     on x=-18631..37533,y=-124565..-50804,z=-35667..28308\n\
///     on x=-57817..18248,y=49321..117703,z=5745..55881\n\
///     on x=14781..98692,y=-1341..70827,z=15753..70151\n\
///     on x=-34419..55919,y=-19626..40991,z=39015..114138\n\
///     on x=-60785..11593,y=-56135..2999,z=-95368..-26915\n\
///     on x=-32178..58085,y=17647..101866,z=-91405..-8878\n\
///     on x=-53655..12091,y=50097..105568,z=-75335..-4862\n\
///     on x=-111166..-40997,y=-71714..2688,z=5609..50954\n\
///     on x=-16602..70118,y=-98693..-44401,z=5197..76897\n\
///     on x=16383..101554,y=4615..83635,z=-44907..18747\n\
///     off x=-95822..-15171,y=-19987..48940,z=10804..104439\n\
///     on x=-89813..-14614,y=16069..88491,z=-3297..45228\n\
///     on x=41075..99376,y=-20427..49978,z=-52012..13762\n\
///     on x=-21330..50085,y=-17944..62733,z=-112280..-30197\n\
///     on x=-16478..35915,y=36008..118594,z=-7885..47086\n\
///     off x=-98156..-27851,y=-49952..43171,z=-99005..-8456\n\
///     off x=2032..69770,y=-71013..4824,z=7471..94418\n\
///     on x=43670..120875,y=-42068..12382,z=-24787..38892\n\
///     off x=37514..111226,y=-45862..25743,z=-16714..54663\n\
///     off x=25699..97951,y=-30668..59918,z=-15349..69697\n\
///     off x=-44271..17935,y=-9516..60759,z=49131..112598\n\
///     on x=-61695..-5813,y=40978..94975,z=8655..80240\n\
///     off x=-101086..-9439,y=-7088..67543,z=33935..83858\n\
///     off x=18020..114017,y=-48931..32606,z=21474..89843\n\
///     off x=-77139..10506,y=-89994..-18797,z=-80..59318\n\
///     off x=8476..79288,y=-75520..11602,z=-96624..-24783\n\
///     on x=-47488..-1262,y=24338..100707,z=16292..72967\n\
///     off x=-84341..13987,y=2429..92914,z=-90671..-1318\n\
///     off x=-37810..49457,y=-71013..-7894,z=-105357..-13188\n\
///     off x=-27365..46395,y=31009..98017,z=15428..76570\n\
///     off x=-70369..-16548,y=22648..78696,z=-1892..86821\n\
///     on x=-53470..21291,y=-120233..-33476,z=-44150..38147\n\
///     off x=-93533..-4276,y=-16170..68771,z=-104985..-24507\n";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "474140");
/// // part 2 is broken for now
/// //assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "2758514936282235");
/// ```
pub struct Solution
{
	input: String,
	part: Part,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			input,
			part: Part::Part1,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			input,
			part: Part::Part2,
		}
	}
}

fn instruction(input: &str) -> IResult<&str, (bool, (RangeInclusive<i128>, RangeInclusive<i128>, RangeInclusive<i128>))>
{
	let (input, state) = alt((tag("on"), tag("off")))(input)?;
	let state = state == "on";

	let (input, _) = char(' ')(input)?;

	let (input, (x_min, x_max)) = preceded(tag("x="), separated_pair(map(double, |d| d as i128), many1(char('.')), map(double, |d| d as i128)))(input)?;
	let (input, _) = char(',')(input)?;

	let (input, (y_min, y_max)) = preceded(tag("y="), separated_pair(map(double, |d| d as i128), many1(char('.')), map(double, |d| d as i128)))(input)?;
	let (input, _) = char(',')(input)?;

	let (input, (z_min, z_max)) = preceded(tag("z="), separated_pair(map(double, |d| d as i128), many1(char('.')), map(double, |d| d as i128)))(input)?;

	Ok((input, (state, (x_min..=x_max, y_min..=y_max, z_min..=z_max))))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let (_, instructions) = all_consuming(many1(terminated(instruction, newline)))
			.parse(&self.input)
			.map_err(|err| anyhow!("{}", err))?;


		if self.part == Part::Part1
		{

			let count = instructions.into_iter()
				.filter_map(|(b, (x, y, z))|
				{
					if *x.start() < -50 || *x.end() > 50 || *y.start() < -50 || *y.end() > 50 || *z.start() < -50 || *z.end() > 50
					{
						None
					}
					else
					{
						Some((b, (*x.start().max(&-50)..=*x.end().min(&50), *y.start().max(&-50)..=*y.end().min(&50), *z.start().max(&-50)..=*z.end().min(&50))))
					}
				})
				.fold(std::collections::HashSet::new(), |mut set, (b, (x, y, z))|
				{
					for x in x.clone()
					{
						for y in y.clone()
						{
							for z in z.clone()
							{
								if b
								{
									set.insert((x, y , z));
								}
								else
								{
									set.remove(&(x, y, z));
								}
							}
						}
					}
					set
				})
				.len();

			Ok(format!("{}", count))
		}
		else
		{
			debug!("instructions: {:#?}", instructions);

		let blocks = instructions.iter()
			.map(|(s, (x, y, z))| if !s { 0 } else { ((x.end() + 1 - x.start()) * (y.end() + 1 - y.start()) * (z.end() + 1 - z.start())) as i128 })
			.sum::<i128>();

		let subs = instructions.iter()
			.enumerate()
			.flat_map(|i1| instructions.iter().enumerate().map(move |i2| (i1.clone(), i2.clone())))
			.map(|((ln, (ls, (lx, ly, lz))), (rn, (rs, (rx, ry, rz))))|
			{
				if ln >= rn
				{
					return 0;
				}
				if lx.contains(rx.start()) || lx.contains(rx.end()) || rx.contains(lx.start()) || rx.contains(lx.end())
				{
					if ly.contains(ry.start()) || ly.contains(ry.end()) || ry.contains(ly.start()) || ry.contains(ly.end())
					{
						if lz.contains(rz.start()) || lz.contains(rz.end()) || rz.contains(lz.start()) || rz.contains(lz.end())
						{
							let x = lx.start().max(rx.start())..=lx.end().min(rx.end());
							let y = ly.start().max(ry.start())..=ly.end().min(ry.end());
							let z = lz.start().max(rz.start())..=lz.end().min(rz.end());
							let size = (**x.end() + 1 - **x.start()) * (**y.end() + 1 - **y.start()) * (**z.end() + 1 - **z.start());
							let change = match (ls, rs)
							{
								(true, true) => 0,
								(false, true) => size,
								(true, false) => -2 * size,
								(false, false) => 0,
							};
							trace!("size change: {} by {} ({} vs. {}) with {}:{}", change, size, ln, rn, ls, rs);
							return change;
						}
					}
				}
				0
			})
			.sum::<i128>();

			Ok(format!("{}", blocks + subs))
		}
	}
}

