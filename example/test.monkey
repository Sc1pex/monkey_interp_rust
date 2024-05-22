let map = fn(arr, f) {
    let iter = fn(arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            iter(rest(arr), push(accumulated, f(first(arr))));
        }
    };
    iter(arr, []);
};

let reduce = fn(arr, initial, f) {
    let iter = fn(arr, result) {
        if (len(arr) == 0) {
            result
        } else {
            iter(rest(arr), f(result, first(arr)));
        }
    };
    iter(arr, initial);
};

let sum = fn(arr) {
    reduce(arr, 0, fn (initial, el) { initial + el; });
}

let a = [1, 2, 3, 4, 6];
map(a, fn (x) {x * x});

sum(a);
