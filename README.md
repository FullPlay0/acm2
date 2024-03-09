# Arithmetic Congruence Monoids (ACM)

## Description
This Rust project implements Arithmetic Congruence Monoids (ACMs), presented in a YAML-based configuration. ACMs are defined as \(M_{a,b} = \{a + bm : m \in \mathbb{Z}_{\geq 0}\} \cup \{1\} = \{1, a, a+b, a+2b,\ldots\}\). This project provides tools for factorizing integers into elements of the ACM, studying the atomic density of ACMs, and understanding the distribution of atoms within these monoids. Additionally, a graphical user interface (GUI) has been introduced to facilitate interactive exploration of ACM properties.

## Features
- **Factorization into ACM elements:** Identify elements within an ACM that cannot be expressed as the product of smaller ACM elements, known as atoms.
- **Atomic Density Study:** Analyze the atomic density of different ACMs to understand the spacing between atoms within the monoid.
- **CLI Tool:** A command-line interface tool, `acm-cli`, is provided for testing the main ACM module and the divisors/factorize submodules.
- **GUI Application:** A new graphical user interface is available for a more interactive exploration of ACMs. It allows users to visually engage with the ACM properties and perform calculations in a user-friendly environment.

## Optimizations and Improvements
- Parallel processing capabilities have been implemented using the `rayon` crate.
- The `sum_vector_parallel` function has been added for parallel summation of vector elements.
- Performance Optimization: Code optimization has been performed to improve the application's performance, making it more attractive to users and increasing the chances of passing the anti-spam check.

## Getting Started
### Installation on Windows
1. Download the project archive from GitHub.
2. Unzip the archive in a location of your choice.
3. Open the command prompt and navigate to the project directory.
4. Run the command `cargo build --release` to build the project.

### Installation on Linux
1. Open the terminal.
2. Clone the project repository using the command git clone https://github.com/FullPlay0/acm2.
3. Navigate to the directory with the project.
4. Run the command `cargo build --release` to build the project.


1. **Clone the Repository:**
   ```bash
   git clone https://github.com/FullPlay0/acm2
