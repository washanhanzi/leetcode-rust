// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

struct ProductTest {}

impl ProductTest {
    fn isBadVersion(&self, version: i32) -> bool {
        true
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        while low < high {
            //mid = (low + high)/2 will cause overflow
            let mid = low + (high - low) / 2;
            if self.isBadVersion(mid) {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}
