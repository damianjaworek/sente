# Sente Programming Language

Sente is a simple programming language compiled to WebAssembly. 

## Building from source

1. Make sure you have installed: 
  - ```rustc``` and ```cargo```
  - ```git```
2. Clone the [source](https://github.com/damianjaworek/sente) with ```git```:
  ```
    git clone https://github.com/damianjaworek/sente.git
    cd sente
  ```
3. Build:
  ```
    cargo build --release
  ```
  When complete it will place ```sentec```, Sente compiler, in ```target/release```.

## Using

To compile ```inputFile``` and place the result in ```outputFile``` use: 
```
  ./sentec -i inputFile -o outputFile
``` 

## Examples

```sente
  fn fibRec(n: Int32) -> Int32 {
    if (n == 0) {
      return 1;
    }
    if (n == 1) {
      return 1;
    }
    return fibRec(n-1) + fibRec(n-2);
  }
```
```sente
  fn fibIter(n: Int32) -> Int32 {
    var a: Int32 = 0;
    var b: Int32 = 1;
    while (n > 0) {
      var t: Int32 = a;
      a = b;
      b = t + a;
      n = n - 1;
    }
    return b;
  }
```
```sente
  fn sqrt(s: Float64) -> Float64 {
    var x: Float64 = 1.0;
    var i: Int64 = 0;
    
    while (i < 1000000) {
      x = (x + s / x) / 2.0;
      i = i + 1;
    }
    
    return x;
  }
```
