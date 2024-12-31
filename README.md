# Check24 Best Combination Challenge

This repository contains my submission for the Check24 GenDev challenge.

## How to run locally

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

If there are difficulties while trying to get the application running, feel free to reach out to me!

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

## Possible Future Optimizations  

### 1. Precalculating/Caching  
Popular queries could be precalculated and cached, with an LRU eviction strategy. Advanced caching could reuse results from previous queries that cover subsets of the current games to skip parts of the algorithm or guide execution.  

### 2. Enhancing the Algorithm  
Parts of the algorithm could be further optimized:  
- **SIMD:** SIMD could speed up coverage calculations, though initial attempts showed no improvements.  
- **Simpler Data Structures:** Replacing `uncovered_games_map` with a vector of game IDs could simplify logic and enable SIMD optimizations.  

### 3. Switching the Algorithm  
The problem could be solved using linear programming (LP) and dedicated solvers. While I chose a custom solution for its flexibility and learning opportunities, comparisons suggest its performance is on par with LP-based approaches.  

### Conclusion  
The current implementation performs well, but caching, algorithm refinements, or LP solvers could unlock further improvements. These options provide a solid foundation for future work.  

## Application Structure and Tech Stack  

For this project, I decided to build a desktop application. The calculations are lightweight enough to run on-device, and it gave me an opportunity to try out **Tauri**.  

### Why Tauri?  
Tauri is a framework for building desktop applications using a Rust backend and a web-based frontend. It produces lightweight applications with a small footprint, and its structure enforces a separation between the backend and frontend.  

### Backend  
The backend is entirely written in **Rust** and divided into several modules:  
- **`data.rs`:** Handles data parsing, processing, and serialization into a binary format.  
- **`algo.rs`:** Implements the algorithm for finding the best combination of packages.  
- **`lib.rs`:** Contains shared utilities and defines the API accessed by the frontend.  

I chose not to use a dedicated database to have more control over data structures and memory layout, allowing for better optimization and performance tuning.  

### Frontend  
The frontend uses a modern web stack:  
- **Svelte + SvelteKit:** For building the UI and application logic.  
- **ShadCN UI Framework:** Provides reusable UI components.  
- **TailwindCSS:** Simplifies styling with utility-first classes.  

I didn’t have much prior experience with these tools, and they were chosen somewhat arbitrarily. That said, they worked well for this project, and I didn’t encounter significant issues during development. My frontend code is pretty nasty though.

### Summary  
The application structure, using Rust for the backend and modern web tools for the frontend, is modular and fits well within the Tauri framework. While there are trade-offs with this stack, it served the needs of the project effectively.  