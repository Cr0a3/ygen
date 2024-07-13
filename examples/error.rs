use Ygen::Support::Error;

fn main() {
    let mut error = Error::new(
        "undetermined statement".into(), "test.c".into(), "4".into(), "5".into()
    );

    error.setCodeLine("    printf(\"Hello World\")".into());
    error.addWhere("expected a ;".into(), 26, 1);

    error.print();
}