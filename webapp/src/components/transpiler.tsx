import { useState } from "react"
import { Button } from "./ui/button"
import { Card, CardContent } from "./ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs"
import { ModeToggle } from "./mode-toggle"
import { Badge } from "./ui/badge"
import { Tooltip, TooltipProvider, TooltipTrigger } from "./ui/tooltip"
import { toast } from "sonner"

const VITE_API_URL = import.meta.env.VITE_API_URL

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

  const convertToRML = async () => {
    setIsConverting(true);
    console.log(import.meta.env.VITE_API_URL)

    try {
      if (!shexmlCode.trim()) {
        throw new Error("SheXML code is empty");
      }

      const response = await fetch(`${VITE_API_URL}/transpile`, {
        method: "POST",
        headers: {
          "Content-Type": "text/plain",
        },
        body: shexmlCode,
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Error ${response.status}: ${errorText || 'Unknown error'}`);
      }

      const converted = await response.text();
      setRmlCode(converted);

    } catch (error) {
      const errorMessage = (error instanceof Error && error.message) ? error.message : "Error converting code";
      toast.error(errorMessage);
      setRmlCode("");
    } finally {
      setIsConverting(false);
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
    toast.success("The code has been copied to your clipboard.")
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
          <TooltipProvider>
            <Tooltip content="Click me!" side="bottom">        
              <h1 onClick={showGiantFerris} className="text-4xl font-bold tracking-tight">ShExML to RML</h1>
            </Tooltip>
          </TooltipProvider>
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
