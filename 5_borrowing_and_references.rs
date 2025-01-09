// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts


// Understanding References
// only one owner, so need more copies to assign new owners/variables, that's slow!
// References: Enable you to borrow values without taking ownership.
// Immutable Reference.
// Mutable Reference.
// Create a Reference by adding "&" before the value which needs to be referred.


fn main(){
    let mut x: i32 = 5;

    let r: &mut i32 = &mut x; // make a reference, instead of changing ownership from x to r by adding & before x

    *r += 1;
    *r -= 3;

    println!("Value of x: {}", x);
    // println!("Value of r: {}", r); // doesnt run here! throws an error
    // why does the above error occur? 
    // you can have only 1 mutable reference to a value OR infinite immutable references
    // but when line 23 hits, after line 22
    // what happened is, on line 22, x is considered as immutable reference
    // and if then r is used, on line 23, it is a mutable reference, hence the clash, cannot have both immutable and mutable references
    // either 1 mutable ref. OR INF immutable ref.


    let mut account:BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdaw money
    account.withdraw(45.50);

    // Immutable borrow to check the balance
    account.check_balance();

    // as you can see in lines 37-44 there is a mutable borrow as well as 2 immutable borrows
    // but still it works fine! why?
    // because, if you can see functions in BankAccount, the mutable and immutable references
    // are not in the same SCOPE! since they are different scope, the rule is not violated!
    
}

struct BankAccount{
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount:f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }
    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}


// Safety refers to what? related to these
// null ptr dereferencing
// dangling ptrs
// data raises