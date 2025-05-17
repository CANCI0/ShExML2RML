import { useState } from "react"
import { Button } from "./ui/button"
import { Card, CardContent } from "./ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs"
import { ModeToggle } from "./mode-toggle"
import { Badge } from "./ui/badge"
import { Tooltip, TooltipProvider, TooltipTrigger } from "./ui/tooltip"
import { useToast } from "../hooks/use-toast"

const ArrowRight = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    className="ml-2 h-4 w-4"
  >
    <path d="M5 12h14"></path>
    <path d="m12 5 7 7-7 7"></path>
  </svg>
)

const Copy = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    className="h-4 w-4"
  >
    <rect width="14" height="14" x="8" y="8" rx="2" ry="2"></rect>
    <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>
  </svg>
)

const RotateFerris = (width: number = 64, height: number = 64) => (
    <svg
    xmlns="http://www.w3.org/2000/svg"
    width={width}
    height={height}
    viewBox="0 0 64 64"
    className="mr-2 h-8 w-8 animate-spin"
  >
    <image
      href="./ferris.svg"
      width={width}
      height={height}
      preserveAspectRatio="xMidYMid meet"
    />
    <circle cx="50" cy="14" r="3" fill="#fff" opacity="0.6">
      <animate 
        attributeName="cy" 
        values="14;6;14" 
        dur="1.5s" 
        repeatCount="indefinite"
      />
    </circle>
  </svg>
)

const Code = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    className="h-4 w-4"
  >
    <polyline points="16 18 22 12 16 6"></polyline>
    <polyline points="8 6 2 12 8 18"></polyline>
  </svg>
)

const FileCode = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    className="h-5 w-5 text-primary"
  >
    <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
    <polyline points="14 2 14 8 20 8"></polyline>
    <path d="m10 13-2 2 2 2"></path>
    <path d="m14 17 2-2-2-2"></path>
  </svg>
)

const Info = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    className="h-5 w-5"
  >
    <circle cx="12" cy="12" r="10"></circle>
    <path d="M12 16v-4"></path>
    <path d="M12 8h.01"></path>
  </svg>
)

const showGiantFerris = () => {
  const ferris = document.createElement("img")
  ferris.src = "./ferris.svg"
  ferris.style.position = "fixed"
  ferris.style.top = "50%"
  ferris.style.left = "50%"
  ferris.style.transform = "translate(-50%, -50%)"
  ferris.style.width = "100vmin"
  ferris.style.height = "100vmin"
  ferris.style.animation = "girar 1s linear infinite"
  ferris.style.zIndex = "9999"
  document.body.appendChild(ferris)

  const style = document.createElement('style')
  style.textContent = `
    @keyframes girar {
      from { transform: translate(-50%, -50%) rotate(0deg); }
      to { transform: translate(-50%, -50%) rotate(360deg); }
    }
  `
  document.head.appendChild(style)

  setTimeout(() => {
    document.body.removeChild(ferris)
    document.head.removeChild(style)
  }, 5000)
}

