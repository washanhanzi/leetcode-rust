// The API is_bad_version is defined for you.
// is_bad_version(version:i32)-> bool;
// to call it use self.is_bad_version(version)

struct ProductTest {}

impl ProductTest {
    fn is_bad_version(&self, _: i32) -> bool {
        true
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        while low < high {
            //mid = (low + high)/2 will cause overflow
            let mid = low + (high - low) / 2;
            if self.is_bad_version(mid) {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}
