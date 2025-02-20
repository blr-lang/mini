use std::path::PathBuf;

use mini::infer;
use test_case::test_case;
use tokio::fs;

#[test_case("access-record")]
#[test_case("adt0")]
#[test_case("adt-and-toplevel-binding")]
#[test_case("adt-binding2")]
#[test_case("adt-binding")]
#[test_case("adt-row2")]
#[test_case("adt-row")]
#[test_case("annot0")]
#[test_case("annot1")]
#[test_case("annot2")]
#[test_case("annotated-letrec-match")]
#[test_case("appdcon0")]
#[test_case("appdcon1")]
#[test_case("appdcon2")]
#[test_case("core-let")]
#[test_case("dec1")]
#[test_case("dec2")]
#[test_case("dec3")]
#[test_case("dec4")]
#[test_case("dec5")]
#[test_case("empty")]
#[test_case("empty-record")]
#[test_case("exists")]
#[test_case("extend-record")]
#[test_case("extension")]
#[test_case("forall")]
#[test_case("hhkind")]
#[test_case("hhmap")]
#[test_case("let-pattern")]
#[test_case("list2")]
#[test_case("list")]
#[test_case("list-map-annotated")]
#[test_case("list-not-currified")]
#[test_case("map")]
#[test_case("match0")]
#[test_case("match1")]
#[test_case("match2")]
#[test_case("match3")]
#[test_case("match-poly2")]
#[test_case("match-poly")]
#[test_case("or-pattern")]
#[test_case("pair-argument-datatype")]
#[test_case("pair")]
#[test_case("pair-match")]
#[test_case("record-annotation")]
#[test_case("record")]
#[test_case("record-with-some-simple-labels-2")]
#[test_case("record-with-some-simple-labels")]
#[test_case("rectype1")]
#[test_case("rectype")]
#[test_case("seq")]
#[test_case("triple-argument-datatype")]
#[test_case("tycon-arity")]
#[tokio::test]
async fn test_infer_good(input_file: &str) {
    let dir = PathBuf::from("tests/testdata");
    let input_path = dir.join(input_file).with_extension("good-input");
    let expected_path = dir.join(input_file).with_extension("good-output");
    let input_content = fs::read_to_string(&input_path).await.unwrap();
    let expected_content = fs::read_to_string(&expected_path).await.unwrap();

    let result = infer(&input_content).await.unwrap();

    assert_eq!(expected_content, result);
}

#[test_case("abs-label")]
#[test_case("adt0")]
#[test_case("adt1")]
#[test_case("adt2")]
#[test_case("constant-variable")]
#[test_case("dec1")]
#[test_case("dec2")]
#[test_case("dec3")]
#[test_case("dec4")]
#[test_case("empty")]
#[test_case("invalid-access")]
#[test_case("invalid-arity")]
#[test_case("invalid-record-type")]
#[test_case("invalid-row-kind")]
#[test_case("match0")]
#[test_case("match1")]
#[test_case("nonlinear-pattern")]
#[test_case("or-pattern")]
#[test_case("partial-app")]
#[test_case("two-occurences-of-label")]
#[test_case("typevar-typecon")]
#[test_case("unbound-typecon")]
#[tokio::test]
async fn test_infer_bad(input_file: &str) {
    let dir = PathBuf::from("tests/testdata");
    let input_path = dir.join(input_file).with_extension("bad-input");
    let input_content = fs::read_to_string(&input_path).await.unwrap();

    let result = infer(&input_content).await;

    assert!(result.is_err());
}
