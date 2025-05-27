use car::Car;
use interfaces::*;

#[test]
fn test_try_turn_steering_wheel() {
    let car = Car::new();
    let result = car.try_turn_steering_wheel(&Steering::Left);
    assert!(result.is_err());

    let result = car.try_turn_steering_wheel(&Steering::Right);
    assert!(result.is_ok());

    let result = car.try_turn_steering_wheel(&Steering::Straight);
    assert!(result.is_ok());
}

#[test]
fn test_try_pedaling() {
    let mut car = Car::new();
    let result = car.try_pedaling(&Velocity::Acceleration);
    assert_eq!(result.unwrap(), 10);

    let mut car = Car::new();
    let _ = car.try_pedaling(&Velocity::Acceleration);
    let _ = car.try_pedaling(&Velocity::Acceleration);
    let result = car.try_pedaling(&Velocity::Acceleration);
    assert_eq!(result.unwrap(), 30);

    let mut car = Car::new();
    let _ = car.try_pedaling(&Velocity::Acceleration);
    let _ = car.try_pedaling(&Velocity::Acceleration);
    let result = car.try_pedaling(&Velocity::Deceleration);
    assert_eq!(result.unwrap(), 15);

    let mut car = Car::new();
    let result = (1..20).fold(0, |_, _| car.try_pedaling(&Velocity::Deceleration).unwrap());
    assert_eq!(result, 0);

    let mut car = Car::new();
    let result = (1..20).fold(0, |_, _| car.try_pedaling(&Velocity::Acceleration).unwrap());
    assert_eq!(result, 90);
}
