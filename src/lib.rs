pub mod checks {
    pub fn requires_subtraction(number: &u16) -> bool {
        let operator: [u16; 5] = [1, 5, 10, 50, 100];
        let round: [u16; 6] = [5, 10, 50, 100, 500, 1000];

        for i in operator.iter() {
            for j in round.iter() {
                if number + *i == *j {
                    return false;
                };
            };
        };

        return true;
    }
}
