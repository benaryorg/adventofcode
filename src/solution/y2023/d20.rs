use crate::error::*;

/// # Examples
///
/// Part 1:
///
/// ```
/// # use adventofcode::solution::{ y2023::d20pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     broadcaster -> a, b, c\n\
///     %a -> b\n\
///     %b -> c\n\
///     %c -> inv\n\
///     &inv -> a";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "32000000");
/// ```
pub struct Solution
{
	input: String,
	part: AocPart,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			part: AocPart::One,
			input,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			part: AocPart::Two,
			input,
		}
	}
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum AocPart
{
	One,
	Two,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum ModuleType
{
	Broadcast,
	FlipFlop,
	Conjunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'a>
{
	Broadcast
	{
		target: Vec<&'a String>,
	},
	FlipFlop
	{
		target: Vec<&'a String>,
		state: bool,
	},
	Conjunction
	{
		target: Vec<&'a String>,
		state: std::collections::BTreeMap<&'a String, bool>,
	},
}

#[allow(unused)]
impl<'a> Module<'a>
{
	fn signal(&mut self, pulse: bool, source: &String) -> Vec<(bool, &'a String)>
	{
		match self
		{
			Module::Broadcast { target, } =>
			{
				target.iter().cloned().map(|s| (pulse, s)).collect()
			},
			Module::FlipFlop { target, ref mut state, } =>
			{
				if pulse
				{
					Vec::new()
				}
				else
				{
					*state = !*state;
					target.iter().cloned().map(|s| (*state, s)).collect()
				}
			},
			Module::Conjunction { target, ref mut state, } =>
			{
				*state.get_mut(source).unwrap() = pulse;
				let out = state.values().all(|&b| b);
				target.iter().cloned().map(|s| (!out, s)).collect()
			},
		}
	}

	fn has_target(&self, name: &String) -> bool
	{
		match self
		{
			Module::Broadcast { target, } =>
			{
				target.iter().any(|&s| s == name)
			},
			Module::FlipFlop { target, .. } =>
			{
				target.iter().any(|&s| s == name)
			},
			Module::Conjunction { target, .. } =>
			{
				target.iter().any(|&s| s == name)
			},
		}
	}

	fn bits(&self) -> Vec<bool>
	{
		match self
		{
			Module::Broadcast { .. } =>
			{
				Vec::new()
			},
			Module::FlipFlop { ref state, .. } =>
			{
				vec![*state]
			},
			Module::Conjunction { ref state, .. } =>
			{
				state.values().copied().collect()
			},
		}
	}

	fn bits_mut(&mut self) -> Vec<&mut bool>
	{
		match self
		{
			Module::Broadcast { .. } =>
			{
				Vec::new()
			},
			Module::FlipFlop { ref mut state, .. } =>
			{
				vec![state]
			},
			Module::Conjunction { ref mut state, .. } =>
			{
				state.values_mut().collect()
			},
		}
	}
}

impl<'a> From<(ModuleType, Vec<&'a String>, Vec<&'a String>)> for Module<'a>
{
	fn from((mtype, input, output): (ModuleType, Vec<&'a String>, Vec<&'a String>)) -> Module<'a>
	{
		match mtype
		{
			ModuleType::Broadcast => Module::Broadcast
			{
				target: output,
			},
			ModuleType::FlipFlop => Module::FlipFlop
			{
				target: output,
				state: false,
			},
			ModuleType::Conjunction => Module::Conjunction
			{
				target: output,
				state: input.into_iter().map(|s| (s, false)).collect(),
			},
		}
	}
}

fn simulate(modules: &mut std::collections::BTreeMap<&String, Module>) -> Vec<(String, bool, String)>
{
	let broadcaster = "broadcaster".to_string();
	let button = "button".to_string();

	let mut queue = std::collections::VecDeque::new();
	let mut out = Vec::new();

	queue.push_back((broadcaster, false, button));

	while let Some((target, pulse, source)) = queue.pop_front()
	{
		trace!("queue len: {}", queue.len());
		trace!("processing {} from {:?} to {:?}", pulse, source, target);
		if let Some(module) = modules.get_mut(&target)
		{
			queue.extend(module
				.signal(pulse, &source)
				.into_iter()
				.map(|(pulse, t)| (t.clone(), pulse, target.clone()))
				.inspect(|(target, pulse, source)| trace!("injecting {} from {:?} to {:?}", pulse, source, target))
			);
		}
		out.push((target, pulse, source));
	}

	out
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let modules = self.input.lines()
			.map(|line|
			{
				trace!("parsing line {:?}", line);
				let (name, target) = line.split_once(" -> ").ok_or(anyhow!("no arrow in line"))?;
				let (mtype, name) = None
					.or_else(||
					{
						name.strip_prefix('%')
							.map(|name| (ModuleType::FlipFlop, name.to_string()))
					})
					.or_else(||
					{
						name.strip_prefix('&')
							.map(|name| (ModuleType::Conjunction, name.to_string()))
					})
					.unwrap_or_else(|| (ModuleType::Broadcast, name.to_string()));
				let target = target.split(", ").map(str::to_string).collect::<Vec<String>>();
				Ok((mtype, name, target))
			})
			.chain([Ok((ModuleType::Broadcast, "rx".to_string(), Vec::new()))])
			.collect::<Result<Vec<_>>>()?;

		let mut modules = modules.iter()
			.map(|(mtype, name, target)|
			{
				let input = modules.iter()
					.filter(|(_, _, target)| target.contains(name))
					.map(|(_, name, _)| name)
					.collect::<Vec<_>>();
				let target = target.iter().collect();
				let module = (*mtype, input, target).into();
				trace!("new module {:?}: {:?}", name, module);
				(name, module)
			})
			.collect::<std::collections::BTreeMap<&String, Module>>();

		let result: usize = match self.part
		{
			AocPart::One =>
			{
				(1..=1000)
					.map(|i|
					{
						debug!("run {}", i);
						simulate(&mut modules)
					})
					.map(|out|
					{
						let high = out.iter().filter(|(_, pulse, _)| *pulse).count();
						(out.len() - high, high)
					})
					.reduce(|(low1, high1), (low2, high2)| (low1 + low2, high1 + high2))
					.map(|(a, b)| a * b)
					.unwrap()
			},
			AocPart::Two =>
			{
				let prerx = modules.iter().find_map(|(n, m)| m.has_target(&"rx".to_string()).then_some(n)).unwrap().to_string();
				let len = modules.get(&prerx).unwrap().bits().len();
				let mut freqs = std::collections::BTreeMap::<String, usize>::new();
				for i in 1..
				{
					trace!("run {}", i);
					simulate(&mut modules)
						.into_iter()
						.filter(|(target, pulse, _)| *target == prerx && *pulse)
						.for_each(|(_, _, source)|
						{
							freqs.entry(source.clone()).or_insert(i);
							debug!("found bit at {}", i);
						});
					if freqs.len() == len
					{
						break;
					}
				}
				freqs.into_values().reduce(num::integer::lcm).unwrap()
			},
		};

		Ok(format!("{}", result))
	}
}

