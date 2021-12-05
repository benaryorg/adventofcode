use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D20Pt1 as Solution,
/// #     Solution as S,
/// # };
/// let example = "Tile 2311:\n\
/// #   ..##.#..#.\n\
/// #   ##..#.....\n\
/// #   #...##..#.\n\
/// #   ####.#...#\n\
/// #   ##.##.###.\n\
/// #   ##...#.###\n\
/// #   .#.#.#..##\n\
/// #   ..#....#..\n\
/// #   ###...#.#.\n\
/// #   ..###..###\n\
/// #   \n\
/// #   Tile 1951:\n\
/// #   #.##...##.\n\
/// #   #.####...#\n\
/// #   .....#..##\n\
/// #   #...######\n\
/// #   .##.#....#\n\
/// #   .###.#####\n\
/// #   ###.##.##.\n\
/// #   .###....#.\n\
/// #   ..#.#..#.#\n\
/// #   #...##.#..\n\
/// #   \n\
/// #   Tile 1171:\n\
/// #   ####...##.\n\
/// #   #..##.#..#\n\
/// #   ##.#..#.#.\n\
/// #   .###.####.\n\
/// #   ..###.####\n\
/// #   .##....##.\n\
/// #   .#...####.\n\
/// #   #.##.####.\n\
/// #   ####..#...\n\
/// #   .....##...\n\
/// #   \n\
/// #   Tile 1427:\n\
/// #   ###.##.#..\n\
/// #   .#..#.##..\n\
/// #   .#.##.#..#\n\
/// #   #.#.#.##.#\n\
/// #   ....#...##\n\
/// #   ...##..##.\n\
/// #   ...#.#####\n\
/// #   .#.####.#.\n\
/// #   ..#..###.#\n\
/// #   ..##.#..#.\n\
/// #   \n\
/// #   Tile 1489:\n\
/// #   ##.#.#....\n\
/// #   ..##...#..\n\
/// #   .##..##...\n\
/// #   ..#...#...\n\
/// #   #####...#.\n\
/// #   #..#.#.#.#\n\
/// #   ...#.#.#..\n\
/// #   ##.#...##.\n\
/// #   ..##.##.##\n\
/// #   ###.##.#..\n\
/// #   \n\
/// #   Tile 2473:\n\
/// #   #....####.\n\
/// #   #..#.##...\n\
/// #   #.##..#...\n\
/// #   ######.#.#\n\
/// #   .#...#.#.#\n\
/// #   .#########\n\
/// #   .###.#..#.\n\
/// #   ########.#\n\
/// #   ##...##.#.\n\
/// #   ..###.#.#.\n\
/// #   \n\
/// #   Tile 2971:\n\
/// #   ..#.#....#\n\
/// #   #...###...\n\
/// #   #.#.###...\n\
/// #   ##.##..#..\n\
/// #   .#####..##\n\
/// #   .#..####.#\n\
/// #   #..#.#..#.\n\
/// #   ..####.###\n\
/// #   ..#.#.###.\n\
/// #   ...#.#.#.#\n\
/// #   \n\
/// #   Tile 2729:\n\
/// #   ...#.#.#.#\n\
/// #   ####.#....\n\
/// #   ..#.#.....\n\
/// #   ....#..#.#\n\
/// #   .##..##.#.\n\
/// #   .#.####...\n\
/// #   ####.#.#..\n\
/// #   ##.####...\n\
/// #   ##..#.##..\n\
/// #   #.##...##.\n\
/// #   \n\
/// #   Tile 3079:\n\
/// #   #.#.#####.\n\
/// #   .#..######\n\
/// #   ..#.......\n\
/// #   ######....\n\
/// #   ####.#..#.\n\
/// #   .#...#.##.\n\
/// #   #.#####.##\n\
/// #   ..#.###...\n\
/// #   ..#.......\n\
///     ..#.###...\n";
/// assert_eq!(Solution::new(example.to_string()).solve().expect("1"), "20899048083289");
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

fn get_10array<I: IntoIterator<Item=T>,T>(iter: I) -> Result<[T;10]>
{
	let mut iter = iter.into_iter();
	Ok([
		iter.next().ok_or(Error::AocParseError)?,
		iter.next().ok_or(Error::AocParseError)?,
		iter.next().ok_or(Error::AocParseError)?,
		iter.next().ok_or(Error::AocParseError)?,
		iter.next().ok_or(Error::AocParseError)?,
		iter.next().ok_or(Error::AocParseError)?,
		iter.next().ok_or(Error::AocParseError)?,
		iter.next().ok_or(Error::AocParseError)?,
		iter.next().ok_or(Error::AocParseError)?,
		iter.next().ok_or(Error::AocParseError)?,
	])
}

#[derive(Clone,Debug)]
struct Tile
{
	id: usize,
	grid: [[bool;10];10],
}

impl Tile
{
	fn borders(&self) -> [[bool;10];8]
	{
		[
			self.grid[0],
			get_10array(self.grid[0].iter().copied().rev()).unwrap(),
			self.grid[9],
			get_10array(self.grid[9].iter().copied().rev()).unwrap(),
			get_10array(self.grid.iter().map(|row| row[0])).unwrap(),
			get_10array(self.grid.iter().map(|row| row[0]).rev()).unwrap(),
			get_10array(self.grid.iter().map(|row| row[9])).unwrap(),
			get_10array(self.grid.iter().map(|row| row[9]).rev()).unwrap(),
		]
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let tiles = self.input.split("\n\n")
			.filter(|s| !s.is_empty())
			.map(|split|
			{
				let mut lines = split.lines();
				let header = lines.next().ok_or(Error::AocParseError)?;
				let id = header.chars().skip(5).take(4).collect::<String>().parse()?;
				let grid = get_10array(lines
					.map(|line|
					{
						get_10array(line.chars().map(|ch| ch == '#'))
					})
					.collect::<Result<Vec<_>>>()?)?;
				Ok(Tile
				{
					id,
					grid,
				})
			})
			.collect::<Result<Vec<Tile>>>()?;

		let map = tiles.iter()
			.cloned()
			.flat_map(|tile|
			{
				tile.borders()
					.iter()
					.cloned()
					.map(
					{
						move |border| (tile.id, border)
					})
					.collect::<Vec<_>>()
			})
			.fold(std::collections::HashMap::<[bool;10],std::collections::BTreeSet<usize>>::new(), |mut map, (id, border)|
			{
				let key = border.min(get_10array(border.iter().rev().copied()).unwrap());
				map.entry(key).or_default().insert(id);
				map
			});

		let unmatched = map.values()
			.filter_map(|set| if set.len() == 1 { set.iter().next() } else { None })
			.fold(std::collections::BTreeMap::<usize,usize>::new(), |mut map, &id|
			{
				map.entry(id)
					.and_modify(|val| *val += 1)
					.or_insert(1);
				map
			});

		let corners = unmatched.iter()
			.filter(|&(_,&count)| count == 2)
			.map(|(id,_)| id)
			.copied()
			.collect::<Vec<_>>();

		if corners.len() != 4
		{
			bail!(Error::AocNoSolution);
		}
		else
		{
			let result = corners.iter().product::<usize>();
			Ok(format!("{}", result))
		}
	}
}


