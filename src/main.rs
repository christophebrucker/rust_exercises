mod functions;

fn main() {
    //functions::reverse("Hello, world!");
    //functions::max(34, 32);
    //functions::sort([4,3,2,1]);
    //functions::remove("test", "t");
    //functions::sum([10,10,10,10,10,10,10,10,10,10]);
    //functions::chunk(vec![1,2,3,4],3);
    //functions::occurencies(["a","a","b"]);
    functions::greeting("Hello Chris !!");
}

/*

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
