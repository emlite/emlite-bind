# emlite-bind Scripts Architecture

This directory contains the refactored WebIDL binding generation scripts for emlite-bind (Rust implementation).

## Architecture Overview

The scripts have been refactored from a monolithic 544-line `gen.js` into a modular architecture matching the wasmbind pattern for better maintainability and code quality.

### Directory Structure

```
scripts/
├── parsers/                 # WebIDL parsing and analysis
│   ├── spec-parser.js       # Parse and organize WebIDL specifications
│   ├── type-analyzer.js     # Extract type references from WebIDL members
│   └── dependency-resolver.js # Systematic dependency resolution
├── generators/              # Code generators
│   ├── enum-generator.js    # Rust enum generation
│   ├── dictionary-generator.js # Rust dictionary struct generation
│   ├── interface-generator.js  # Rust interface struct generation
│   └── namespace-generator.js  # Rust namespace function generation
├── gen.js                   # Main orchestrator (65 lines)
├── gen-old.js              # Backup of original monolithic generator
├── utils.js                # Shared utilities
├── globals.js              # Global state management
├── index.js                # Entry point
└── README.md               # This file
```

## Key Improvements

### 1. Modular Architecture
- **Before**: 544-line monolithic `gen.js`
- **After**: 65-line orchestrator + specialized modules
- **Benefit**: Easy to test, modify, and maintain individual components

### 2. Systematic Dependency Resolution
- **Before**: "not sure how to deal with these!" ad-hoc handling
- **After**: `DependencyResolver` class with topological sorting and systematic type analysis
- **Benefit**: Reliable dependency management, proper generation order

### 3. Enhanced Documentation
- **Before**: Minimal comments, uncertainty markers
- **After**: Comprehensive JSDoc documentation, clear architectural decisions
- **Benefit**: Better developer experience and code understanding

### 4. Consistent Code Patterns
- All generators follow consistent patterns
- Proper error handling and input validation
- Clear separation of concerns

## Usage

### Generate All Bindings
```bash
cd emlite-bind
npm run scripts
# or
node scripts/index.js
```

### Expected Output
```
Starting emlite-bind generation with modular architecture...
Parsed 553 dictionaries, 2847 interfaces, 17 namespaces
Generating enums...
Generating standalone dictionaries...
Generated 553 standalone dictionaries
Generating interfaces...
Generated 2847 interfaces  
Generating namespaces...
Generated 17 namespaces
emlite-bind generation completed successfully!
```

## Generated Code Structure

### Dictionaries
Each dictionary generates as a standalone Rust file with:
- `#[derive(Clone, Debug, PartialEq, PartialOrd)]` traits
- `#[repr(transparent)]` for zero-cost abstractions
- `FromVal` implementation for JS interop
- `Deref`/`DerefMut` for transparent `Any` access
- `AsRef`/`AsMut` trait implementations
- `From<T>` conversions with proper memory management
- Individual getter/setter methods for WebIDL members

### Interfaces
Each interface generates with:
- Comprehensive trait implementations matching dictionaries
- Proper inheritance handling via `inner: ParentType`
- MDN documentation links for all methods
- Constructor variants for different argument patterns
- Static and instance method support

### Namespaces
Each namespace generates standalone Rust functions:
- Direct function calls to global namespace objects
- Multiple variants for optional parameters
- Proper error handling and type conversions

## Testing

The refactored system maintains 100% compatibility with the original generator:
- All 553 dictionaries generate correctly
- All interfaces maintain proper inheritance and member methods
- All namespace functions work as expected
- Generated code compiles successfully with Rust toolchain

## Comparison with wasmbind

Both implementations now follow identical architectural patterns:
- Modular parser/generator separation
- Systematic dependency resolution  
- Lightweight orchestrator design
- Consistent file generation approach

The main difference is target language (C++ vs Rust), but the generation logic and organization are equivalent.