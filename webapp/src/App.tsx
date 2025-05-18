import { Transpiler } from "./components/transpiler"
import { Toaster } from "@/components/ui/sonner"

function App() {
  return (
    <div className="min-h-screen bg-background">
      <Transpiler />
      <Toaster richColors/>
    </div>
  )
}

export default App
