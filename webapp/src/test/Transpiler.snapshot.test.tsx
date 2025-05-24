import { describe, it, expect } from 'vitest'
import { render } from '@testing-library/react'
import { Transpiler } from '../components/transpiler'

describe('Transpiler Snapshots', () => {
  it('matches snapshot for initial render', () => {
    const { container } = render(<Transpiler />)
    expect(container.firstChild).toMatchSnapshot()
  })

  it('matches snapshot for header section', () => {
    const { container } = render(<Transpiler />)
    const header = container.querySelector('.flex.justify-between.items-center.mb-8')
    expect(header).toMatchSnapshot()
  })
})
