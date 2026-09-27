use conduit::config_engine::transaction::ConfigTransaction;

#[test]
fn transaction_can_be_created() {
    let transaction = ConfigTransaction::new();

    assert!(!transaction.is_committed());
    assert!(!transaction.is_rolled_back());
}

#[test]
fn transaction_can_commit() {
    let mut transaction = ConfigTransaction::new();

    transaction.commit().expect("commit should succeed");

    assert!(transaction.is_committed());
    assert!(!transaction.is_rolled_back());
}

#[test]
fn transaction_can_rollback() {
    let mut transaction = ConfigTransaction::new();

    transaction.rollback().expect("rollback should succeed");

    assert!(transaction.is_rolled_back());
    assert!(!transaction.is_committed());
}
