use crate::error::*;

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

// TODO: day 19 is missing so far

/// # Examples
///
/// ```no_run
/// # use adventofcode::solution::{ y2021::d19::Solution, Solution as S };
/// # env_logger::init();
/// let input = "--- scanner 0 ---\n\
///     0,0,2\n\
///     0,4,1\n\
///     0,3,3\n\
///     \n\
///     --- scanner 1 ---\n\
///     0,-1,-1\n\
///     0,-5,0\n\
///     0,-2,1\n";
/// //assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "79");
/// let input = "--- scanner 0 ---\n\
///     404,-588,-901\n\
///     528,-643,409\n\
///     -838,591,734\n\
///     390,-675,-793\n\
///     -537,-823,-458\n\
///     -485,-357,347\n\
///     -345,-311,381\n\
///     -661,-816,-575\n\
///     -876,649,763\n\
///     -618,-824,-621\n\
///     553,345,-567\n\
///     474,580,667\n\
///     -447,-329,318\n\
///     -584,868,-557\n\
///     544,-627,-890\n\
///     564,392,-477\n\
///     455,729,728\n\
///     -892,524,684\n\
///     -689,845,-530\n\
///     423,-701,434\n\
///     7,-33,-71\n\
///     630,319,-379\n\
///     443,580,662\n\
///     -789,900,-551\n\
///     459,-707,401\n\
///     \n\
///     --- scanner 1 ---\n\
///     686,422,578\n\
///     605,423,415\n\
///     515,917,-361\n\
///     -336,658,858\n\
///     95,138,22\n\
///     -476,619,847\n\
///     -340,-569,-846\n\
///     567,-361,727\n\
///     -460,603,-452\n\
///     669,-402,600\n\
///     729,430,532\n\
///     -500,-761,534\n\
///     -322,571,750\n\
///     -466,-666,-811\n\
///     -429,-592,574\n\
///     -355,545,-477\n\
///     703,-491,-529\n\
///     -328,-685,520\n\
///     413,935,-424\n\
///     -391,539,-444\n\
///     586,-435,557\n\
///     -364,-763,-893\n\
///     807,-499,-711\n\
///     755,-354,-619\n\
///     553,889,-390\n\
///     \n\
///     --- scanner 2 ---\n\
///     649,640,665\n\
///     682,-795,504\n\
///     -784,533,-524\n\
///     -644,584,-595\n\
///     -588,-843,648\n\
///     -30,6,44\n\
///     -674,560,763\n\
///     500,723,-460\n\
///     609,671,-379\n\
///     -555,-800,653\n\
///     -675,-892,-343\n\
///     697,-426,-610\n\
///     578,704,681\n\
///     493,664,-388\n\
///     -671,-858,530\n\
///     -667,343,800\n\
///     571,-461,-707\n\
///     -138,-166,112\n\
///     -889,563,-600\n\
///     646,-828,498\n\
///     640,759,510\n\
///     -630,509,768\n\
///     -681,-892,-333\n\
///     673,-379,-804\n\
///     -742,-814,-386\n\
///     577,-820,562\n\
///     \n\
///     --- scanner 3 ---\n\
///     -589,542,597\n\
///     605,-692,669\n\
///     -500,565,-823\n\
///     -660,373,557\n\
///     -458,-679,-417\n\
///     -488,449,543\n\
///     -626,468,-788\n\
///     338,-750,-386\n\
///     528,-832,-391\n\
///     562,-778,733\n\
///     -938,-730,414\n\
///     543,643,-506\n\
///     -524,371,-870\n\
///     407,773,750\n\
///     -104,29,83\n\
///     378,-903,-323\n\
///     -778,-728,485\n\
///     426,699,580\n\
///     -438,-605,-362\n\
///     -469,-447,-387\n\
///     509,732,623\n\
///     647,635,-688\n\
///     -868,-804,481\n\
///     614,-800,639\n\
///     595,780,-596\n\
///     \n\
///     --- scanner 4 ---\n\
///     727,592,562\n\
///     -293,-554,779\n\
///     441,611,-461\n\
///     -714,465,-776\n\
///     -743,427,-804\n\
///     -660,-479,-426\n\
///     832,-632,460\n\
///     927,-485,-438\n\
///     408,393,-506\n\
///     466,436,-512\n\
///     110,16,151\n\
///     -258,-428,682\n\
///     -393,719,612\n\
///     -211,-452,876\n\
///     808,-476,-593\n\
///     -575,615,604\n\
///     -485,667,467\n\
///     -680,325,-822\n\
///     -627,-443,-432\n\
///     872,-547,-609\n\
///     833,512,582\n\
///     807,604,487\n\
///     839,-516,451\n\
///     891,-625,532\n\
///     -652,-548,-490\n\
///     30,-46,-14\n";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "79");
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

