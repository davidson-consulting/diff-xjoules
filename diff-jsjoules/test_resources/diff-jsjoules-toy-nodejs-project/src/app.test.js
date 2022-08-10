const app = require('./app')

test('test added statement', () => {
    app.addedStatement();
});

test('test removed statement', () => {
    app.removedStatement();
});

test('test updated statement', () => {
    app.updatedStatement();
});

test('test added and removed statement', () => {
    app.addedAndRemovedStatement();
});

test('test empty', () => {
    var test_array = [];
    var sorted_array = app.quicksort(test_array, function (a, b) { return a < b; });
    expect(is_sorted(sorted_array)).toBeTruthy();
});

test('test singleton', () => {
    var test_array = [10];
    var sorted_array = app.quicksort(test_array, function (a, b) { return a < b; });
    expect(is_sorted(sorted_array)).toBeTruthy();
});

test('test random', () => {
    var test_array = get_random_list(100);
    var sorted_array = app.sort(test_array);
    expect(is_sorted(sorted_array)).toBeTruthy();
});

test('test quicksort', () => {
    var test_array = get_random_list(100);
    var sorted_array = app.quicksort(test_array, function (a, b) { return a < b; });
    expect(is_sorted(sorted_array)).toBeTruthy();
});

function get_random_list(nb_element) {
    return Array.from({ length: nb_element }, () => Math.floor(Math.random() * nb_element));
}

function is_sorted(array) {
    let previous = array[0];
    for (var i = 1; i < array.length; i++) {
        let current = array[i];
        if (current < previous) {
            return false;
        }
        previous = current;
    }
    return true;
}