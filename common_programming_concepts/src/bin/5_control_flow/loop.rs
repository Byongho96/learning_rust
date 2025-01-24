fn main() {
    let mut cnt = 0;

    let result = loop {
        cnt += 1;

        if cnt == 10 {
            // the value after `break` will be returned out of the loop
            break cnt * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels to distinguish which loop we want to `break` or `continue`
    let mut outer_cnt = 0;
    let mut inner_cnt = 0;

    'outer_loop: loop {
        inner_cnt = 0;

        'inner_loop: loop {
            if inner_cnt == 10 {
                break 'outer_loop;
            }

            inner_cnt += 1;
        }

        if outer_cnt == 10 {
            break;
        }

        outer_cnt += 1;
    }
    println!("outer loop count: {outer_cnt}");
    println!("inner loop count: {inner_cnt}");
}
