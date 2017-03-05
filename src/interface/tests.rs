use super::{Parameter, Command};

/*
 * Command
 */
#[test]
fn command_from_str_valid() {
    let c = Command::from_str("listvm").unwrap();

    assert_eq!(c.name.as_str(), "listvm");
    assert_eq!(c.parameters.len(), 0);

    let c = Command::from_str("\t listimg   ").unwrap();

    assert_eq!(c.name.as_str(), "listimg");
    assert_eq!(c.parameters.len(), 0);

    let c = Command::from_str("createvm, name test, cpus 1, ram 512").unwrap();

    assert_eq!(c.name.as_str(), "createvm");
    assert_eq!(c.parameters.len(), 3);

    assert_eq!(c.parameters[0].key.as_str(), "name");
    assert_eq!(c.parameters[0].value.as_str(), "test");

    assert_eq!(c.parameters[1].key.as_str(), "cpus");
    assert_eq!(c.parameters[1].value.as_str(), "1");

    assert_eq!(c.parameters[2].key.as_str(), "ram");
    assert_eq!(c.parameters[2].value.as_str(), "512");
}

#[test]
#[should_panic]
fn command_from_str_invalid_empty() {
    Command::from_str("").unwrap();
}

#[test]
#[should_panic]
fn command_from_str_invalid_param() {
    Command::from_str("createvm, name").unwrap();
}