const EXAMPLES = {
  basic: `PREFIX : <http://example.com/>
PREFIX schema: <http://schema.org/>
SOURCE films_xml_file <http://shexml.herminiogarcia.com/files/films.xml>

ITERATOR film_xml <xpath: //film> {
    FIELD id <@id>
    FIELD name <name>
}

EXPRESSION films <films_xml_file.film_xml>

:Film :[films.id] {
    schema:name [films.name] ;
}`,
  advanced: `PREFIX : <http://example.com/>
PREFIX dbr: <http://dbpedia.org/resource/>
PREFIX schema: <http://schema.org/>
SOURCE films_xml_file <https://shexml.herminiogarcia.com/files/films.xml>
SOURCE films_json_file <https://shexml.herminiogarcia.com/files/films.json>
ITERATOR film_xml <xpath: //film> {
    FIELD id <@id>
    FIELD name <name>
    FIELD year <year>
    FIELD country <country>
    FIELD directors <crew/directors/director>
    FIELD screenwritters <crew//screenwritter>
    FIELD music <crew/music>
    FIELD photography <crew/photography>
    ITERATOR actors <cast/actor> {
        FIELD name <name>
        FIELD role <role>
        FIELD film <../../@id>
    }
    ITERATOR actresses <cast/actress> {
        FIELD name <name>
        FIELD role <role>
        FIELD film <../../@id>
    }
}
ITERATOR film_json <jsonpath: $.films[*]> {
    FIELD id <id>
    FIELD name <name>
    FIELD year <year>
    FIELD country <country>
    FIELD directors <crew.director>
    FIELD screenwritters <crew.screenwritter>
    FIELD music <crew.music>
    FIELD photography <crew.cinematography>
    ITERATOR actors <cast[*]> {
        FIELD name <name>
        FIELD role <role>
    }
}
EXPRESSION films <films_xml_file.film_xml UNION films_json_file.film_json>

:Films :[films.id] {
    schema:name [films.name] ;
    :year dbr:[films.year] ;
    schema:countryOfOrigin dbr:[films.country] ;
    schema:director dbr:[films.directors] ;
    :screenwritter dbr:[films.screenwritters] ;
    schema:musicBy dbr:[films.music] ;
    :cinematographer dbr:[films.photography] ;
    schema:actor @:Actor ;
    schema:actor @:Actress ;
}

:Actor dbr:[films.actors.name] {
    :name [films.actors.name] ;
    :appear_on :[films.actors.film] ;
}

:Actress dbr:[films.actresses.name] {
    :name [films.actresses.name] ;
    :appear_on :[films.actresses.film] ;
}`,
}

