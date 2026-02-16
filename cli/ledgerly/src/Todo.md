# From Top I have to :

1. Learn making a cli program that uses the name ledgerly to start.
2. Accepts some arguments.
3. Do real mercy handling on the type of data entered.

# Deep :

E.g -> 

// add to ledger
ledgerly add water 250ml
ledgerly add code rust 45m 

// view today ledger
ledgerly today

// queries by time

ledgerly last 7d
ledgerly since YY/MM/DD 

# Skills I will learn :

1. Struct design 
2. Enums 
3. Error Handling 
4. File input/output 
5. Serialization Json/Csv/Custom format 
6. Lifetimes & borrowings.
7. Iterators.
8. Modules.
9. CLI parsing.

# A must :

1. The cmds must be inside an enum for e.g 

enum Command {
    Add(AddCmd),
    Today,
    Last(Duration),
    Sum(SumCmd),
}

2. Each program must be a module instead of being cultured into a single .rs file.


3. Stick to chrono or use time if wants toughness.

