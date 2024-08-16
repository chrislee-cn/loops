fn main() {
    // loop {
    //     println!("again!");
    // }
    {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {result}");
    }

    {
        let mut count = 0;
        //循环标签
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 9;
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count = {count}");
        let mut number = 3;
        while number != 0 {
            println!("{number}!");
            number -= 1;
        }
        println!("LIFTOFF!!!");
    }

    {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
        while index <= 5 {
            println!("the value is: {}", a[index]);
            index += 1;
        }
    }
    {
        let a = [10, 20, 30, 40, 50];
        for e in a {
            println!("the value is :{e}")
        }
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
    {
        fn fib(n: u64) -> u64 {
            match n {
                0 => 1,
                1 => 1,
                n => fib(n - 1) + fib(n - 2),
            }
        }
        let mut i = 0;
        while i < 10 {
            let result = fib(i);
            println!("{result}");
            i = i + 1;
        }
    }
    {
        let mut a = 0;
        let mut b = 1;
        let mut num;
        let mut i = 0;
        while i < 14 {
            num = a + b;
            a = b;
            b = num;
            i = i + 1;
            println!("{num}");
        }
    }

    let mut a = 0;
    let mut b = 1;
    let mut num;
    let mut m;
    for n in 1..15 {
        num = a + b;
        a = b;
        b = num;
        m = n + 1;
        println!("f({m})={num}");
    }

    let months = [
        "anuary",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for m in months {
        println!("{m}");
    }
}
