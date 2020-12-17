/*
## project_motion

### Instructions

For this exercise you will have to create a [projectile motion](https://cimg2.ck12.org/datastreams/f-d%3Abb024be6673110b31e78b46819e792adaed8dc661e082a61f0a6d64e%2BIMAGE%2BIMAGE.1).

You will be provided with a structure called `Object` that will have all variables that are
essential for the projectile physics. (distance, velocity, height, time)

You must implement :

- A function `throw_object` that will initialize the Object with a given velocity and height.
- The trait Iterator with the `.next()` in which it must calculate the next position of the object after 1 second.
  It will return an `Option` with the Object, It will return `None` if the object already reached the floor.

### Example

```rust
fn main() {
    let mut obj = Object::throw_object(50.0, 150.0);
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    // output:
    // Some(Object { distance: 50.0, velocity: 50.0, height: 145.1, time: 1.0 })
    // Some(Object { distance: 100.0, velocity: 50.0, height: 125.5, time: 2.0 })
    // Some(Object { distance: 150.0, velocity: 50.0, height: 81.4, time: 3.0 })
    // Some(Object { distance: 200.0, velocity: 50.0, height: 3.0, time: 4.0 })
    // None
    // None
}
```

### Notions

- https://doc.rust-lang.org/std/iter/trait.Iterator.html
- https://doc.rust-lang.org/rust-by-example/trait/iter.html

*/

// use std::iter::Sum;

#[derive(Debug, Clone, PartialEq)]
struct Object {
    distance: f32,
    velocity: f32,
    height: f32,
    time: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_distance() {
        let mut object = Object::throw_object(50.0, 20.0);

        assert_eq!(object.next(), Some(Object { distance: 50.0, velocity: 50.0, height: 15.1, time: 1.0 }));
        assert_eq!(object, Object { distance: 50.0, velocity: 50.0, height: 15.1, time: 1.0 });

        assert!(object.next().is_none(), "{:?} instead of None", object);
        assert!(object.next().is_none(), "{:?} instead of None", object);
    }

    #[test]
    fn test_media_distance() {
        let mut object = Object::throw_object(100.0, 30.0);

        assert_eq!(object.next(), Some(Object { distance: 100.0, velocity: 100.0, height: 25.1, time: 1.0 }));
        assert_eq!(object, Object { distance: 100.0, velocity: 100.0, height: 25.1, time: 1.0 });

        assert_eq!(object.next(), Some(Object { distance: 200.0, velocity: 100.0, height: 5.5, time: 2.0 }));
        assert_eq!(object, Object { distance: 200.0, velocity: 100.0, height: 5.5, time: 2.0 });

        assert!(object.next().is_none(), "{:?} instead of None", object);
    }

    #[test]
    fn test_long_distance() {
        let mut object = Object::throw_object(120.0, 100.0);

        assert_eq!(object.next(), Some(Object { distance: 120.0, velocity: 120.0, height: 95.1, time: 1.0 }));
        assert_eq!(object, Object { distance: 120.0, velocity: 120.0, height: 95.1, time: 1.0 });

        assert_eq!(object.next(), Some(Object { distance: 240.0, velocity: 120.0, height: 75.5, time: 2.0 }));
        assert_eq!(object, Object { distance: 240.0, velocity: 120.0, height: 75.5, time: 2.0 });

        assert_eq!(object.next(), Some(Object { distance: 360.0, velocity: 120.0, height: 31.4, time: 3.0 }));
        assert_eq!(object, Object { distance: 360.0, velocity: 120.0, height: 31.4, time: 3.0 });
        
        assert!(object.next().is_none(), "{:?} instead of None", object);
    }
}
