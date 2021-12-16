use std::fmt;

/// The numeric value corresponds to the number of points won.
#[derive(Debug)]
pub enum ScoreValue {
	Love = 0,
	Fifteen = 1,
	Thirty = 2,
	Forty = 3,
	Ad = 4,
}

impl fmt::Display for ScoreValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match *self {
			Self::Love => "0",
			Self::Fifteen => "15",
			Self::Thirty => "30",
			Self::Forty => "40",
			Self::Ad => "Ad",
		};
		write!(f, "{}", s)
	}
}

#[derive(Debug)]
pub enum Score {
	Love,
	LoveFifteen,
	LoveThirty,
	LoveForty,
	FifteenLove,
	FifteenAll,
	FifteenThirty,
	FifteenForty,
	ThirtyLove,
	ThirtyFifteen,
	ThirtyAll,
	ThirtyForty,
	FortyLove,
	FortyFifteen,
	FortyThirty,
	Deuce,
	AdIn,
	AdOut,
}

impl Score {
	pub fn to_score_values(&self) -> (ScoreValue, ScoreValue) {
		match &self {
			Self::Love => (ScoreValue::Love, ScoreValue::Love),
			Self::LoveFifteen => (ScoreValue::Love, ScoreValue::Fifteen),
			Self::LoveThirty => (ScoreValue::Love, ScoreValue::Thirty),
			Self::LoveForty => (ScoreValue::Love, ScoreValue::Forty),
			Self::FifteenLove => (ScoreValue::Fifteen, ScoreValue::Love),
			Self::FifteenAll => (ScoreValue::Fifteen, ScoreValue::Fifteen),
			Self::FifteenThirty => (ScoreValue::Fifteen, ScoreValue::Thirty),
			Self::FifteenForty => (ScoreValue::Fifteen, ScoreValue::Forty),
			Self::ThirtyLove => (ScoreValue::Thirty, ScoreValue::Love),
			Self::ThirtyFifteen => (ScoreValue::Thirty, ScoreValue::Fifteen),
			Self::ThirtyAll => (ScoreValue::Thirty, ScoreValue::Thirty),
			Self::ThirtyForty => (ScoreValue::Thirty, ScoreValue::Forty),
			Self::FortyLove => (ScoreValue::Forty, ScoreValue::Love),
			Self::FortyFifteen => (ScoreValue::Forty, ScoreValue::Fifteen),
			Self::FortyThirty => (ScoreValue::Forty, ScoreValue::Thirty),
			Self::Deuce => (ScoreValue::Forty, ScoreValue::Forty),
			Self::AdIn => (ScoreValue::Ad, ScoreValue::Forty),
			Self::AdOut => (ScoreValue::Forty, ScoreValue::Ad),
		}
	}
}

impl fmt::Display for Score {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let val_s;
		let s = match &*self {
			Self::AdIn => "Ad In",
			Self::AdOut => "Ad Out",
			_ => {
				let v = self.to_score_values();
				val_s = format!("{}\u{2013}{}", v.0, v.1);
				&val_s
			}
		};
		write!(f, "{}", s)
	}
}

#[cfg(test)]
mod tests {
	use super::Score;

	#[test]
	fn display_love() {
		assert_eq!(format!("{}", Score::Love), "0\u{2013}0".to_string());
	}

	#[test]
	fn display_fifteen_love() {
		assert_eq!(format!("{}", Score::FifteenLove), "15\u{2013}0".to_string());
	}

	#[test]
	fn display_deuce() {
		assert_eq!(format!("{}", Score::Deuce), "40\u{2013}40".to_string());
	}

	#[test]
	fn display_ad_in() {
		assert_eq!(format!("{}", Score::AdIn), "Ad In".to_string());
	}

	#[test]
	fn display_ad_out() {
		assert_eq!(format!("{}", Score::AdOut), "Ad Out".to_string());
	}
}
