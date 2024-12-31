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

### 1. Data Preprocessing  
To improve performance, data is preprocessed by converting it from CSV into an efficient binary format (`bincode`).  
- The data is reorganized for fast lookups using indexes.  
- TypeScript types are generated for seamless integration with the frontend.  

This reduces runtime overhead and ensures efficient data access.  

### 2. Optimizing Offers with Bitmaps  
Previously, offers were stored as a large list (~3200 entries), which was slow and consumed 224 kB of memory. Now, each game uses two bitmaps ("Live" and "Highlights") to represent offers.  
- **Memory Reduced:** Down to ~45 kB.  
- **Faster Queries:** Offers can now be accessed and compared with simple bitmap operations.  
- **Algorithm Simplification:** The optimization streamlined the selection process.  

**Results:** Benchmarks show up to **327x faster performance** in finding the best combination.  

### 3. The Algorithm  
The algorithm is a **Depth-First Search (DFS)** with branch pruning:  
- It traverses possible package combinations, maintaining the best one found so far.  
- Branches that cannot provide a better solution are skipped early.  
- The bitmap structure enables quick calculations and comparisons of coverage.  

#### Micro-Optimizations  
Several optimizations enhance algorithm efficiency:  
1. Preallocated memory to avoid runtime allocation overhead.  
2. In-place computations to reduce temporary object creation.  
3. Compact bitmaps for fast bitwise operations.  
4. Optimized branch pruning to minimize unnecessary checks.  

These improvements make the algorithm highly efficient and capable of handling complex combinations in real time. The source code is thoroughly documented, and I am especially proud of how clear and optimized it turned out. I strongly encourage exploring it for further insights.