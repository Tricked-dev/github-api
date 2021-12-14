pub mod end_points;

#[feature(test)]
mod test {
    use crate::end_points::*;
    #[test]
    pub fn test() {
        let point = EndPoints::GetUsersusernameEventsOrgsorg("test".to_owned(), "aa".to_owned());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());

    }
}
