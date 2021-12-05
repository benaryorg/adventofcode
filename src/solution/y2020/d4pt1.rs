use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D4Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
///     byr:1937 iyr:2017 cid:147 hgt:183cm\n\
///     \n\
///     iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
///     hcl:#cfa07d byr:1929\n\
///     \n\
///     hcl:#ae17e1 iyr:2013\n\
///     eyr:2024\n\
///     ecl:brn pid:760753108 byr:1931\n\
///     hgt:179cm\n\
///     \n\
///     hcl:#cfa07d eyr:2025 pid:166559648\n\
///     iyr:2011 ecl:brn hgt:59in";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "2");
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
	pid: String,
	eyr: String,
	hcl: String,
	byr: String,
	iyr: String,
	cid: Option<String>,
	hgt: String,
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
			ecl: kv.get("ecl").ok_or(Error::AocParseError)?.to_string(),
			pid: kv.get("pid").ok_or(Error::AocParseError)?.to_string(),
			eyr: kv.get("eyr").ok_or(Error::AocParseError)?.to_string(),
			hcl: kv.get("hcl").ok_or(Error::AocParseError)?.to_string(),
			byr: kv.get("byr").ok_or(Error::AocParseError)?.to_string(),
			iyr: kv.get("iyr").ok_or(Error::AocParseError)?.to_string(),
			cid: kv.get("cid").map(|s| s.to_string()),
			hgt: kv.get("hgt").ok_or(Error::AocParseError)?.to_string(),
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

