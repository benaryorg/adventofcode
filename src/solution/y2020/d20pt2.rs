use crate::error::*;

/// # Examples
///
/// ```no_run
/// # // FIXME: currently broken
/// # use adventofcode::solution::
/// # {
/// #     y2020::D20Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
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
/// assert_eq!(Solution::new(example.to_string()).solve().expect("1"), "273");
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

#[derive(Clone,Debug,Eq,PartialEq)]
struct Tile
{
	id: usize,
	grid: [[bool;10];10],
}

impl Tile
{
	fn top(&self) -> [[bool;10];2]
	{
		[
			self.grid[0],
			get_10array(self.grid[0].iter().copied().rev()).unwrap(),
		]
	}

	fn bottom(&self) -> [[bool;10];2]
	{
		[
			get_10array(self.grid[9].iter().copied().rev()).unwrap(),
			self.grid[9],
		]
	}

	fn left(&self) -> [[bool;10];2]
	{
		[
			get_10array(self.grid.iter().map(|row| row[0]).rev()).unwrap(),
			get_10array(self.grid.iter().map(|row| row[0])).unwrap(),
		]
	}

	fn right(&self) -> [[bool;10];2]
	{
		[
			get_10array(self.grid.iter().map(|row| row[9])).unwrap(),
			get_10array(self.grid.iter().map(|row| row[9]).rev()).unwrap(),
		]
	}

	fn borders(&self) -> [[bool;10];8]
	{
		[
			self.top()[0],
			self.top()[1],
			self.right()[0],
			self.right()[1],
			self.bottom()[0],
			self.bottom()[1],
			self.left()[0],
			self.left()[1],
		]
	}

	fn flip(&mut self)
	{
		for vec in &mut self.grid
		{
			vec.reverse();
		}
	}

