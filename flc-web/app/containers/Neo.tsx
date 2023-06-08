'use client'
import React, { KeyboardEvent, useState } from 'react'

interface NeoProps {
  children?: React.ReactNode
}

const Neo = (props: NeoProps) => {
  const { children } = props
  const [prompt, setPrompt] = useState('')

  const handleKeyDown = (e: KeyboardEvent<HTMLDivElement>) => {
    setPrompt(e.code)
    console.log(prompt)
  }

  return (
    <div className="neo" onKeyDown={handleKeyDown}>
      {children}
    </div>
  )
}

export default Neo
