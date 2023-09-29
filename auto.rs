use rust_decimal::Decimal; // Finance Safe Floating Point Decimal Type
use rust_decimal_macros::*; // Macros for Decimal Type

// fn main() {
//     let initial = dec!(500.00);
    
//     let balance = Balance {
// 	initial_bal: initial,
// 	base_bal: dec!(0.00000000),
// 	quote_bal: initial,
// 	current_bal: initial,
//     };

//     let balance = balance.buy(dec!(5000.00));

//     println!("Base Balance: {} Quote Balance {} Initial Balance {} Current Balance {} ",
// 	     balance.base_bal, balance.quote_bal, balance.initial_bal, balance.current_bal);
//     let balance = balance.sell(dec!(6000.00));
//     let profit = calc_profit(balance.initial_bal, balance.current_bal);

//     println!("Base Balance: {} Quote Balance {} Initial Balance {} Current Balance {} ",
// 	     balance.base_bal, balance.quote_bal, balance.initial_bal, balance.current_bal);
//     println!("Profit: ${} Percentage Gain: {}%", profit.0, profit.1);
// }
  
