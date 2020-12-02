/*
## box_recursion

### Instructions

Using the given code create the following functions:

- `new` that will initialize the `WorkEnvironment` as `None`
- `add_worker`, that receives two strings, one being the type of worker and the other the name of the worker.
- `remove_worker`, that removes the last worker that was placed in the `WorkEnvironment`, this functions should 
  return a Option with the name of the worker.
- `search_worker`, that return a tuple with the name and type of worker.

You must also create a type called `Link` this will be the connection of the structures `WorkEnvironment` and `Worker`.
Do not forget that this will be a recursion type and it must point to `None` if there is no workers.

### Notions

- https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
- https://doc.rust-lang.org/book/ch15-01-box.html
- https://doc.rust-lang.org/std/option/
- https://doc.rust-lang.org/book/ch15-01-box.html
- https://doc.rust-lang.org/std/keyword.impl.html
*/

#[derive(Debug)]
struct WorkEnvironment {
    grade: Link,
}

#[derive(Debug)]
struct Worker {
    worker_type: String,
    worker_name: String,
    next_worker: Link,
}

impl WorkEnvironment {
    fn new() -> WorkEnvironment {}
    fn add_worker(&mut self, t: String, name: String) {}
    fn remove_worker(&mut self) -> Option<String> {}
    fn search_worker(&self) -> Option<(String, String)> {}
}

/*
// Example :
fn main() {
  let mut list = WorkEnvironment::new();
  list.add_worker(String::from("CEO"), String::from("Marie"));
  list.add_worker(String::from("Manager"), String::from("Monica"));
  list.add_worker(String::from("Normal Worker"), String::from("Ana"));
  list.add_worker(String::from("Normal Worker"), String::from("Alice"));
  println!("{:?}", list);
  // output:
  // WorkEnvironment { grade: Some(Worker { worker_type: "Normal Worker", worker_name: "Alice", next_worker: Some(Worker { worker_type: "Normal Worker", worker_name: "Ana", next_worker: Some(Worker { worker_type: "Manager", worker_name: "Monica", next_worker: Some(Worker { worker_type: "CEO", worker_name: "Marie", next_worker: None }) }) }) }) }

  println!("{:?}", list.search_worker());
  // output:
  // Some(("Alice", "Normal Worker"))
  list.remove_worker();
  list.remove_worker();
  list.remove_worker();
  list.remove_worker();
  println!("{:?}", list);
  // output:
  // WorkEnvironment { grade: None }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut list = WorkEnvironment::new();
        assert!(list.grade.is_none());
    }

    #[test]
    fn test_one_worker() {
        let mut list = WorkEnvironment::new();
        list.add_worker(String::from("CEO"), String::from("Marie"));
        list.remove_worker();
        assert!(list.grade.is_none());
    }

    #[test]
    fn test_two_workers() {
        let mut list = WorkEnvironment::new();
        list.add_worker(String::from("CEO"), String::from("Marie"));
        list.add_worker(String::from("Manager"), String::from("Monica"));
        list.remove_worker();

        assert_eq!(list.grade.as_ref().unwrap().worker_type, "CEO");
        assert_eq!(list.grade.as_ref().unwrap().worker_name, "Marie");
    }

    #[test]
    fn test_more_workers() {
        let mut list = WorkEnvironment::new();
        list.add_worker(String::from("CEO"), String::from("Marie"));
        list.add_worker(String::from("Manager"), String::from("Monica"));
        list.add_worker(String::from("Normal Worker"), String::from("Ana"));
        list.add_worker(String::from("Normal Worker"), String::from("Alice"));
        list.remove_worker();

        assert_eq!(list.grade.as_ref().unwrap().worker_type, "Normal Worker");
        assert_eq!(list.grade.as_ref().unwrap().worker_name, "Ana");

        list.remove_worker();
        list.remove_worker();
        assert_eq!(list.grade.as_ref().unwrap().worker_type, "CEO");
        assert_eq!(list.grade.as_ref().unwrap().worker_name, "Marie");
    }

    #[test]
    fn test_search() {
        let mut list = WorkEnvironment::new();
        list.add_worker(String::from("CEO"), String::from("Marie"));
        list.add_worker(String::from("Manager"), String::from("Monica"));
        list.add_worker(String::from("Normal Worker"), String::from("Ana"));
        list.add_worker(String::from("Normal Worker"), String::from("Alice"));

        assert_eq!(
            list.search_worker().unwrap(),
            (String::from("Alice"), String::from("Normal Worker"))
        );

        list.remove_worker();
        assert_eq!(
            list.search_worker().unwrap(),
            (String::from("Ana"), String::from("Normal Worker"))
        );

        list.remove_worker();
        assert_eq!(
            list.search_worker().unwrap(),
            (String::from("Monica"), String::from("Manager"))
        );

        list.remove_worker();
        assert_eq!(
            list.search_worker().unwrap(),
            (String::from("Marie"), String::from("CEO"))
        );
    }
}
