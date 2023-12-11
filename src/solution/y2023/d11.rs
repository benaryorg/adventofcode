use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d11::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     ...#......\n\
///     .......#..\n\
///     #.........\n\
///     ..........\n\
///     ......#...\n\
///     .#........\n\
///     .........#\n\
///     ..........\n\
///     .......#..\n\
///     #...#.....";
/// assert_eq!(Solution::with_empty(2, input.to_string()).solve().unwrap(), "374");
/// assert_eq!(Solution::with_empty(10, input.to_string()).solve().unwrap(), "1030");
/// assert_eq!(Solution::with_empty(100, input.to_string()).solve().unwrap(), "8410");
/// ```
pub struct Solution
{
	empty: usize,
	input: String,
}

impl Solution
{
	pub fn with_empty(empty: usize, input: String) -> Self
	{
		Self { empty, input, }
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut map = self.input.lines()
			.map(|line|
			{
				line.chars()
					.map(|ch| (ch == '#').then_some(0))
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();

		map.iter_mut()
			.flat_map(|row| row.iter_mut())
			.flat_map(|option| option.iter_mut())
			.enumerate()
			.for_each(|(idx, galaxy)| *galaxy = idx + 1);

		let empty_cols = &(0..map[0].len())
			.filter(|x|
			{
				map.iter().all(|row| row.get(*x).unwrap().is_none())
			})
			.collect::<Vec<_>>();

		debug!("empty cols: {:?}", empty_cols);

		/*empty_cols.into_iter()
			.enumerate()
			.for_each(|(offset, x)|
			{
				map.iter_mut().for_each(|row| row.insert(x + offset, None));
			});*/

		let empty_rows = &map.iter()
			.enumerate()
			.filter(|(_, row)| row.iter().all(Option::is_none))
			.map(|(idx, _)| idx)
			.collect::<Vec<_>>();

		debug!("empty rows: {:?}", empty_rows);

		/*empty_rows.into_iter()
			.enumerate()
			.map(|(a, b)| a + b)
			.for_each(|idx| map.insert(idx, map.get(idx).unwrap().clone()));*/

		trace!("galaxy:\n{:?}", map);

		let galaxies = map.into_iter()
			.enumerate()
			.flat_map(|(y, row)|
			{
				row.into_iter()
					.enumerate()
					.filter_map(move |(x, opt)| opt.map(|_| (x, y)))
			})
			.collect::<Vec<(usize, usize)>>();

		debug!("galaxies:\n{:?}", galaxies);

		let result: usize = galaxies.iter()
			.enumerate()
			.flat_map(|(me, my_pos)|
			{
				galaxies.iter()
					.enumerate()
					.skip(me + 1)
					.map(move |(other, other_pos)|
					{
						let x = my_pos.0.min(other_pos.0)..my_pos.0.max(other_pos.0);
						let y = my_pos.1.min(other_pos.1)..my_pos.1.max(other_pos.1);
						let x_corr = empty_cols.iter().filter(|idx| x.contains(idx)).count() * (self.empty - 1);
						let y_corr = empty_rows.iter().filter(|idx| y.contains(idx)).count() * (self.empty - 1);
						let dist = x.count() + y.count() + x_corr + y_corr;
						trace!("distance between {} ({:?}) and {} ({:?}) is {}", me, my_pos, other, other_pos, dist);
						dist
					})
			})
			.sum();

		Ok(format!("{}", result))
	}
}

