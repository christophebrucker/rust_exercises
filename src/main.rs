mod functions;

fn main() {
    //functions::reverse("Hello, world!");
    //functions::max(34, 32);
    functions::sort([4,3,2,1]);
}

/*

// (D) remove a given character from a string
// TESTS:
// removeChar("Hello World", "H")            // => "ello World"
// removeChar("Hi, how are you doing?", "i") // => "H, how are you dong?"
function removeChar(str, char) {
  return str.split(char).join();
}

// (E) sum all numbers of an array
// TESTS:
// sum([1,2,3,4,5])      // => 15
// sum([829,-12,758,2])  // => 1577
function sum(arr) {
  return arr.reduce((a,b) => a + b);
}

// (F) chunk an array into an array of arrays
// => what data structur would you use for that in Rust
// => can you even create an Array of Arrays in Rust?
// TESTS:
// chunkArray([1,2,3,4,5,1,2,3,4,5,1,2,3], 5)
//        => [[1,2,3,4,5],[1,2,3,4,5],[1,2,3]]
// chunkArray([1,2,3,4,5,1,2,3,4,5,1,2,3], 3)
//        => [[1,2,3],[4,5,1],[2,3,4],[5,1,2],[3]]
function chunkArray(arr, size) {
  const chunkedArr = [];
  let index = 0;
  while (index < array.length) {
    chunkedArr.push(array.slice(index, size + index));
    index += size;
  }
  return chunkedArr;
}

// (G) count occurencies all distinct values of an array and return as dictionary
// => what datastructure to use in RUST for such a task?
// TESTS:
// createDict(["a","b","a","a","c","c","a"])
//         => {"a":4, "b":1, "c":2}
function createDict(arr) {
  const dict = {};
  arr.forEach(element => {
    dict[element] = dict[element] ? dict[element] + 1 : 1;
  });
  return dict;
}

// (H) flatten an array of an array
// how implement Array of Array in Rust?
// TESTS:
// flatArr([1,2,3],[4,5,6]) // => [1,2,3,4,5,6]
function flatArr(arr) {
	return arr.flat();
} */
