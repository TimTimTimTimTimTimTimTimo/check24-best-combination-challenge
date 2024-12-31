# Check24 Best Combination Challenge

This repository contains my submission for the Check24 GenDev challenge.

## Getting Started

### Prerequisites

Make sure you have the following installed:

- [pnpm](https://pnpm.io/)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Installation

1. Install the dependencies:
    ```bash
    pnpm install
    ```

2. Run the development server:
    ```bash
    pnpm run tauri dev
    ```
    Note: Running the development server will have slower performance as the Rust backend will be compiled in debug mode. For a more accurate performance evaluation, build the project or use the release binaries.

3. Build the project:
    ```bash
    pnpm tauri build
    ```

## Optimizations

### 1. Data Preprocessing and Precomputation  
To eliminate parsing and conversion overhead, data is parsed from CSV files, processed, and serialized into an efficient binary format using `bincode`. During this step:  
- The data is reshaped for quick querying by indexing directly into vectors based on their IDs.  
- Relevant TypeScript types are generated for seamless integration with the frontend.

This approach minimizes runtime processing overhead and ensures fast, efficient data access in the backend.

### 2. Eliminating Offers aka. I ❤️ Bitmaps!  
Previously, the relationship between games and packages relied on traversing a large set of offers (approximately 3200 entries). This approach had:  
- A significant memory footprint (~224 kB).  
- Poor query performance for common operations.

**Optimization:** Each game's offers are now represented using two bitmap fields (one for "Live" and one for "Highlights"). Each bit in the bitmap corresponds to a package ID:  
- **Memory Efficiency:** Reduced to ~45 kB by using compact 64-bit integers.  
- **Query Speed:** Accessing all offers for a game is now as simple as reading the bitmap.  
- **Algorithm Simplification:** The best combination selection algorithm is now straightforward and efficient.

**Results:**  
Preliminary benchmarks using sample datasets show up to **327x faster performance** for finding the best combination. Without this optimization, achieving acceptable response times would not have been feasible.

### 3. The Algorithm  
The core algorithm used to determine the best combination of packages is a **Depth-First Search (DFS) with Branch Pruning**. It is conceptually simple but highly optimized in implementation. Here's an overview:  

- **Traversal:**  
  The algorithm systematically explores all possible package combinations using DFS. During traversal, it keeps track of the **best combination** found so far that covers all required games.  

- **Branch Pruning:**  
  If a combination cannot provide a better solution than the current best, the algorithm immediately skips further exploration of that branch.  

- **Bitmap Efficiency:**  
  The game bitmaps are key to this process, enabling fast computation and comparison of game coverages for packages or combinations. This eliminates the need for slower, iterative checks.  

#### Micro-Optimizations in the Algorithm  
To achieve maximum performance, several low-level optimizations have been implemented:  
1. **Preallocation:**  
   - All necessary memory is preallocated before the algorithm starts, eliminating runtime allocation overhead.  
2. **In-Place Computations:**  
   - Wherever possible, operations are performed directly in memory without creating temporary objects or structures.  
3. **Compact Representations:**  
   - Data structures, such as the bitmaps, are designed to minimize memory usage while enabling efficient bitwise operations.  
4. **Branch Skipping Logic:**  
   - Conditions for branch pruning are carefully optimized to minimize unnecessary comparisons and checks.  

#### Why It Matters  
These optimizations significantly enhance the algorithm's speed and scalability. Preliminary benchmarks demonstrate exceptional performance improvements, making it feasible to handle complex combinations in real-time.  

#### Source Code  
The source code for the algorithm is meticulously documented, with detailed comments explaining each step and optimization. I encourage you to review it if you're interested in the implementation or wish to adapt the algorithm for similar problems. This part of the project represents a substantial effort, and I am especially proud of its clarity and efficiency.