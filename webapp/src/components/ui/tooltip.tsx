"use client"

import type React from "react"
import { useState } from "react"
import { cn } from "../../lib/utils"

interface TooltipProps {
  children: React.ReactNode
  content: React.ReactNode
  side?: "top" | "right" | "bottom" | "left"
  align?: "start" | "center" | "end"
}

const TooltipProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <>{children}</>
}

const Tooltip: React.FC<TooltipProps> = ({ children, content, side = "top", align = "center" }) => {
  const [isVisible, setIsVisible] = useState(false)

  return (
    <div className="relative inline-block">
      <div
        onMouseEnter={() => setIsVisible(true)}
        onMouseLeave={() => setIsVisible(false)}
        onFocus={() => setIsVisible(true)}
        onBlur={() => setIsVisible(false)}
      >
        {children}
      </div>
      {isVisible && (
        <div
          className={cn(
            "absolute z-50 px-3 py-1.5 text-xs rounded shadow-md bg-popover text-popover-foreground animate-in fade-in-0 zoom-in-95",
            side === "top" && "bottom-full mb-2",
            side === "right" && "left-full ml-2",
            side === "bottom" && "top-full mt-2",
            side === "left" && "right-full mr-2",
            align === "start" && "start-0",
            align === "center" && "left-1/2 -translate-x-1/2",
            align === "end" && "end-0",
          )}
        >
          {content}
        </div>
      )}
    </div>
  )
}

const TooltipTrigger: React.FC<{
  children: React.ReactNode
  asChild?: boolean
}> = ({ children }) => {
  return <>{children}</>
}

const TooltipContent: React.FC<{
  children: React.ReactNode
  className?: string
}> = ({ children, className }) => {
  return <div className={className}>{children}</div>
}

export { Tooltip, TooltipTrigger, TooltipContent, TooltipProvider }
