function sort(array) {
  array.sort(function (a, b) {
    return a - b;
  })
  return array;
}

function quicksort(array, less) {

  function swap(i, j) {
    var t = array[i];
    array[i] = array[j];
    array[j] = t;
  }

  function _quicksort(left, right) {

    if (left < right) {
      var pivot = array[left + Math.floor((right - left) / 2)],
        left_new = left,
        right_new = right;

      do {
        while (less(array[left_new], pivot)) {
          left_new += 1;
        }
        while (less(pivot, array[right_new])) {
          right_new -= 1;
        }
        if (left_new <= right_new) {
          swap(left_new, right_new);
          left_new += 1;
          right_new -= 1;
        }
      } while (left_new <= right_new);

      _quicksort(left, right_new);
      _quicksort(left_new, right);

    }
  }

  _quicksort(0, array.length - 1);

  return array;
}

function addedStatement() {
  console.log("");
}

function removedStatement() {
  console.log("ToRemove");
  console.log("");
}

function updatedStatement() {
  console.log("ToUpdate");
}

function addedAndRemovedStatement() {
  console.log("ToRemove");
  console.log("");
  console.log("");
  console.log("");
}

function notExecutedByTestAdded() {
  console.log("");
}

function notExecutedByTestRemoved() {
  console.log("ToRemove");
  console.log("");
}

function notExecutedByTestUpdated() {
  console.log("ToUpdate");
}

function notExecutedByTestAddedAndRemoved() {
  console.log("ToRemove");
  console.log("");
  console.log("");
  console.log("");
}


exports.quicksort = quicksort;
exports.sort = sort;
exports.addedStatement = addedStatement;
exports.removedStatement = removedStatement;
exports.updatedStatement = updatedStatement;
exports.addedAndRemovedStatement = addedAndRemovedStatement;
exports.notExecutedByTestAdded = notExecutedByTestAdded;
exports.notExecutedByTestRemoved = notExecutedByTestRemoved;
exports.notExecutedByTestUpdated = notExecutedByTestUpdated;
exports.notExecutedByTestAddedAndRemoved = notExecutedByTestAddedAndRemoved;