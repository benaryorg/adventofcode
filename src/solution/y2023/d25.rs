use crate::error::*;

use std::ops::Bound;

/// May not work for all inputs.
///
/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d25::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     jqt: rhn xhk nvd\n\
///     rsh: frs pzl lsr\n\
///     xhk: hfx\n\
///     cmg: qnr nvd lhk bvb\n\
///     rhn: xhk bvb hfx\n\
///     bvb: xhk hfx\n\
///     pzl: lsr hfx nvd\n\
///     qnr: nvd\n\
///     ntq: jqt hfx bvb xhk\n\
///     nvd: lhk\n\
///     lsr: lhk\n\
///     rzs: qnr cmg lsr rsh\n\
///     frs: qnr lhk lsr";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "54");
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

fn count_cluster<T>(origin: T, connections: &std::collections::BTreeSet<(T, T)>) -> usize
	where
		T: Ord,
{
	let mut set = std::collections::BTreeSet::<&T>::new();
	set.insert(&origin);
	loop
	{
		let len = set.len();
		for (a, b) in connections.iter()
		{
			if set.contains(&a) || set.contains(&b)
			{
				set.insert(a);
				set.insert(b);
			}
		}
		if set.len() == len
		{
			break len;
		}
	}
}

fn walk<T>(dist: &mut std::collections::BTreeMap<T, usize>, connections: &std::collections::BTreeSet<(T, T)>) -> usize
	where
		T: Ord + Copy,
{
	let min = connections.first().unwrap().0;
	let max = connections.last().unwrap().0;
	let current = dist.values().max().unwrap() + 1;

	let last_round = dist.iter()
		.filter_map(|(&k, &v)| (v == current - 1).then_some(k))
		.collect::<Vec<_>>();

	let next = last_round.iter()
		.flat_map(|&key|
		{
			connections.range((Bound::Included((key, min)), Bound::Included((key, max))))
				.map(|(_, b)| b)
		})
		.filter(|next| !dist.contains_key(next))
		.map(|&next| (next, current))
		.collect::<Vec<_>>();

	if next.is_empty()
	{
		current - 1
	}
	else
	{
		dist.extend(next);
		walk(dist, connections)
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut conns = self.input.lines()
			.map(|line|
			{
				let (left, right) = line.split_once(": ").ok_or(anyhow!("cannot split on colon"))?;
				Ok(right.split(' ').flat_map(move |right| [(left, right), (right, left)]))
			})
			.collect::<Result<Vec<_>>>()?
			.into_iter()
			.flatten()
			.inspect(|(l, r)| trace!("inserting connection between {:?} and {:?}", l, r))
			.collect::<std::collections::BTreeSet<_>>();

		if log::log_enabled!(log::Level::Trace)
		{
			let repr = conns.iter().map(|(l, r)| format!("\t{} -- {};", l, r)).collect::<Vec<_>>();
			trace!("dot data:\ngraph {{\n{}\n}}", repr.join("\n"));
		}

		let nodes = conns.iter().flat_map(|&(a, b)| [a, b]).collect::<std::collections::BTreeSet<_>>();
		let min_node = *nodes.first().unwrap();
		let max_node = *nodes.last().unwrap();
		let dist = nodes.iter()
			.map(|&n|
			{
				(walk(&mut std::collections::BTreeMap::from([(n, 0)]), &conns), n)
			})
			.inspect(|(dist, n)| trace!("{:?}: max distance: {}", n, dist))
			.collect::<std::collections::BTreeSet<_>>();
		let dist = &dist;

		let min_dist = dist.first().unwrap().0;
		let min_count = dist.range((Bound::Included((min_dist, min_node)), Bound::Included((min_dist, max_node)))).count();
		let remove = dist.range((Bound::Included((min_dist, min_node)), Bound::Included((min_dist, max_node))))
			.map(|&(d, n)|
			{
				let next = conns.range((Bound::Included((n, min_node)), Bound::Included((n, max_node))))
					.map(|(_, b)| b)
					.filter_map(|n|
					{
						dist.iter().find(|(od, on)| n == on && (min_count == 6 || *od > min_dist))
					})
					.min_by_key(|(od, _)| od)
					.unwrap()
					.1;
				debug!("shortest dist ({}) from {:?}: {:?}", d, n, next);
				(n, next)
			})
			.collect::<Vec<_>>();

		for (a, b) in remove.iter()
		{
			conns.remove(&(a, b));
			conns.remove(&(b, a));
		}

		let result: usize = count_cluster(remove[0].0, &conns) * count_cluster(remove[0].1, &conns);

		Ok(format!("{}", result))
	}
}

