use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D4Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
///     hcl:#623a2f\n\
///     \n\
///     eyr:2029 ecl:blu cid:129 byr:1989\n\
///     iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
///     \n\
///     hcl:#888785\n\
///     hgt:164cm byr:2001 iyr:2015 cid:88\n\
///     pid:545766238 ecl:hzl\n\
///     eyr:2022\n\
///     \n\
///     iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
/// let invalid = "eyr:1972 cid:100\n\
///     hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
///     \n\
///     iyr:2019\n\
///     hcl:#602927 eyr:1967 hgt:170cm\n\
///     ecl:grn pid:012533040 byr:1946\n\
///     \n\
///     hcl:dab227 iyr:2012\n\
///     ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
///     \n\
///     hgt:59cm ecl:zzz\n\
///     eyr:2038 hcl:74454a iyr:2023\n\
///     pid:3556412378 byr:2007";
/// assert_eq!(Solution::new(valid.to_string()).solve().expect("1"), "4");
/// assert_eq!(Solution::new(invalid.to_string()).solve().expect("2"), "0");
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

#[derive(Clone,Debug)]
struct Passport
{
	ecl: String,
	pid: usize,
	eyr: usize,
	hcl: [char;6],
	byr: usize,
	iyr: usize,
	cid: Option<usize>,
	hgt: (usize,String),
}

impl std::str::FromStr for Passport
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		let kv = input.split_whitespace()
			.flat_map(|part| part.find(':').map(|idx| part.split_at(idx)))
			.flat_map(|(key,value)| value.strip_prefix(':').map(|value| (key,value)))
			.collect::<std::collections::HashMap<_,_>>();

		Ok(Passport
		{
			ecl: match kv.get("ecl").ok_or(Error::AocParseError)?
			{
				e@&"amb" | e@&"blu" | e@&"brn" | e@&"gry" | e@&"grn" | e@&"hzl" | e@&"oth" => e.to_string(),
				_ => return Err(Error::AocParseError),
			},
			pid:
			{
				let s = kv.get("pid").ok_or(Error::AocParseError)?;
				if s.chars().count() != 9 || s.chars().filter(char::is_ascii_digit).count() != 9
				{
					return Err(Error::AocParseError);
				}
				s.parse()?
			},
			eyr: match kv.get("eyr").ok_or(Error::AocParseError)?.parse()?
			{
				e@2020..=2030 => e,
				_ => return Err(Error::AocParseError),
			},
			hcl:
			{
				let mut chars = kv.get("hcl").ok_or(Error::AocParseError)?.chars();
				if chars.next() != Some('#')
				{
					return Err(Error::AocParseError);
				}
				let vec = chars.collect::<Vec<char>>();
				if vec.len() != 6 || vec.iter().copied().filter(char::is_ascii_hexdigit).count() != 6
				{
					return Err(Error::AocParseError);
				}
				[vec[0],vec[1],vec[2],vec[3],vec[4],vec[5],]
			},
			byr: match kv.get("byr").ok_or(Error::AocParseError)?.parse()?
			{
				e@1920..=2002 => e,
				_ => return Err(Error::AocParseError),
			},
			iyr: match kv.get("iyr").ok_or(Error::AocParseError)?.parse()?
			{
				e@2010..=2020 => e,
				_ => return Err(Error::AocParseError),
			},
			cid: kv.get("cid").map(|s| s.parse()).transpose()?,
			hgt:
			{
				let s = kv.get("hgt").ok_or(Error::AocParseError)?;
				let (num,unit) = s.split_at(s.find(|ch: char| !ch.is_ascii_digit()).ok_or(Error::AocParseError)?);
				let num = num.parse()?;
				if !match unit
				{
					"cm" => (150..=193).contains(&num),
					"in" => (59..=76).contains(&num),
					_ => false,
				}
				{
					return Err(Error::AocParseError);
				}
				(num,unit.to_string())
			},
		})
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("started with input: {}", self.input);

		let num_valid = self.input.split("\n\n")
			.map(|line| line.parse::<Passport>())
			.filter(Result::is_ok)
			.inspect(|passport| trace!("{:?}", passport))
			.count();

		Ok(format!("{}", num_valid))
	}
}

