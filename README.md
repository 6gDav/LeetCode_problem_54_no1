# <a href="https://leetcode.com/problems/spiral-matrix/description/">54. Spiral Matrix</a>

## 📝 Description

Given an m x n matrix, return all elements of the matrix in spiral order.

## 🧠 How I solved the problem 

I created a while loop that runs until the matrix becomes empty or the first row runs out.
In the while loop I implemented four steps: <br>
    1. remove the current first row and add it to the solution vector <br>
    2. iterate through all rows and remove all last elements while pushing these to the vector <br>
    3. remove the entire last row reverse it and add it too to the solution <br>
    4. add every first element while remove these too <br>
As you see my solution based on remove and cloning.

## ➗ Complexity

* **Time complexity**: *O((M×N)×(M+N))* - Because the matrix is *M* wide and *N* tall and the code visit elements only once but I used extra processes like *remove*, *pop* and *extend* which slow down the actual process.
* **Space complexity**: *O(1)* - I used only a vector as a extra data storage.

## 📊 Benchmark

I made it in release mode for more accurate results:
```bash
cargo run --release
```

Hardware: *Apple Mac Mini M4*

### 🤏 Small Input Test

* **Execution Time**: *5.875µs*
* **Memory Delta**: *16384 bytes*
* **Current Memory**: *1507328 bytes*

### 😖 Stress Test (Large Input)

* **Execution Time**: *19.666µs*
* **Memory Delta**: *81920 bytes*
* **Current Memory**: *1638400 bytes*