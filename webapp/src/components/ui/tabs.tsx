"use client"

import React from "react"
import { cn } from "../../lib/utils"

interface TabsTriggerProps {
  className?: string
  children: React.ReactNode
  value: string
  setValue?: (value: string) => void
  active?: boolean
}

interface TabsContentProps {
  className?: string
  children: React.ReactNode
  value: string
}

interface TabsListProps {
  className?: string
  children: React.ReactNode
  setValue?: (value: string) => void
  value?: string
}

const Tabs = ({
  defaultValue,
  className,
  children,
  ...props
}: {
  defaultValue: string
  className?: string
  children: React.ReactNode
}) => {
  const [value, setValue] = React.useState(defaultValue)

  const triggers: React.ReactElement[] = []
  const contents: React.ReactElement[] = []

  React.Children.forEach(children, (child) => {
    if (React.isValidElement(child)) {
      if (child.type === TabsList) {
        const clonedTabsList = React.cloneElement(child as React.ReactElement<TabsListProps>, {
          setValue,
          value,
        })
        triggers.push(clonedTabsList)
      } else if (child.type === TabsContent) {
        const childProps = child.props as TabsContentProps
        if (childProps.value === value) {
          contents.push(child)
        }
      }
    }
  })

  return (
    <div className={cn("", className)} {...props}>
      {triggers}
      {contents}
    </div>
  )
}

const TabsList = ({
  className,
  children,
  setValue,
  value,
  ...props
}: TabsListProps) => {
  const clonedChildren = React.Children.map(children, (child) => {
    if (React.isValidElement(child) && child.type === TabsTrigger) {
      const childProps = child.props as TabsTriggerProps
      return React.cloneElement(child as React.ReactElement<TabsTriggerProps>, {
        setValue,
        active: childProps.value === value,
      })
    }
    return child
  })

  return (
    <div
      className={cn(
        "inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground",
        className,
      )}
      {...props}
    >
      {clonedChildren}
    </div>
  )
}

const TabsTrigger = ({
  className,
  children,
  value,
  setValue,
  active,
  ...props
}: {
  className?: string
  children: React.ReactNode
  value: string
  setValue?: (value: string) => void
  active?: boolean
}) => {
  return (
    <button
      className={cn(
        "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        active ? "bg-background text-foreground shadow-sm" : "hover:bg-background/50 hover:text-foreground",
        className,
      )}
      onClick={() => setValue && setValue(value)}
      {...props}
    >
      {children}
    </button>
  )
}

const TabsContent = ({
  className,
  children,
  value,
  ...props
}: {
  className?: string
  children: React.ReactNode
  value: string
}) => {
  return (
    <div
      className={cn(
        "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  )
}

export { Tabs, TabsList, TabsTrigger, TabsContent }
