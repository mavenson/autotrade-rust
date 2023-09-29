use rust_decimal::Decimal; 
use rust_decimal_macros::*; 

struct Balance {
    initial_bal: Decimal,
    base_bal: Decimal,
    quote_bal: Decimal,
    current_bal: Decimal,
}

impl Balance {
    fn buy(&self, price: Decimal) -> Balance {
	Balance {
	    initial_bal: self.initial_bal,
	    base_bal: self.quote_bal / price,
	    quote_bal: dec!(0.00),
	    current_bal: self.quote_bal,
	}
    }
    
    fn sell(&self, price: Decimal) -> Balance {
	let bal = self.base_bal * price; 
	Balance {
	    initial_bal: self.initial_bal,
	    base_bal: dec!(0.00000000),
	    quote_bal: bal,
	    current_bal: bal,
	}
    }
}