	fn rotate(&mut self)
	{
		self.grid = get_10array(self.grid[9].iter().copied()
			.zip(self.grid[8].iter().copied())
			.zip(self.grid[7].iter().copied())
			.zip(self.grid[6].iter().copied())
			.zip(self.grid[5].iter().copied())
			.zip(self.grid[4].iter().copied())
			.zip(self.grid[3].iter().copied())
			.zip(self.grid[2].iter().copied())
			.zip(self.grid[1].iter().copied())
			.zip(self.grid[0].iter().copied())
			.map(|(((((((((it0,it1),it2),it3),it4),it5),it6),it7),it8),it9)| [it0,it1,it2,it3,it4,it5,it6,it7,it8,it9]))
			.unwrap();
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
			.map(|(&id,_)| tiles.iter().find(|&tile| tile.id == id).unwrap())
			.cloned()
			.collect::<Vec<_>>();

		let edges = unmatched.iter()
			.filter(|&(_,&count)| count == 1)
			.map(|(&id,_)| tiles.iter().find(|&tile| tile.id == id).unwrap())
			.cloned()
			.collect::<Vec<_>>();

		debug!("corners: {}", corners.len());
		debug!("edges: {}", edges.len());

		if corners.len() != 4 || edges.len() != 4
		{
			bail!(Error::AocNoSolution);
		}

		let mut big_grid = [[false;30];30];

		let fill_at = |small: &[[bool;10];10],big: &mut [[bool;30];30],offx: isize,offy: isize|
		{
			for (y,line) in small.iter().enumerate()
			{
				if y as isize + offy < 0 || y + offy as usize >= big.len()
				{
					continue;
				}
				for (x,v) in line.iter().enumerate()
				{
					if x as isize + offx < 0 || x + offx as usize >= big[0].len()
					{
						continue;
					}
					big[offy as usize+y][offx as usize+x] = *v;
				}
			}
		};

		let middle = tiles.iter().find(|tile| !corners.contains(tile) && !edges.contains(tile)).ok_or(Error::AocNoSolution)?;
		fill_at(&middle.grid, &mut big_grid, 10, 10);

		let mut top = None;
		let mut bottom = None;

		for edge in edges.iter()
		{
			if let Some(offset) = edge.borders().iter().position(|border| border == &middle.bottom()[0])
			{
				let mut edge = edge.clone();
				if offset % 2 == 0
				{
					edge.flip();
				}
				let offset = edge.borders().iter().position(|border| border == &middle.bottom()[0]).unwrap();
				for _ in 0..(offset/2)
				{
					edge.rotate();
				}
				fill_at(&edge.grid, &mut big_grid, 10, 20);
				bottom = Some(edge);
			}
			if let Some(offset) = edge.borders().iter().position(|border| border == &middle.top()[0])
			{
				let mut edge = edge.clone();
				if offset % 2 == 0
				{
					edge.flip();
				}
				let offset = edge.borders().iter().position(|border| border == &middle.top()[0]).unwrap();
				for _ in 0..((offset+4)/2)
				{
					edge.rotate();
				}
				fill_at(&edge.grid, &mut big_grid, 10, 0);
				top = Some(edge);
			}
			if let Some(offset) = edge.borders().iter().position(|border| border == &middle.right()[0])
			{
				let mut edge = edge.clone();
				if offset % 2 == 0
				{
					edge.flip();
				}
				let offset = edge.borders().iter().position(|border| border == &middle.right()[0]).unwrap();
				for _ in 0..((offset+6)/2)
				{
					edge.rotate();
				}
				fill_at(&edge.grid, &mut big_grid, 20, 10);
			}
			if let Some(offset) = edge.borders().iter().position(|border| border == &middle.left()[0])
			{
				let mut edge = edge.clone();
				if offset % 2 == 0
				{
					edge.flip();
				}
				let offset = edge.borders().iter().position(|border| border == &middle.left()[0]).unwrap();
				for _ in 0..((offset+6)/2)
				{
					edge.rotate();
				}
				fill_at(&edge.grid, &mut big_grid, 0, 10);
			}
		}

		debug!("{}",big_grid.chunks(10)
			.map(|chunk|
			{
				chunk.iter()
					.map(|line|
					{
						line.chunks(10)
							.map(|chunk|
							{
								chunk.iter()
									.map(|&b| if b { '#' } else { '.' })
									.collect::<String>()
							})
							.collect::<Vec<String>>()
							.join(" ")
					})
					.collect::<Vec<_>>()
					.join("\n")
			})
			.collect::<Vec<_>>()
			.join("\n\n"));

		let top = top.ok_or(Error::AocNoSolution)?;
		let bottom = bottom.ok_or(Error::AocNoSolution)?;

		for corner in corners.iter()
		{
			if let Some(offset) = corner.borders().iter().position(|border| border == &top.left()[0])
			{
				let mut corner = corner.clone();
				if offset % 2 == 0
				{
					corner.flip();
				}
				let offset = corner.borders().iter().position(|border| border == &top.left()[0]).unwrap();
				for _ in 0..((offset+6)/2)
				{
					corner.rotate();
				}
				fill_at(&corner.grid, &mut big_grid, 0, 0);
			}
			if let Some(offset) = corner.borders().iter().position(|border| border == &bottom.left()[0])
			{
				let mut corner = corner.clone();
				if offset % 2 == 0
				{
					corner.flip();
				}
				let offset = corner.borders().iter().position(|border| border == &bottom.left()[0]).unwrap();
				for _ in 0..((offset+6)/2)
				{
					corner.rotate();
				}
				fill_at(&corner.grid, &mut big_grid, 0, 20);
			}
			if let Some(offset) = corner.borders().iter().position(|border| border == &top.right()[0])
			{
				let mut corner = corner.clone();
				if offset % 2 == 0
				{
					corner.flip();
				}
				let offset = corner.borders().iter().position(|border| border == &top.right()[0]).unwrap();
				for _ in 0..((offset+2)/2)
				{
					corner.rotate();
				}
				fill_at(&corner.grid, &mut big_grid, 20, 0);
			}
			if let Some(offset) = corner.borders().iter().position(|border| border == &bottom.right()[0])
			{
				let mut corner = corner.clone();
				if offset % 2 == 0
				{
					corner.flip();
				}
				let offset = corner.borders().iter().position(|border| border == &bottom.right()[0]).unwrap();
				for _ in 0..((offset+2)/2)
				{
					corner.rotate();
				}
				fill_at(&corner.grid, &mut big_grid, 20, 20);
			}
		}

		debug!("{}",big_grid.chunks(10)
			.map(|chunk|
			{
				chunk.iter()
					.map(|line|
					{
						line.chunks(10)
							.map(|chunk|
							{
								chunk.iter()
									.map(|&b| if b { '#' } else { '.' })
									.collect::<String>()
							})
							.collect::<Vec<String>>()
							.join(" ")
					})
					.collect::<Vec<_>>()
					.join("\n")
			})
			.collect::<Vec<_>>()
			.join("\n\n"));

		let small_grid = big_grid[1..9].iter().chain(big_grid[11..19].iter()).chain(big_grid[21..29].iter()).copied()
			.map(|line| line[1..9].iter().chain(line[11..19].iter()).chain(line[21..29].iter()).copied().collect::<Vec<_>>())
			.collect::<Vec<_>>();

		debug!("{}",small_grid.chunks(8)
			.map(|chunk|
			{
				chunk.iter()
					.map(|line|
					{
						line.chunks(8)
							.map(|chunk|
							{
								chunk.iter()
									.map(|&b| if b { '#' } else { '.' })
									.collect::<String>()
							})
							.collect::<Vec<String>>()
							.join(" ")
					})
					.collect::<Vec<_>>()
					.join("\n")
			})
			.collect::<Vec<_>>()
			.join("\n\n"));

		let monster = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ".lines()
			.map(|line| line.chars().map(|ch| ch == '#').collect::<Vec<_>>())
			.collect::<Vec<_>>();
		let monster_rot = monster[0].iter().copied().zip(monster[1].iter().copied()).zip(monster[2].iter().copied())
			.map(|((a,b),c)| vec![a,b,c])
			.collect::<Vec<_>>();

		/*(0..(small_grid.len()-monster.len()))
			.flat_map(|yoff|
			{
				(0..(small_grid[0].len()-monster[0].len()))
					.map(|xoff|
					{
						let matching = (0..monster.len())
							.flat_map(|y|
							{
								vec!
								[
									(0..monster[0].len())
									.flat_map(|x|
									{
										let grid_pos = (x+xoff,y+yoff);
										let monster_pos = (x,y);

									})
									.filter_map(|((grid_x,grid_y),(mon_x,mon_y))|
									{
										if small_grid[y]
									})
								]
							})
							.collect::<Vec<_>>();

					})
			})
*/
		let coords_normal = small_grid.windows(3)
			.enumerate()
			.flat_map(|(y,s): (usize,&[Vec<bool>])|
			{
				s[0].windows(monster[0].len()).collect::<Vec<_>>().iter().zip(s[1].windows(monster[0].len()).collect::<Vec<_>>()).zip(s[2].windows(monster[0].len()).collect::<Vec<_>>())
					.enumerate()
					.flat_map(
					{
						let monster = monster.clone();
						move |(xoff,((s0,s1),s2))|
						{
							vec!
							[
								std::iter::empty()
									.chain(s0.iter().copied().zip(monster[0].iter().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y),b)))
									.chain(s1.iter().copied().zip(monster[1].iter().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y+1),b)))
									.chain(s2.iter().copied().zip(monster[2].iter().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y+2),b)))
									.collect::<Vec<_>>(),
								std::iter::empty()
									.chain(s0.iter().copied().zip(monster[0].iter().rev().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y),b)))
									.chain(s1.iter().copied().zip(monster[1].iter().rev().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y+1),b)))
									.chain(s2.iter().copied().zip(monster[2].iter().rev().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y+2),b)))
									.collect::<Vec<_>>(),
								std::iter::empty()
									.chain(s0.iter().copied().zip(monster[2].iter().rev().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y),b)))
									.chain(s1.iter().copied().zip(monster[1].iter().rev().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y+1),b)))
									.chain(s2.iter().copied().zip(monster[0].iter().rev().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y+2),b)))
									.collect::<Vec<_>>(),
								std::iter::empty()
									.chain(s0.iter().copied().zip(monster[2].iter().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y),b)))
									.chain(s1.iter().copied().zip(monster[1].iter().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y+1),b)))
									.chain(s2.iter().copied().zip(monster[0].iter().copied().collect::<Vec<_>>()).enumerate().map(move |(x,b)| ((x+xoff,y+2),b)))
									.collect::<Vec<_>>(),
							]
						}
					})
					.collect::<Vec<_>>()
			})
			.filter_map(|vec|
			{
				trace!("{:?}", vec[0].0);
				if vec[0].0 == (2,2)
				{
					debug!("{:?}",vec);
				}
				let filtered = vec.into_iter().filter(|&(_,(_sea, monster))| monster).collect::<Vec<_>>();
				if filtered.iter().all(|(_,(sea, monster))| sea & monster)
				{
					Some(filtered.iter().map(|&(coords,_)| coords).collect::<Vec<_>>())
				}
				else
				{
					None
				}
			})
			.flatten()
			.collect::<Vec<(usize,usize)>>();

		debug!("coords_normal: {}", coords_normal.len());

		let coords_rot = small_grid.windows(20)
			.enumerate()
			.flat_map(|(y,s)|
			{
				vec!
				[
					std::iter::empty()
						.chain(s[0].iter().copied().zip(monster_rot[0].iter().copied()).enumerate().map(|(x,b)| ((x,y),b)))
						.chain(s[1].iter().copied().zip(monster_rot[1].iter().copied()).enumerate().map(|(x,b)| ((x,y+1),b)))
						.chain(s[2].iter().copied().zip(monster_rot[2].iter().copied()).enumerate().map(|(x,b)| ((x,y+2),b)))
						.chain(s[3].iter().copied().zip(monster_rot[3].iter().copied()).enumerate().map(|(x,b)| ((x,y+3),b)))
						.chain(s[4].iter().copied().zip(monster_rot[4].iter().copied()).enumerate().map(|(x,b)| ((x,y+4),b)))
						.chain(s[5].iter().copied().zip(monster_rot[5].iter().copied()).enumerate().map(|(x,b)| ((x,y+5),b)))
						.chain(s[6].iter().copied().zip(monster_rot[6].iter().copied()).enumerate().map(|(x,b)| ((x,y+6),b)))
						.chain(s[7].iter().copied().zip(monster_rot[7].iter().copied()).enumerate().map(|(x,b)| ((x,y+7),b)))
						.chain(s[8].iter().copied().zip(monster_rot[8].iter().copied()).enumerate().map(|(x,b)| ((x,y+8),b)))
						.chain(s[9].iter().copied().zip(monster_rot[9].iter().copied()).enumerate().map(|(x,b)| ((x,y+9),b)))
						.chain(s[10].iter().copied().zip(monster_rot[10].iter().copied()).enumerate().map(|(x,b)| ((x,y+10),b)))
						.chain(s[11].iter().copied().zip(monster_rot[11].iter().copied()).enumerate().map(|(x,b)| ((x,y+11),b)))
						.chain(s[12].iter().copied().zip(monster_rot[12].iter().copied()).enumerate().map(|(x,b)| ((x,y+12),b)))
						.chain(s[13].iter().copied().zip(monster_rot[13].iter().copied()).enumerate().map(|(x,b)| ((x,y+13),b)))
						.chain(s[14].iter().copied().zip(monster_rot[14].iter().copied()).enumerate().map(|(x,b)| ((x,y+14),b)))
						.chain(s[15].iter().copied().zip(monster_rot[15].iter().copied()).enumerate().map(|(x,b)| ((x,y+15),b)))
						.chain(s[16].iter().copied().zip(monster_rot[16].iter().copied()).enumerate().map(|(x,b)| ((x,y+16),b)))
						.chain(s[17].iter().copied().zip(monster_rot[17].iter().copied()).enumerate().map(|(x,b)| ((x,y+17),b)))
						.chain(s[18].iter().copied().zip(monster_rot[18].iter().copied()).enumerate().map(|(x,b)| ((x,y+18),b)))
						.chain(s[19].iter().copied().zip(monster_rot[19].iter().copied()).enumerate().map(|(x,b)| ((x,y+19),b)))
						.collect::<Vec<_>>(),
					std::iter::empty()
						.chain(s[0].iter().copied().zip(monster_rot[0].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y),b)))
						.chain(s[1].iter().copied().zip(monster_rot[1].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+1),b)))
						.chain(s[2].iter().copied().zip(monster_rot[2].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+2),b)))
						.chain(s[3].iter().copied().zip(monster_rot[3].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+3),b)))
						.chain(s[4].iter().copied().zip(monster_rot[4].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+4),b)))
						.chain(s[5].iter().copied().zip(monster_rot[5].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+5),b)))
						.chain(s[6].iter().copied().zip(monster_rot[6].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+6),b)))
						.chain(s[7].iter().copied().zip(monster_rot[7].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+7),b)))
						.chain(s[8].iter().copied().zip(monster_rot[8].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+8),b)))
						.chain(s[9].iter().copied().zip(monster_rot[9].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+9),b)))
						.chain(s[10].iter().copied().zip(monster_rot[10].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+10),b)))
						.chain(s[11].iter().copied().zip(monster_rot[11].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+11),b)))
						.chain(s[12].iter().copied().zip(monster_rot[12].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+12),b)))
						.chain(s[13].iter().copied().zip(monster_rot[13].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+13),b)))
						.chain(s[14].iter().copied().zip(monster_rot[14].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+14),b)))
						.chain(s[15].iter().copied().zip(monster_rot[15].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+15),b)))
						.chain(s[16].iter().copied().zip(monster_rot[16].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+16),b)))
						.chain(s[17].iter().copied().zip(monster_rot[17].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+17),b)))
						.chain(s[18].iter().copied().zip(monster_rot[18].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+18),b)))
						.chain(s[19].iter().copied().zip(monster_rot[19].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+19),b)))
						.collect::<Vec<_>>(),
					std::iter::empty()
						.chain(s[19].iter().copied().zip(monster_rot[0].iter().copied()).enumerate().map(|(x,b)| ((x,y),b)))
						.chain(s[18].iter().copied().zip(monster_rot[1].iter().copied()).enumerate().map(|(x,b)| ((x,y+1),b)))
						.chain(s[17].iter().copied().zip(monster_rot[2].iter().copied()).enumerate().map(|(x,b)| ((x,y+2),b)))
						.chain(s[16].iter().copied().zip(monster_rot[3].iter().copied()).enumerate().map(|(x,b)| ((x,y+3),b)))
						.chain(s[15].iter().copied().zip(monster_rot[4].iter().copied()).enumerate().map(|(x,b)| ((x,y+4),b)))
						.chain(s[14].iter().copied().zip(monster_rot[5].iter().copied()).enumerate().map(|(x,b)| ((x,y+5),b)))
						.chain(s[13].iter().copied().zip(monster_rot[6].iter().copied()).enumerate().map(|(x,b)| ((x,y+6),b)))
						.chain(s[12].iter().copied().zip(monster_rot[7].iter().copied()).enumerate().map(|(x,b)| ((x,y+7),b)))
						.chain(s[11].iter().copied().zip(monster_rot[8].iter().copied()).enumerate().map(|(x,b)| ((x,y+8),b)))
						.chain(s[10].iter().copied().zip(monster_rot[9].iter().copied()).enumerate().map(|(x,b)| ((x,y+9),b)))
						.chain(s[9].iter().copied().zip(monster_rot[10].iter().copied()).enumerate().map(|(x,b)| ((x,y+10),b)))
						.chain(s[8].iter().copied().zip(monster_rot[11].iter().copied()).enumerate().map(|(x,b)| ((x,y+11),b)))
						.chain(s[7].iter().copied().zip(monster_rot[12].iter().copied()).enumerate().map(|(x,b)| ((x,y+12),b)))
						.chain(s[6].iter().copied().zip(monster_rot[13].iter().copied()).enumerate().map(|(x,b)| ((x,y+13),b)))
						.chain(s[5].iter().copied().zip(monster_rot[14].iter().copied()).enumerate().map(|(x,b)| ((x,y+14),b)))
						.chain(s[4].iter().copied().zip(monster_rot[15].iter().copied()).enumerate().map(|(x,b)| ((x,y+15),b)))
						.chain(s[3].iter().copied().zip(monster_rot[16].iter().copied()).enumerate().map(|(x,b)| ((x,y+16),b)))
						.chain(s[2].iter().copied().zip(monster_rot[17].iter().copied()).enumerate().map(|(x,b)| ((x,y+17),b)))
						.chain(s[1].iter().copied().zip(monster_rot[18].iter().copied()).enumerate().map(|(x,b)| ((x,y+18),b)))
						.chain(s[0].iter().copied().zip(monster_rot[19].iter().copied()).enumerate().map(|(x,b)| ((x,y+19),b)))
						.collect::<Vec<_>>(),
					std::iter::empty()
						.chain(s[19].iter().copied().zip(monster_rot[0].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y),b)))
						.chain(s[18].iter().copied().zip(monster_rot[1].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+1),b)))
						.chain(s[17].iter().copied().zip(monster_rot[2].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+2),b)))
						.chain(s[16].iter().copied().zip(monster_rot[3].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+3),b)))
						.chain(s[15].iter().copied().zip(monster_rot[4].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+4),b)))
						.chain(s[14].iter().copied().zip(monster_rot[5].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+5),b)))
						.chain(s[13].iter().copied().zip(monster_rot[6].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+6),b)))
						.chain(s[12].iter().copied().zip(monster_rot[7].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+7),b)))
						.chain(s[11].iter().copied().zip(monster_rot[8].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+8),b)))
						.chain(s[10].iter().copied().zip(monster_rot[9].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+9),b)))
						.chain(s[9].iter().copied().zip(monster_rot[10].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+10),b)))
						.chain(s[8].iter().copied().zip(monster_rot[11].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+11),b)))
						.chain(s[7].iter().copied().zip(monster_rot[12].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+12),b)))
						.chain(s[6].iter().copied().zip(monster_rot[13].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+13),b)))
						.chain(s[5].iter().copied().zip(monster_rot[14].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+14),b)))
						.chain(s[4].iter().copied().zip(monster_rot[15].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+15),b)))
						.chain(s[3].iter().copied().zip(monster_rot[16].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+16),b)))
						.chain(s[2].iter().copied().zip(monster_rot[17].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+17),b)))
						.chain(s[1].iter().copied().zip(monster_rot[18].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+18),b)))
						.chain(s[0].iter().copied().zip(monster_rot[19].iter().rev().copied()).enumerate().map(|(x,b)| ((x,y+19),b)))
						.collect::<Vec<_>>(),
				]
			})
			.filter_map(|vec|
			{
				let filtered = vec.into_iter().filter(|&(_,(_sea, monster))| monster).collect::<Vec<_>>();
				if filtered.iter().all(|(_,(sea, monster))| sea & monster)
				{
					Some(filtered.iter().map(|&(coords,_)| coords).collect::<Vec<_>>())
				}
				else
				{
					None
				}
			})
			.flatten()
			.collect::<Vec<(usize,usize)>>();

		debug!("coords_rot: {}", coords_rot.len());

		let mut small_grid = small_grid;
		for &(x,y) in coords_normal.iter().chain(coords_rot.iter())
		{
			small_grid[y][x] = false;
		}
		let result = small_grid.iter().flatten().copied().filter(|&b| b).count();

		Ok(format!("{}", result))
	}
}

