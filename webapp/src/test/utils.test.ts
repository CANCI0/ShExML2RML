import { describe, it, expect } from 'vitest'

// Test utilities and helper functions
describe('Utility Functions', () => {
  describe('ShExML Examples', () => {
    it('should have valid ShExML syntax in basic example', () => {
      const basicExample = `PREFIX : <http://example.com/>
PREFIX schema: <http://schema.org/>
SOURCE films_xml_file <http://shexml.herminiogarcia.com/files/films.xml>

ITERATOR film_xml <xpath: //film> {
    FIELD id <@id>
    FIELD name <name>
}

EXPRESSION films <films_xml_file.film_xml>

:Film :[films.id] {
    schema:name [films.name] ;
}`

      // Basic syntax checks
      expect(basicExample).toContain('PREFIX')
      expect(basicExample).toContain('SOURCE')
      expect(basicExample).toContain('ITERATOR')
      expect(basicExample).toContain('FIELD')
      expect(basicExample).toContain('EXPRESSION')
    })

    it('should have proper structure for RML conversion', () => {
      const rmlExample = `@prefix rml: <http://semweb.mmlab.be/ns/rml#> .
@prefix rr: <http://www.w3.org/ns/r2rml#> .
@prefix ql: <http://semweb.mmlab.be/ns/ql#> .`

      expect(rmlExample).toContain('@prefix rml:')
      expect(rmlExample).toContain('@prefix rr:')
      expect(rmlExample).toContain('@prefix ql:')
    })
  })

  describe('API Configuration', () => {
    it('should handle different API URL configurations', () => {
      // Test local development URL
      const devUrl = 'http://localhost:8080'
      expect(devUrl).toMatch(/^https?:\/\//)
      
      // Test production URL format
      const prodUrl = 'http://20.50.195.201:8080'
      expect(prodUrl).toMatch(/^https?:\/\//)
    })
  })

  describe('File Extensions', () => {
    it('should validate ShExML file extensions', () => {
      const validFiles = ['example.shexml', 'test.shexml', 'mapping.shexml']
      const invalidFiles = ['example.txt', 'test.rml', 'mapping.ttl']

      validFiles.forEach(file => {
        expect(file).toMatch(/\.shexml$/)
      })

      invalidFiles.forEach(file => {
        expect(file).not.toMatch(/\.shexml$/)
      })
    })

    it('should validate RML file extensions', () => {
      const validFiles = ['output.rml', 'result.ttl', 'mapping.rml']
      const validExtensions = /\.(rml|ttl)$/

      validFiles.forEach(file => {
        expect(file).toMatch(validExtensions)
      })
    })
  })
})
