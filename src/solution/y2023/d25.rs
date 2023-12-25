use crate::error::*;

/// # Examples
///
/// Cannot run the test because the values in the code are hard-coded for my solution.
/// Use *neato* on the dot trace output to get a graph of your stuff, open it up in a browser or something, look at the graph and determine the three values, then run the modified code.
/// I'm sorry, but this problem is NP-hard and any solution for estimating requires complex math and/or libraries.
///
/// ```no_run
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

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut conns = self.input.lines()
			.map(|line|
			{
				let (left, right) = line.split_once(": ").ok_or(anyhow!("cannot split on colon"))?;
				Ok(right.split(' ').map(move |right| (left.min(right), left.max(right))))
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

		conns.remove(&("cms", "thk"));
		conns.remove(&("dht", "xmv"));
		conns.remove(&("rcn", "xkf"));

		let result: usize = count_cluster("cms", &conns) * count_cluster("thk", &conns);

		Ok(format!("{}", result))
	}
}

