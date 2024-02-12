# compute/core.rs

This file contains the core logic for the compute module. It includes the following functions:

* `init`: Initializes the matrix from a string representation.
* `init_from_file`: Initializes the matrix from a file.
* `solve`: Solves the matrix using the Jacobi method.
* `print`: Prints the matrix to stdout.
* `print_sol`: Prints the solution to stdout.

## Usage

To use the `compute/core.rs` file, you can either import it into your own Rust project or use it as a standalone library.

To import the file, you can use the following code:


To use the file as a standalone library, you can run the following command:

```
use compute::core::Matrix;

let mut matrix = Matrix::new(); matrix.init(" 3 1 2 3 4 5 6 7 8 9 0.5 ");

let solution = matrix.solve();

println!("Solution: {:?}", solution);
```


## Examples

The following example shows how to use the `compute/core.rs` file to solve a simple matrix equation:


This example will print the following output:

Solution: { "sol": [ 1.0, 2.0, 3.0 ], "acc": [ 0.0, 0.0, 0.0 ], "iter": 1 }


# Linear Equation Solver

This is a React application that solves linear equations. The application can be used to solve linear equations manually or by uploading a file containing the equations. The application will return the number of iterations it took to solve the equations, the vector of unknowns, and the vector of errors.

## Getting Started

To get started, clone the repository and install the dependencies.

```
git clone https://github.com/GoogleCloudPlatform/nodejs-docs-samples.git
cd nodejs-docs-samples/compmath/src/frontend/spa
bun install

```

Once the dependencies are installed, you can start the application.

```
bun run

```

The application will be available at http://localhost:3000.

## Using the Application

To use the application, follow these steps:

1.  Enter the linear equations in the text box.
2.  Click the "Solve" button.
3.  The application will return the number of iterations it took to solve the equations, the vector of unknowns, and the vector of errors.

## Contributing

If you would like to contribute to this project, please fork the repository and submit a pull request.

## License

This project is licensed under the Apache License, Version 2.0. See the LICENSE file for more information.


## Contributing

If you would like to contribute to the `compute/core.rs` file, please feel free to open a pull request on GitHub.

## License

The `compute/core.rs` file is licensed under the MIT license.
