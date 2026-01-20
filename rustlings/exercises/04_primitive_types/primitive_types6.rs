fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);
        let numbers2 = [1, 2, 3];

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        let second = numbers.1;
        let second2 = numbers2[1];

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
        assert_eq!(second2, 2, "This is not the 2nd number in the array!");
    }
}
