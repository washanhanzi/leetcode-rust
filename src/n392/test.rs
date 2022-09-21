use crate::n392::is_subsequence::is_subsequence_naive;

struct TestData {
    name: String,
    input: [String; 2],
    output: bool,
}

#[test]
fn run_test() {
    let data = [
        TestData {
            name: String::from("case1"),
            input: [String::from("abc"), String::from("ahbgdc")],
            output: true,
        },
        TestData {
            name: String::from("case2"),
            input: [String::from("axc"), String::from("ahbgdc")],
            output: false,
        },
    ];
    for d in data {
        let [s, t] = d.input;
        assert_eq!(
            is_subsequence_naive(s, t),
            d.output,
            "Testing: {:?}",
            d.name
        )
    }
}