export function Transpiler() {
  const [shexmlCode, setShexmlCode] = useState<string>(EXAMPLES.basic)
  const [rmlCode, setRmlCode] = useState<string>("")
  const [isConverting, setIsConverting] = useState(false)
  const [activeExample, setActiveExample] = useState("basic")
  const { toast } = useToast()

  const convertToRML = () => {
    setIsConverting(true)

    // Simulate processing time
    setTimeout(() => {
      // This is a placeholder for the actual conversion logic
      const converted =
        activeExample === "basic"
          ? `@prefix rml: <http://semweb.mmlab.be/ns/rml#> .
@prefix rr: <http://www.w3.org/ns/r2rml#> .
@prefix d2rq: <http://www.wiwiss.fu-berlin.de/suhl/bizer/D2RQ/0.1#> .
@prefix ql: <http://semweb.mmlab.be/ns/ql#> .
@prefix map: <http://mapping.example.com/> .
@prefix : <http://example.com/> .
@prefix schema: <http://schema.org/> .

map:m_1  a rr:TriplesMap ;
    rml:logicalSource      map:ls_1 ;
    rr:predicateObjectMap  map:po_1 ;
    rr:subjectMap          map:s_1 .

map:ls_1  a                rml:LogicalSource ;
        rml:iterator              "//film" ;
        rml:referenceFormulation  ql:XPath ;
        rml:source                "http://shexml.herminiogarcia.com/files/films.xml" .

map:s_1  a           rr:SubjectMap ;
        rr:template  "http://example.com/{@id}" .

map:po_1  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_1 ;
        rr:predicateMap  map:p_1 .

map:o_1  a          rr:ObjectMap ;
        rr:template  "{name}" ;
        rr:termType  rr:Literal .

map:p_1  a           rr:predicateMap ;
        rr:constant  schema:name .
`
          : `@prefix rml: <http://semweb.mmlab.be/ns/rml#> .
@prefix rr: <http://www.w3.org/ns/r2rml#> .
@prefix d2rq: <http://www.wiwiss.fu-berlin.de/suhl/bizer/D2RQ/0.1#> .
@prefix ql: <http://semweb.mmlab.be/ns/ql#> .
@prefix map: <http://mapping.example.com/> .
@prefix : <http://example.com/> .
@prefix dbr: <http://dbpedia.org/resource/> .
@prefix schema: <http://schema.org/> .

map:m_1  a rr:TriplesMap ;
    rml:logicalSource      map:ls_1 ;
    rr:predicateObjectMap  map:po_1 , map:po_2 , map:po_3 , map:po_4 , map:po_5 , map:po_6 , map:po_7 , map:po_8 , map:po_9 ;
    rr:subjectMap          map:s_1 .

map:ls_1  a                rml:LogicalSource ;
        rml:iterator              "//film" ;
        rml:referenceFormulation  ql:XPath ;
        rml:source                "https://shexml.herminiogarcia.com/files/films.xml" .

map:s_1  a           rr:SubjectMap ;
        rr:template  "http://example.com/{@id}" .

map:po_1  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_1 ;
        rr:predicateMap  map:p_1 .

map:o_1  a          rr:ObjectMap ;
        rr:template  "{name}" ;
        rr:termType  rr:Literal .

map:p_1  a           rr:predicateMap ;
        rr:constant  schema:name .
map:po_2  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_2 ;
        rr:predicateMap  map:p_2 .

map:o_2  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{year}" ;
        rr:termType  rr:IRI .

map:p_2  a           rr:predicateMap ;
        rr:constant  :year .
map:po_3  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_3 ;
        rr:predicateMap  map:p_3 .

map:o_3  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{country}" ;
        rr:termType  rr:IRI .

map:p_3  a           rr:predicateMap ;
        rr:constant  schema:countryOfOrigin .
map:po_4  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_4 ;
        rr:predicateMap  map:p_4 .

map:o_4  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{crew/directors/director}" ;
        rr:termType  rr:IRI .

map:p_4  a           rr:predicateMap ;
        rr:constant  schema:director .
map:po_5  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_5 ;
        rr:predicateMap  map:p_5 .

map:o_5  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{crew//screenwritter}" ;
        rr:termType  rr:IRI .

map:p_5  a           rr:predicateMap ;
        rr:constant  :screenwritter .
map:po_6  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_6 ;
        rr:predicateMap  map:p_6 .

map:o_6  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{crew/music}" ;
        rr:termType  rr:IRI .

map:p_6  a           rr:predicateMap ;
        rr:constant  schema:musicBy .
map:po_7  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_7 ;
        rr:predicateMap  map:p_7 .

map:o_7  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{crew/photography}" ;
        rr:termType  rr:IRI .

map:p_7  a           rr:predicateMap ;
        rr:constant  :cinematographer .
map:po_8  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_8 ;
        rr:predicateMap  map:p_8 .

map:o_8  a          rr:ObjectMap .

map:p_8  a           rr:predicateMap ;
        rr:constant  schema:actor .
map:po_9  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_9 ;
        rr:predicateMap  map:p_9 .

map:o_9  a          rr:ObjectMap .
        rr:parentTriplesMap  map:m_2 .

map:p_9  a           rr:predicateMap ;
        rr:constant  schema:actor .


map:m_2  a rr:TriplesMap ;
    rml:logicalSource      map:ls_4 ;
    rr:predicateObjectMap  map:po_10 , map:po_11 , map:po_12 , map:po_13 , map:po_14 , map:po_15 , map:po_16 , map:po_17 , map:po_18 ;
    rr:subjectMap          map:s_2 .

map:ls_4  a                rml:LogicalSource ;
        rml:iterator              "$.films[*]" ;
        rml:referenceFormulation  ql:JSONPath ;
        rml:source                "https://shexml.herminiogarcia.com/files/films.json" .

map:s_2  a           rr:SubjectMap ;
        rr:template  "http://example.com/{id}" .

map:po_10  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_10 ;
        rr:predicateMap  map:p_10 .

map:o_10  a          rr:ObjectMap ;
        rr:template  "{name}" ;
        rr:termType  rr:Literal .

map:p_10  a           rr:predicateMap ;
        rr:constant  schema:name .
map:po_11  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_11 ;
        rr:predicateMap  map:p_11 .

map:o_11  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{year}" ;
        rr:termType  rr:IRI .

map:p_11  a           rr:predicateMap ;
        rr:constant  :year .
map:po_12  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_12 ;
        rr:predicateMap  map:p_12 .

map:o_12  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{country}" ;
        rr:termType  rr:IRI .

map:p_12  a           rr:predicateMap ;
        rr:constant  schema:countryOfOrigin .
map:po_13  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_13 ;
        rr:predicateMap  map:p_13 .

map:o_13  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{crew.director}" ;
        rr:termType  rr:IRI .

map:p_13  a           rr:predicateMap ;
        rr:constant  schema:director .
map:po_14  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_14 ;
        rr:predicateMap  map:p_14 .

map:o_14  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{crew.screenwritter}" ;
        rr:termType  rr:IRI .

map:p_14  a           rr:predicateMap ;
        rr:constant  :screenwritter .
map:po_15  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_15 ;
        rr:predicateMap  map:p_15 .

map:o_15  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{crew.music}" ;
        rr:termType  rr:IRI .

map:p_15  a           rr:predicateMap ;
        rr:constant  schema:musicBy .
map:po_16  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_16 ;
        rr:predicateMap  map:p_16 .

map:o_16  a          rr:ObjectMap ;
        rr:template  "http://dbpedia.org/resource/{crew.cinematography}" ;
        rr:termType  rr:IRI .

map:p_16  a           rr:predicateMap ;
        rr:constant  :cinematographer .
map:po_17  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_17 ;
        rr:predicateMap  map:p_17 .

map:o_17  a          rr:ObjectMap .

map:p_17  a           rr:predicateMap ;
        rr:constant  schema:actor .
map:po_18  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_18 ;
        rr:predicateMap  map:p_18 .

map:o_18  a          rr:ObjectMap .

map:p_18  a           rr:predicateMap ;
        rr:constant  schema:actor .


map:m_3  a rr:TriplesMap ;
    rml:logicalSource      map:ls_5 ;
    rr:predicateObjectMap  map:po_19 , map:po_20 ;
    rr:subjectMap          map:s_3 .

map:ls_5  a                rml:LogicalSource ;
        rml:iterator              "//film/cast/actor" ;
        rml:referenceFormulation  ql:JSONPath ;
        rml:source                "https://shexml.herminiogarcia.com/files/films.xml" .

map:s_3  a           rr:SubjectMap ;
        rr:template  "http://dbpedia.org/resource/{name}" .

map:po_19  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_19 ;
        rr:predicateMap  map:p_19 .

map:o_19  a          rr:ObjectMap ;
        rr:template  "{name}" ;
        rr:termType  rr:Literal .

map:p_19  a           rr:predicateMap ;
        rr:constant  :name .
map:po_20  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_20 ;
        rr:predicateMap  map:p_20 .

map:o_20  a          rr:ObjectMap ;
        rr:template  "http://example.com/{../../@id}" ;
        rr:termType  rr:IRI .

map:p_20  a           rr:predicateMap ;
        rr:constant  :appear_on .


map:m_4  a rr:TriplesMap ;
    rml:logicalSource      map:ls_5 ;
    rr:predicateObjectMap  map:po_21 ;
    rr:subjectMap          map:s_4 .

map:ls_5  a                rml:LogicalSource ;
        rml:iterator              "$.films[*].cast[*]" ;
        rml:referenceFormulation  ql:JSONPath ;
        rml:source                "https://shexml.herminiogarcia.com/files/films.json" .

map:s_4  a           rr:SubjectMap ;
        rr:template  "http://dbpedia.org/resource/{name}" .

map:po_21  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_21 ;
        rr:predicateMap  map:p_21 .

map:o_21  a          rr:ObjectMap ;
        rr:template  "{name}" ;
        rr:termType  rr:Literal .

map:p_21  a           rr:predicateMap ;
        rr:constant  :name .


map:m_5  a rr:TriplesMap ;
    rml:logicalSource      map:ls_3 ;
    rr:predicateObjectMap  map:po_22 , map:po_23 ;
    rr:subjectMap          map:s_5 .

map:ls_3  a                rml:LogicalSource ;
        rml:iterator              "//film/cast/actress" ;
        rml:referenceFormulation  ql:XPath ;
        rml:source                "https://shexml.herminiogarcia.com/files/films.xml" .

map:s_5  a           rr:SubjectMap ;
        rr:template  "http://dbpedia.org/resource/{name}" .

map:po_22  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_22 ;
        rr:predicateMap  map:p_22 .

map:o_22  a          rr:ObjectMap ;
        rr:template  "{name}" ;
        rr:termType  rr:Literal .

map:p_22  a           rr:predicateMap ;
        rr:constant  :name .
map:po_23  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_23 ;
        rr:predicateMap  map:p_23 .

map:o_23  a          rr:ObjectMap ;
        rr:template  "http://example.com/{../../@id}" ;
        rr:termType  rr:IRI .

map:p_23  a           rr:predicateMap ;
        rr:constant  :appear_on .
`

      setRmlCode(converted)
      setIsConverting(false)
    }, 800)
  }

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
    toast({
      title: "Copied to clipboard",
      description: "The code has been copied to your clipboard.",
      duration: 2000,
    })
  }

  const clearAll = () => {
    setShexmlCode("")
    setRmlCode("")
  }

  const handleExampleChange = (value: string) => {
    setActiveExample(value)
    setShexmlCode(EXAMPLES[value as keyof typeof EXAMPLES])
    setRmlCode("")
  }

  return (
    <div className="container mx-auto px-4 py-8 h-dvh">
      <div className="flex justify-between items-center mb-8">
        <div>
          <h1 onClick={showGiantFerris} className="text-4xl font-bold tracking-tight">ShExML to RML</h1>
          <p className="text-muted-foreground mt-2 leading-4.5">Transform ShExML mappings to RML with this modern, Rust-based transpiler</p>
        </div>
        <div className="flex items-center gap-4">
          <TooltipProvider>
            <Tooltip content="ShExML Documentation">
              <TooltipTrigger asChild>
                <a
                  href="https://shexml.herminiogarcia.com/spec/"
                  title="ShExML Documentation"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-muted-foreground hover:text-foreground transition-colors"
                >
                  <Info />
                </a>
              </TooltipTrigger>
            </Tooltip>
          </TooltipProvider>
          <ModeToggle />
        </div>
      </div>

      <Tabs defaultValue="editor" className="space-y-6">
        <TabsList className="grid w-full max-w-md grid-cols-2">
          <TabsTrigger value="editor" className="flex items-center gap-2">
            <Code />
            Editor
          </TabsTrigger>
          <TabsTrigger value="about" className="flex items-center gap-2">
            <Info />
            About
          </TabsTrigger>
        </TabsList>

        <TabsContent value="editor" className="space-y-6 h-full">
          <div className="flex flex-wrap gap-4 mb-4">
            <Badge variant="outline" className="text-sm">
              Examples:
            </Badge>
            <Button
              variant={activeExample === "basic" ? "secondary" : "ghost"}
              size="sm"
              onClick={() => handleExampleChange("basic")}
            >
              Basic
            </Button>
            <Button
              variant={activeExample === "advanced" ? "secondary" : "ghost"}
              size="sm"
              onClick={() => handleExampleChange("advanced")}
            >
              Advanced
            </Button>
          </div>

          <div className="grid lg:grid-cols-2 gap-6">
            <Card className="overflow-hidden border bg-card">
              <div className="flex items-center justify-between p-4 border-b">
                <div className="flex items-center gap-2">
                  <FileCode />
                  <h2 className="font-medium">ShExML Input</h2>
                </div>
                <Button variant="ghost" size="icon" onClick={() => copyToClipboard(shexmlCode)} disabled={!shexmlCode}>
                  <Copy />
                  <span className="sr-only">Copy ShExML code</span>
                </Button>
              </div>
              <CardContent className="p-0">
                <textarea
                  className="w-full h-[400px] p-4 font-mono text-sm bg-background/50 focus:outline-none resize-none"
                  aria-label="ShExML Code"
                  value={shexmlCode}
                  onChange={(e) => setShexmlCode(e.target.value)}
                  placeholder="Enter your ShExML code here..."
                  spellCheck="false"
                />
              </CardContent>
            </Card>

            <Card className="overflow-hidden border bg-card">
              <div className="flex items-center justify-between p-4 border-b">
                <div className="flex items-center gap-2">
                  <FileCode />
                  <h2 className="font-medium">RML Output</h2>
                </div>
                <Button variant="ghost" size="icon" onClick={() => copyToClipboard(rmlCode)} disabled={!rmlCode}>
                  <Copy />
                  <span className="sr-only">Copy RML code</span>
                </Button>
              </div>
              <CardContent className="p-0">
                <textarea
                  className="w-full h-[400px] p-4 font-mono text-sm bg-background/50 focus:outline-none resize-none"
                  aria-label="RML Code"
                  value={rmlCode}
                  readOnly
                  placeholder="RML code will appear here after conversion..."
                  spellCheck="false"
                />
              </CardContent>
            </Card>
          </div>

          <div className="flex flex-wrap justify-center gap-4 mt-6">
            <Button onClick={convertToRML} disabled={!shexmlCode || isConverting} className="px-6" size="lg">
              {isConverting ? (
                <>
                  {RotateFerris(64, 64)}
                  Converting...
                </>
              ) : (
                <>
                  Convert to RML
                  <ArrowRight />
                </>
              )}
            </Button>
            <Button variant="outline" onClick={clearAll} size="lg">
              Clear All
            </Button>
          </div>
        </TabsContent>

        <TabsContent value="about">
          <div className="grid md:grid-cols-2 gap-8">
            <Card>
              <CardContent className="pt-6">
                <h3 className="text-xl font-semibold mb-4 flex items-center gap-2">
                  <span className="bg-primary/10 p-2 rounded-md">
                    <Code />
                  </span>
                  ShExML
                </h3>
                <p className="text-muted-foreground">
                  ShExML (Shape Expressions Mapping Language) is a language designed to map and transform heterogeneous
                  data sources into RDF. It provides a concise syntax for defining mappings between different data
                  formats and RDF, with a focus on readability and maintainability.
                </p>
                <div className="mt-4">
                  <a
                    href="https://shexml.herminiogarcia.com/"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-primary hover:underline inline-flex items-center gap-1"
                  >
                    Learn more about ShExML
                    <ArrowRight />
                  </a>
                </div>
              </CardContent>
            </Card>
            <Card>
              <CardContent className="pt-6">
                <h3 className="text-xl font-semibold mb-4 flex items-center gap-2">
                  <span className="bg-primary/10 p-2 rounded-md">
                    <Code />
                  </span>
                  RML
                </h3>
                <p className="text-muted-foreground">
                  RML (RDF Mapping Language) is a mapping language defined to express customized mapping rules from
                  heterogeneous data structures and serializations to the RDF data model. It is an extension of R2RML,
                  the W3C standard for mapping relational databases to RDF.
                </p>
                <div className="mt-4">
                  <a
                    href="https://rml.io/"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-primary hover:underline inline-flex items-center gap-1"
                  >
                    Learn more about RML
                    <ArrowRight />
                  </a>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>

      <footer className="mt-16 pt-8 border-t text-center text-muted-foreground text-sm">
        <p className="leading-4.5">
          This transpiler is for demonstration purposes. For production use, please refer to official ShExML and RML
          documentation.
        </p>
        <p className="mt-2 text-primary leading-4.5">
          © {new Date().getFullYear()}, Developed by <a href="https://github.com/canci0" target="_blank" rel="noopener noreferrer">Martín Cancio Barrera</a>
        </p>
      </footer>
    </div>
  )
}
