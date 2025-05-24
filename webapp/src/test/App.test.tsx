import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import App from '../App'

describe('App Component', () => {
  it('renders without crashing', () => {
    render(<App />)
    expect(screen.getByText('ShExML to RML')).toBeInTheDocument()
  })

  it('contains the main transpiler component', () => {
    render(<App />)
    
    // Check if main UI elements are present
    expect(screen.getByText('Transform ShExML mappings to RML with this modern, Rust-based transpiler')).toBeInTheDocument()
    expect(screen.getByText('Convert to RML')).toBeInTheDocument()
  })

  it('has correct background styling', () => {
    const { container } = render(<App />)
    const appDiv = container.firstChild as HTMLElement
    
    expect(appDiv).toHaveClass('min-h-screen', 'bg-background')
  })
})