type Set<V> = std::collections::BTreeSet<V>;
type Map<K, V> = std::collections::BTreeMap<K, V>;

fn scanner(input: &str) -> IResult<&str, (usize, Vec<(isize, isize, isize)>)>
{
	trace!("parsing scanner");
	let (input, id) = delimited(tag("--- scanner "), map(double, |d| d as usize), tag(" ---\n"))(input)?;
	trace!("parsing scanner {}", id);
	let (input, vec) = many1(tuple((terminated(map(double, |d| d as isize), char(',')), terminated(map(double, |d| d as isize), char(',')), terminated(map(double, |d| d as isize), char('\n')))))(input)?;
	trace!("have scanner {}: {:?}", id, vec);

	Ok((input, (id, vec)))
}

fn untransform(num: usize, x: isize, y: isize, z: isize) -> (isize, isize, isize)
{
	[
		(|x: isize, y: isize, z: isize| { (x, y, z) })(x, y, z),
		(|x: isize, z: isize, y: isize| { (x, y, -z) })(x, y, z),
		(|x: isize, y: isize, z: isize| { (x, -y, -z) })(x, y, z),
		(|x: isize, z: isize, y: isize| { (x, -y, z) })(x, y, z),
		(|y: isize, x: isize, z: isize| { (x, -y, z) })(x, y, z),
		(|y: isize, x: isize, z: isize| { (x, y, -z) })(x, y, z),
		(|z: isize, x: isize, y: isize| { (x, y, z) })(x, y, z),
		(|z: isize, x: isize, y: isize| { (x, -y, -z) })(x, y, z),
		(|x: isize, y: isize, z: isize| { (-x, -y, z) })(x, y, z),
		(|x: isize, y: isize, z: isize| { (-x, y, z) })(x, y, z),
		(|x: isize, y: isize, z: isize| { (-x, y, -z) })(x, y, z),
		(|x: isize, y: isize, z: isize| { (-x, -y, -z) })(x, y, z),
		(|y: isize, x: isize, z: isize| { (-x, y, z) })(x, y, z),
		(|y: isize, x: isize, z: isize| { (x, -y, -z) })(x, y, z),
		(|z: isize, x: isize, y: isize| { (-x, y, -z) })(x, y, z),
		(|z: isize, x: isize, y: isize| { (-x, -y, z) })(x, y, z),
		(|z: isize, y: isize, x: isize| { (x, y, -z) })(x, y, z),
		(|z: isize, y: isize, x: isize| { (x, -y, z) })(x, y, z),
		(|y: isize, z: isize, x: isize| { (x, y, z) })(x, y, z),
		(|y: isize, z: isize, x: isize| { (x, -y, -z) })(x, y, z),
		(|z: isize, y: isize, x: isize| { (-x, y, z) })(x, y, z),
		(|z: isize, y: isize, x: isize| { (-x, -y, -z) })(x, y, z),
		(|y: isize, z: isize, x: isize| { (-x, -y, z) })(x, y, z),
		(|y: isize, z: isize, x: isize| { (-x, y, -z) })(x, y, z),
	][num]
}

