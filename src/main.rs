/* In enums the possible set of values that
we define are namespaced under the identifier of enum
It means we need to use "::" for accessing a particular val*/

#[derive(Debug)]
enum IpAddrKind {
    V4,
    DummyV5,
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    /* We can also have implementation of associated functions
       & methods like struct
    */
    fn _some_func() {
        /* This will do some operation */
    }
}

fn main() {
    let ip_version_four: IpAddrKind = IpAddrKind::V4;
    let ip_version_six: IpAddrKind = IpAddrKind::DummyV5;

    /* As route takes IpAddrKind as input we can pass any of the values which are of type  IpAddrKind*/
    route(ip_version_four);
    route(ip_version_six);
    /* And again the ownership works same for enums, we cant borrow moved values */
    // println!("{:#?}", ipVersionFour);

    /* If we wanted to have a value for each kind of ip address
       we can define a struct and use it like below. So that each instance
       of struct can have its kind and address of ip.
    */
    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:#?}", localhost);

    /* But we can make it more concise by using enum */
    let somehost = IpAddrKind::V6(String::from("::1"));
    println!("{:#?}", somehost);

    /* Enum variants can store any type of data */
    /* We had created a enum for listing all the types of messages and
    each accept a different types of inputs  this type of grouping cant be done in struct*/
    let message: Message = Message::Quit;
    print_message(message);

    /* Optional Enums */
    let _some_number = Some(5);
    let _some_string = Some("Some string");
    /* For this no value case we should definitely annotate the
    type as there is no value rust type system cannot infer it  */
    let _no_val: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    /* Here we are trying to add integer value
    with Optional Integer type which cannot be possible
    To add these we need to have something like if there is value in y,
    we need to get y value else may be we can add with 0
    To do so we can use default methods in the rust like unwrap
    */
    let sum = x + y.unwrap_or(0);
    println!("Unwrap-Option Enum with value add: {sum}");

    let y: Option<i8> = None;
    let sum = x + y.unwrap_or(0);
    println!("Unwrap-Option Enum with None add: {sum}");

    /* Using Match expression */
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        /* We can also store structs in enum variants */
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            /* In the below way, we can use values of enum variants with match expression */
            Coin::Quarter(state) => {
                println!("Quarter of {:#?}", state);
                25
            }
        }
    }
    println!(
        "Dime coin value by using match: {}",
        value_in_cents(Coin::Dime)
    );
    println!(
        "Quarter coin value by using match: {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );

    /* Match expression with Option enum */

    fn plus_one(x: Option<i32>) -> Option<i32> {
        /* Function that checks for x using match, adds 1 and return that value */
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    println!(
        "Option Enum with Match-> add 1 for None: {:?}",
        plus_one(None)
    );
    println!(
        "Option Enum with Match-> add 1 for 2: {:?}",
        plus_one(Some(2))
    );

    /* Here in a match expression, even we care about one value match
    we need to write the default match case as below */
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }
    
    /* Instead of this we can use the if let syntax which is less verbose */
    if let Some(3) = some_value {
        println!("Three");
    }
}

fn route(ip_kind: IpAddrKind) {
    /* This func takes ip_kind as an input */
    println!("{:#?}", ip_kind);
}

fn print_message(message: Message) {
    println!("{:#?}", message);
}
