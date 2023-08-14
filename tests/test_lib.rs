use calc::add;
use calc::substract;
use calc::multiply;
use calc::divide;

#[test]
fn test_add() {
    assert_eq!(add(1,2),3);
}

#[test]
fn test_substract() {
    assert_eq!(substract(1,2),-1);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(1,2),2);
}

#[test]
fn test_divide() {
    assert_eq!(divide(1,2),0);
}