fn transform(num: usize, x: isize, y: isize, z: isize) -> (isize, isize, isize)
{
	[
		(x, y, z),
		(x, -z, y),
		(x, -y, -z),
		(x, z, -y),
		(-y, x, z),
		(y, x, -z),
		(z, x, y),
		(-z, x, -y),
		(-x, -y, z),
		(-x, y, z),
		(-x, y, -z),
		(-x, -y, -z),
		(y, -x, z),
		(-y, -x, -z),
		(-z, -x, y),
		(z, -x, -y),
		(-z, y, x),
		(z, -y, x),
		(y, z, x),
		(-y, -z, x),
		(z, y, -x),
		(-z, -y, -x),
		(-y, z, -x),
		(y, -z, -x),
	][num]
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let (_, scanners) = terminated(many1(terminated(scanner, alt((tag("\n"), eof)))), eof)
			.parse(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		let per_scanner_permutations = scanners.iter()
			.inspect(|(id, beacons)| trace!("scanner {} has {} beacons", id, beacons.len()))
			.map(|(_, beacons)|
			{
				beacons.iter()
					.flat_map(|&(x, y, z)|
					{
						let vec = beacons.iter()
							.map(|&(x_, y_, z_)|
							{
								(x_ - x, y_ - y, z_ - z)
							})
							.collect::<Vec<_>>();

						(0..=23)
							.map(move |n| (((x, y, z), n), vec.iter().map(|&(x, y, z)| transform(n, x, y, z)).collect::<Set<_>>()))
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<Vec<(_, Set<(isize, isize, isize)>)>>>();

		let mut map: Map<(usize, usize), Map<usize, usize>> = Default::default();

		per_scanner_permutations.iter()
			.enumerate()
			.for_each(|(outer_id, scanner)|
			{
				scanner.iter()
					.enumerate()
					.for_each(|(permutation_id, (_params, permutation))|
					{
						per_scanner_permutations.iter()
							.enumerate()
							.for_each(|(inner_id, inner_scanner)|
							{
								if inner_id == outer_id
								{
									return;
								}
								inner_scanner.iter()
									.enumerate()
									.filter(|(_, (_params, p))| p.intersection(permutation).count() >= 12)
									.inspect(|(inner_permutation_id, (_params, p))|
										trace!("({}, {}) (len {}) and ({}, {}) (len {}) have {} intersections",
											outer_id,
											permutation_id,
											permutation.len(),
											inner_id,
											inner_permutation_id,
											p.len(),
											p.intersection(permutation).count()
										)
									)
									.for_each(|(inner_permutation_id, _)|
									{
										let mut combined = Map::<usize, usize>::new();
										let left = map.get(&(inner_id, inner_permutation_id)).cloned().unwrap_or_default();
										combined.extend(left);
										let right = map.get(&(outer_id, permutation_id)).cloned().unwrap_or_default();
										combined.extend(right);
										combined.insert(outer_id, permutation_id);
										combined.insert(inner_id, inner_permutation_id);
										map.insert((outer_id, permutation_id), combined.clone());
										map.insert((inner_id, inner_permutation_id), combined.clone());
									})
							})
					})
			});

		//trace!("{:#?}", per_scanner_permutations);
		trace!("{:?}", map);

		if self.part == Part::Part1
		{
			let mapping: (&(usize, usize), &Map<usize, usize>) = map.iter()
				.max_by_key(|(_k, v)| v.len())
				.ok_or(Error::AocNoSolution)?;

			let ((base_x, base_y, base_z), base_trans) = per_scanner_permutations[mapping.0.0][mapping.0.1].0;

			debug!("{:?}", mapping);

			let mut beacons = mapping.1.iter().map(|(a, b)| (*a, *b)).chain(std::iter::once(*mapping.0))
				.flat_map(|(scanner, permutation)| per_scanner_permutations[scanner][permutation].1.iter())
				.copied()
				.collect::<Set<_>>();

			for n in (0..=scanners.len()).filter(|n| !mapping.1.contains_key(n) && *n != mapping.0.0)
			{
				let ((rel_scan, rel_perm), (target_scan, target_perm)) = map.iter()
					.find_map(|((rel_scan, rel_perm), map)|
					{
						if *rel_scan == mapping.0.0
						{
							return map.get(&n)
								.map(|b| ((rel_scan, rel_perm), (n, b)));
						}
						None
					})
					.ok_or(Error::AocNoSolution)?;
				let target_params = per_scanner_permutations[target_scan][*target_perm].0;
				beacons.extend(per_scanner_permutations[*rel_scan][*rel_perm].1
					.iter()
					.copied()
					.map(|(x, y, z)| untransform(target_params.1, x, y, z))
					.map(|(x, y, z)| transform(base_trans, x + target_params.0.0 - base_x, y + target_params.0.1 - base_y, z + target_params.0.2 - base_z)));
			}

			/*let beacons = solution.1.into_iter().chain(std::iter::once(solution.0))
				.map(|(scanner, permutation)| per_scanner_permutations[scanner][permutation].clone())
				.inspect(|set| trace!("found set with length {}", set.len()))
				.collect::<Set<_>>();*/

			Ok(format!("{}", 0))
		}
		else
		{
			unimplemented!()
		}
	}
}

