import { describe, it, expect, beforeEach, vi } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { Transpiler } from '../components/transpiler'

// Mock fetch
const mockFetch = vi.fn()
global.fetch = mockFetch

describe('Transpiler Component', () => {
  beforeEach(() => {
    mockFetch.mockClear()
  })
  it('renders transpiler interface correctly', () => {
    render(<Transpiler />)
    
    // Check if main elements are present
    expect(screen.getByText('ShExML to RML')).toBeInTheDocument()
    expect(screen.getByText('Transform ShExML mappings to RML with this modern, Rust-based transpiler')).toBeInTheDocument()
  })

  it('displays example buttons correctly', () => {
    render(<Transpiler />)
    
    expect(screen.getByText('Examples:')).toBeInTheDocument()
    expect(screen.getByText('Basic')).toBeInTheDocument()
    expect(screen.getByText('Advanced')).toBeInTheDocument()
  })

  it('shows tabs correctly', () => {
    render(<Transpiler />)
    
    expect(screen.getByText('Editor')).toBeInTheDocument()
    expect(screen.getByText('About')).toBeInTheDocument()
  })

  it('has convert button', () => {
    render(<Transpiler />)
    
    const convertButton = screen.getByText('Convert to RML')
    expect(convertButton).toBeInTheDocument()
  })

  it('handles API call correctly', async () => {
    const user = userEvent.setup()
    const mockRmlResponse = '@prefix rml: <http://example.com/> .'
    
    mockFetch.mockResolvedValueOnce({
      ok: true,
      text: () => Promise.resolve(mockRmlResponse)
    })

    render(<Transpiler />)
    
    const convertButton = screen.getByText('Convert to RML')
    await user.click(convertButton)

    await waitFor(() => {
      expect(mockFetch).toHaveBeenCalledTimes(1)
    })
  })
})
