mod ticket {
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

// TODO: **Exceptionally**, you'll be modifying both the `ticket` module and the `tests` module
//  in this exercise.

#[cfg(test)]
mod tests {
    use super::ticket::Ticket;

    // This test should now be commented out because it tries to access private fields
    // fn should_not_be_possible() {
    //     let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

    //     // Comment this line out to allow the test to pass
    //     // assert_eq!(ticket.description, "A description");
    // }

    fn encapsulation_cannot_be_violated() {
        // This should be impossible as well, and needs to be commented out
        // let ticket = Ticket {
        //     title: "A title".into(),
        //     description: "A description".into(),
        //     status: "To-Do".into(),
        // };
    }
}
