# Tests for ShExML2RML WebApp

This directory contains the test suite for the ShExML2RML web application.

## Test Structure

### `App.test.tsx`
Tests the main App component:
- ✅ Renders without crashing
- ✅ Contains main transpiler component
- ✅ Has correct styling classes

### `Transpiler.test.tsx`
Tests the core Transpiler component:
- ✅ Renders interface correctly
- ✅ Displays example buttons
- ✅ Shows tabs (Editor/About)
- ✅ Has convert button
- ✅ Handles API calls correctly

### `utils.test.ts`
Tests utility functions and configurations:
- ✅ Validates ShExML syntax examples
- ✅ Validates RML output structure
- ✅ Tests API URL configurations
- ✅ Tests file extension validation

## Running Tests

```bash
# Run tests in watch mode
npm test

# Run tests once
npm run test:run

# Run tests with UI (if available)
npm run test:ui

# Run tests with coverage (if configured)
npm run test:coverage
```

## Test Setup

- **Framework**: Vitest
- **Testing Library**: React Testing Library
- **Environment**: jsdom
- **Mocking**: Vi (built into Vitest)

## Coverage

Current test coverage includes:
- Component rendering
- User interactions
- API calls
- Error handling
- Utility functions

## Notes

- Tests use mocked fetch for API calls
- Some console warnings about React keys are expected (from the actual component)
- Tests are configured to run in jsdom environment for DOM testing
