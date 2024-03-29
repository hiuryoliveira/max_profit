#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        let shares_prices_1 = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(shares_prices_1), 7);

        let shares_prices_2 = vec![1, 2, 3, 4, 5];
        assert_eq!(max_profit(shares_prices_2), 4);

        let shares_prices_3 = vec![5, 4, 3, 2, 1];
        assert_eq!(max_profit(shares_prices_3), 0);
    }
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            max_profit += prices[i] - prices[i - 1];
        }
    }
    max_profit
}

fn main() {
    let shares_prices_1 = vec![7, 1, 5, 3, 6, 4];
    println!("Max profit: {:?}", max_profit(shares_prices_1),);
}
