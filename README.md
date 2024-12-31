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
Optimizations

1. Data Preprocessing and Precomputation

To eliminate parsing and conversion overhead, data is parsed from CSV files, processed, and serialized into an efficient binary format using bincode. During this step:
	•	The data is reshaped for quick querying by indexing directly into vectors based on their IDs.
	•	Relevant TypeScript types are generated for seamless integration with the frontend.

This approach minimizes runtime processing overhead and ensures fast, efficient data access in the backend.

2. Eliminating Offers aka. I ❤️ Bitmaps!

Previously, the relationship between games and packages relied on traversing a large set of offers (approximately 3200 entries). This approach had:
	•	A significant memory footprint (~224 kB).
	•	Poor query performance for common operations.

Optimization: Each game’s offers are now represented using two bitmap fields (one for “Live” and one for “Highlights”). Each bit in the bitmap corresponds to a package ID:
	•	Memory Efficiency: Reduced to ~45 kB by using compact 64-bit integers.
	•	Query Speed: Accessing all offers for a game is now as simple as reading the bitmap.
	•	Algorithm Simplification: The best combination selection algorithm is now straightforward and efficient.

Results:
Preliminary benchmarks using sample datasets show up to 327x faster performance for finding the best combination. Without this optimization, achieving acceptable response times would not have been feasible.

3. The Algorithm

The algorithm used to find the best combination of packages is fundamentally a Depth-First Search (DFS) with branch pruning. Here’s how it works:
	•	Traversal: The algorithm traverses all possible package combinations while keeping track of the best combination found so far that covers all required games.
	•	Branch Pruning: Using the “best combination” information, the algorithm skips any combinations that cannot possibly provide a better solution.
	•	Efficiency with Bitmaps: The game bitmaps allow for fast computation and comparison of game coverages between packages or combinations, significantly reducing overhead.

Micro-Optimizations:
To further optimize performance:
	•	Preallocation: All necessary memory is preallocated wherever possible.
	•	In-place Computations: Operations are performed in place to avoid allocation overhead.

The combination of these techniques ensures both speed and memory efficiency.

Source Code:
The code for this algorithm is extensively documented, with attention to detail in both implementation and comments. I strongly recommend reviewing it if you’re interested in understanding or adapting the approach. This part of the project is something I’m particularly proud of.
