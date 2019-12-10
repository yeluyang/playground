use super::*;

#[test]
fn test_search() {
    // type of test cases
    struct TestCase {
        // input
        query: &'static str,
        text: &'static str,
        ignore_case: bool,
        // output
        results: Vec<&'static str>,
    }
    // table of test cases
    let cases = [
        TestCase {
            query: "query",
            text: "Search one query in one line",
            ignore_case: false,
            results: vec!["Search one query in one line"],
        },
        TestCase {
            query: "query",
            text: "Search multiple query,
in multiple lines,
which contains multiple query",
            ignore_case: false,
            results: vec!["Search multiple query,", "which contains multiple query"],
        },
        TestCase {
            query: " ",
            text: "Search blank chars,\nin,\nmultiple lines",
            ignore_case: false,
            results: vec!["Search blank chars,", "multiple lines"],
        },
        TestCase {
            query: "query",
            text: "SEARCH LOWER QUERY IN UPPER TEXT, WHEN NOT IGNORE CASE",
            ignore_case: false,
            results: vec![],
        },
        TestCase {
            query: "QUERY",
            text: "Search upper query in lower text, when not ignore case",
            ignore_case: false,
            results: vec![],
        },
        TestCase {
            query: "query",
            text: "SEARCH LOWER QUERY IN UPPER TEXT, WHEN IGNORE CASE",
            ignore_case: true,
            results: vec!["SEARCH LOWER QUERY IN UPPER TEXT, WHEN IGNORE CASE"],
        },
        TestCase {
            query: "QUERY",
            text: "Search upper query in lower text, when ignore case",
            ignore_case: true,
            results: vec!["Search upper query in lower text, when ignore case"],
        },
    ];

    // test
    for case in cases.iter() {
        let results = search(case.query, case.text, case.ignore_case);
        assert_eq!(case.results, results);
    }
}
