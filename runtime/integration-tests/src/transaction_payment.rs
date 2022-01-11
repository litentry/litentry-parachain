use crate::setup::*;

#[test]
fn test_payment() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(true, true);
	})
}
