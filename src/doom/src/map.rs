pub mod utils {
	//
	// P_AproxDistance
	// Gives an estimation of distance (not exact)
	//

	use std::ops::Sub;
	use common::{fixed::fixed, vector::concrete::vec2};

	pub trait AproxDistance
		where Self: Sub<Output = Self> + Sized
	{
		fn aprox_length(self) -> fixed;
		
		fn aprox_distance_to(self, other: Self) -> fixed {
			(other - self).aprox_length()
		}
	}

	impl AproxDistance for vec2 {
		fn aprox_length(self) -> fixed {
			let dx = self.x.abs();
			let dy = self.y.abs();

			if dx < dy {
				dx + dy - dx / 2
			} else {
				dx + dy - dy / 2
			}
		}
	}
}